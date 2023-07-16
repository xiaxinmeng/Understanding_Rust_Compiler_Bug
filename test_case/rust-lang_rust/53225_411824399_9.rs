ignore (cannot-test-this-because-xxxx)", if the annotation cannot be avoided.
[00:04:07] 
[00:04:08] some tidy checks failed
[00:04:08] 
[00:04:08] 
[00:04:08] 
[00:04:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:08] 
[00:04:08] 
[00:04:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:08] Build completed unsuccessfully in 0:00:54
[00:04:08] Build completed unsuccessfully in 0:00:54
[00:04:08] make: *** [tidy] Error 1
[00:04:08] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1313761c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:015cb9e8
$ sudo tail -n 500 /var/log/syslog
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] kvm-clock: using sched offset of 1750573672 cycles
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] Zone ranges:
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000]   Device   empty
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] Movable zone start for each node
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] Early memory node ranges
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] Policy zone: Normal
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] console [ttyS0] enabled
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.709737] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.714667] pid_max: default: 32768 minimum: 301
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.717622] ACPI: Core revision 20150930
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.725868] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.731656] Security Framework initialized
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.734206] Yama: becoming mindful.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.736188] AppArmor: AppArmor disabled by boot time parameter
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.742213] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.755021] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.763633] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.768517] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.773058] Initializing cgroup subsys io
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.775861] Initializing cgroup subsys memory
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.779582] Initializing cgroup subsys devices
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.782829] Initializing cgroup subsys freezer
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.785937] Initializing cgroup subsys net_cls
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.789044] Initializing cgroup subsys perf_event
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.792130] Initializing cgroup subsys net_prio
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.794731] Initializing cgroup subsys hugetlb
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.796792] Initializing cgroup subsys pids
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.798980] CPU: Physical Processor ID: 0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.801221] CPU: Processor Core ID: 0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.804349] mce: CPU supports 32 MCE banks
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.807260] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.811019] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.817452] Freeing SMP alternatives memory: 32K
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.829699] ftrace: allocating 32185 entries in 126 pages
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.888379] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.893139] smpboot: Max logical packages: 2
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.896846] x2apic enabled
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.900029] Switched APIC routing to physical x2apic.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    0.906536] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.015170] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.021696] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.028799] x86: Booting SMP configuration:
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.031726] .... node  #0, CPUs:      #1
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.034063] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.043252]  #2
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.044928] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.052824]  #3
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.054670] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.062670] x86: Booted up 1 node, 4 CPUs
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.065367] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.071906] devtmpfs: initialized
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.078043] evm: security.selinux
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.081637] evm: security.SMACK64
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.084417] evm: security.SMACK64EXEC
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.086536] evm: security.SMACK64TRANSMUTE
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.089359] evm: security.SMACK64MMAP
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.092505] evm: security.ima
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.094006] evm: security.capability
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.096169] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.102426] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.106486] pinctrl core: initialized pinctrl subsystem
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.109657] RTC time: 16:36:22, date: 08/09/18
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.114200] NET: Registered protocol family 16
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.131184] cpuidle: using governor ladder
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.143153] cpuidle: using governor menu
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.145328] PCCT header not found.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.147489] ACPI: bus type PCI registered
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.149816] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.153407] PCI: Using configuration type 1 for base access
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.172726] ACPI: Added _OSI(Module Device)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.175669] ACPI: Added _OSI(Processor Device)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.177976] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.180720] ACPI: Added _OSI(Processor Aggregator Device)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.186510] ACPI: Executed 2 blocks of module-level executable AML code
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.214416] ACPI: Interpreter enabled
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.217053] ACPI: (supports S0 S3 S4 S5)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.220109] ACPI: Using IOAPIC for interrupt routing
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.222544] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.256407] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.261271] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.266638] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.272036] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.279783] PCI host bridge to bus 0000:00
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.282011] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.287119] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.291943] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.297796] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.301586] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.306174] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.306644] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.335773] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.367920] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.373657] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.386421] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.395165] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.417856] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.427818] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.438839] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.466631] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.473311] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.480833] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.486630] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.492163] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.515916] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.519105] vgaarb: loaded
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.520658] SCSI subsystem initialized
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.523570] libata version 3.00 loaded.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.523594] ACPI: bus type USB registered
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.526590] usbcore: registered new interface driver usbfs
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.530116] usbcore: registered new interface driver hub
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.537176] usbcore: registered new device driver usb
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.540760] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.545113] dmi: Firmware registration failed.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.549328] PCI: Using ACPI for IRQ routing
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.552797] PCI: pci_cache_line_size set to 64 bytes
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.552900] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.552902] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.553030] NetLabel: Initializing
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.556182] NetLabel:  domain hash size = 128
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.559579] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.563215] NetLabel:  unlabeled traffic allowed by default
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.567642] amd_nb: Cannot enumerate AMD northbridges
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.571224] clocksource: Switched to clocksource kvm-clock
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.581007] pnp: PnP ACPI init
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.582968] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.583045] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.583092] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.583145] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.583190] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.583251] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.583336] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.583517] pnp: PnP ACPI: found 7 devices
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.593792] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.599188] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.599190] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.599192] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.599193] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.599234] NET: Registered protocol family 2
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.602189] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.607213] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.611260] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.615364] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.618975] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.623189] NET: Registered protocol family 1
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.625768] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.629518] PCI: CLS 0 bytes, default 64
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    1.629594] Unpacking initramfs...
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.694531] Freeing initrd memory: 21432K
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.696047] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.698111] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.701545] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.705328] hw unit of domain pp0-core 2^-0 Joules
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.707039] hw unit of domain package 2^-0 Joules
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.708648] hw unit of domain dram 2^-16 Joules
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.710117] Scanning for low memory corruption every 60 seconds
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.713195] audit: initializing netlink subsys (disabled)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.715180] audit: type=2000 audit(1533832584.583:1): initialized
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.717445] Initialise system trusted keyring
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.719074] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.722209] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.725989] zbud: loaded
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.727117] VFS: Disk quotas dquot_6.6.0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.728595] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.731559] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.734085] fuse init (API version 7.23)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.735844] Key type big_key registered
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.737274] Allocating IMA MOK and blacklist keyrings.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.743754] Key type asymmetric registered
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.745591] Asymmetric key parser 'x509' registered
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.747893] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.751694] io scheduler noop registered
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.753351] io scheduler deadline registered (default)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.755444] io scheduler cfq registered
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.757482] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.760311] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.762782] intel_idle: does not run on family 6 model 63
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.762881] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.766189] ACPI: Power Button [PWRF]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.767665] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.770528] ACPI: Sleep Button [SLPF]
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.772850] GHES: HEST is not enabled!
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.777362] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.780262] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.790054] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.792942] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.804272] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.829922] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.856648] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.881817] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.906993] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.913974] Linux agpgart interface v0.103
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.921930] loop: module loaded
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.924542] libphy: Fixed MDIO Bus: probed
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.927842] tun: Universal TUN/TAP device driver, 1.6
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.930884] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.984701] PPP generic driver version 2.4.2
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.987749] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.994769] ehci-pci: EHCI PCI platform driver
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    3.998015] ehci-platform: EHCI generic platform driver
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.001635] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.005219] ohci-pci: OHCI PCI platform driver
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.007892] ohci-platform: OHCI generic platform driver
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.010816] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.014593] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.019826] i8042: Warning: Keylock active
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.023257] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.025957] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.029658] mousedev: PS/2 mouse device common for all mice
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.033407] rtc_cmos 00:00: RTC can wake from S4
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.036443] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.040257] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.045236] i2c /dev entries driver
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.047991] device-mapper: uevent: version 1.0.3
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.051076] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.057562] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.062633] NET: Registered protocol family 10
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.065749] NET: Registered protocol family 17
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.068704] Key type dns_resolver registered
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.071804] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.076431] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.080213] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.083452] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.086524] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.093061] registered taskstats version 1
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.095731] Loading compiled-in X.509 certificates
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.099489] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.105183] zswap: loaded using pool lzo/zbud
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.110932] Key type trusted registered
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.120114] Key type encrypted registered
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.122627] ima: No TPM chip found, activating TPM-bypass!
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.127166] evm: HMAC attrs: 0x1
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.129491]   Magic number: 14:964:640
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.131750] rtc_cmos 00:00: setting system clock to 2018-08-09 16:36:25 UTC (1533832585)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.136264] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.139864] EDD information not available.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.142481] PM: Hibernation image not present or could not be loaded.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.143993] Freeing unused kernel memory: 1496K
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.147404] Write protecting the kernel read-only data: 14336k
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.151452] Freeing unused kernel memory: 1956K
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.154601] Freeing unused kernel memory: 92K
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.174636] systemd-udevd[119]: starting version 204
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.237513] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.247271] scsi host0: Virtio SCSI HBA
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.259711] AVX2 version of gcm_enc/dec engaged.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.262566] AES CTR mode by8 optimization enabled
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.265350] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.351736] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.351938] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.351940] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.353073] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.353076] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.353197] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.356926]  sda: sda1
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.359181] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.707400] tsc: Refined TSC clocksource calibration: 2299.999 MHz
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    4.711801] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2127345424d, max_idle_ns: 440795318347 ns
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    5.089468] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    7.227544] floppy0: no floppy controllers found
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    8.391340] raid6: sse2x1   gen()  8681 MB/s
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    8.459307] raid6: sse2x1   xor()  6485 MB/s
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    8.527313] raid6: sse2x2   gen() 10510 MB/s
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    8.595311] raid6: sse2x2   xor()  6986 MB/s
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    8.663309] raid6: sse2x4   gen() 12384 MB/s
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    8.731302] raid6: sse2x4   xor()  8674 MB/s
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    8.799311] raid6: avx2x1   gen() 16781 MB/s
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    8.867306] raid6: avx2x2   gen() 19485 MB/s
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    8.935319] raid6: avx2x4   gen() 22192 MB/s
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    8.939162] raid6: using algorithm avx2x4 gen() 22192 MB/s
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    8.942908] raid6: using avx2x2 recovery algorithm
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    8.948142] xor: automatically using best checksumming function:
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    8.991237]    avx       : 26705.000 MB/sec
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    9.009255] Btrfs loaded
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    9.072670] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    9.076499] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    9.167825] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    9.176324] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    9.178199] EXT4-fs (sda1): recovery complete
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    9.185204] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    9.398993] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    9.520545] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    9.567778] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [    9.759997] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   10.313180] random: cloud-init: uninitialized urandom read (32 bytes read, 44 bits of entropy available)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   10.455607] systemd-udevd[702]: starting version 204
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   10.579957] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   10.631898] intel_rapl: no valid rapl domains found in package 0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   10.693006] ppdev: user-space parallel port driver
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   10.798510] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   10.848562] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   10.905117] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   11.070069] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   11.381874] random: mktemp: uninitialized urandom read (12 bytes read, 60 bits of entropy available)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   11.451939] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   11.522301] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   11.564067] EXT4-fs (sda1): resized filesystem to 7864064
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   12.018563] init: failsafe main process (1093) killed by TERM signal
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO Running set_multiqueue.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO Set channels for eth0 to 4.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  9 16:36:33 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Starting Google Accounts daemon.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   12.766276] random: nonblocking pool is initialized
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Creating a new user account for me.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Created user account me.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Creating a new user account for henrik.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Created user account henrik.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Creating a new user account for emma.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Created user account emma.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Creating a new user account for igor.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Created user account igor.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Created user account konstantinhaase.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Creating a new user account for aj.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 cron[1442]: (CRON) INFO (pidfile fd = 3)
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Created user account aj.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 cron[1481]: (CRON) STARTUP (fork ok)
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 cron[1481]: (CRON) INFO (Running @reboot jobs)
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Creating a new user account for solarce.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 acpid: starting up with netlink and the input layer
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 acpid: 1 rule loaded
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 acpid: waiting for events: event logging is off
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Created user account solarce.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Creating a new user account for asari.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 haveged: haveged starting up
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Created user account asari.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Creating a new user account for bogdana.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   13.280821] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   13.292222] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Created user account bogdana.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Creating a new user account for konstantin.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Created user account konstantin.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Creating a new user account for carmen.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Created user account carmen.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Creating a new user account for maria.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   13.480235] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   13.485576] Bridge firewalling registered
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Created user account maria.
Aug  9 16:36:34 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   13.499805] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 16:36:35 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Removing user packer.
Aug  9 16:36:35 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 google-accounts: INFO Removing user packer.
Aug  9 16:36:35 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   13.584939] Initializing XFRM netlink socket
Aug  9 16:36:35 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   13.592012] Netfilter messages via NETLINK v0.30.
Aug  9 16:36:35 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   13.595045] ctnetlink v0.93: registering with nfnetlink.
Aug  9 16:36:35 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   13.643337] floppy0: no floppy controllers found
Aug  9 16:36:35 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   13.643494] work still pending
Aug  9 16:36:57 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpdate[1849]: adjust time server 169.254.169.254 offset 0.004801 sec
Aug  9 16:37:04 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpd[1884]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 16:37:04 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpd[1885]: proto: precision = 0.107 usec
Aug  9 16:37:04 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpd[1885]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 16:37:04 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpd[1885]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 16:37:04 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpd[1885]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 16:37:04 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpd[1885]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 16:37:04 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpd[1885]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 16:37:04 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpd[1885]: Listen normally on 3 eth0 10.20.1.50 UDP 123
Aug  9 16:37:04 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpd[1885]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 16:37:04 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpd[1885]: peers refreshed
Aug  9 16:37:04 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpd[1885]: Listening on routing socket on fd #21 for interface updates
Aug  9 16:37:05 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [   43.499579] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 16:37:05 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 startup-script: INFO Found startup-script in metadata.
Aug  9 16:37:05 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  9 16:37:05 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 startup-script: INFO startup-script: job 1 at Thu Aug  9 19:47:00 2018
Aug  9 16:37:05 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 startup-script: INFO startup-script: Return code 0.
Aug  9 16:37:05 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 startup-script: INFO startup-script: Return code 0.
Aug  9 16:37:05 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 startup-script: INFO Finished running startup scripts.
Aug  9 16:37:05 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ec2: 
Aug  9 16:37:05 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ec2: #############################################################
Aug  9 16:37:05 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 16:37:05 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ec2: 1024 f0:a6:6e:dc:fb:6c:ae:43:fb:c6:c0:b4:e8:bc:94:c3  root@travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 (DSA)
Aug  9 16:37:05 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ec2: 256 af:26:55:94:de:38:bf:fa:a7:75:93:1a:f0:ef:5d:a3  root@travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 (ECDSA)
Aug  9 16:37:05 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ec2: 256 d0:4f:82:17:38:89:dd:a1:33:28:5d:30:d2:70:2f:67  root@travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 (ED25519)
Aug  9 16:37:05 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ec2: 2048 5f:81:f9:84:7d:86:34:aa:2a:51:9d:0c:65:17:d2:39  root@travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 (RSA)
Aug  9 16:37:05 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 16:37:05 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ec2: #############################################################
Aug  9 16:38:30 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [  128.739585] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 16:39:25 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [  184.066404] device vethe137a17 entered promiscuous mode
Aug  9 16:39:25 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [  184.066477] docker0: port 1(vethe137a17) entered forwarding state
Aug  9 16:39:25 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [  184.066485] docker0: port 1(vethe137a17) entered forwarding state
Aug  9 16:39:25 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [  184.066964] docker0: port 1(vethe137a17) entered disabled state
Aug  9 16:39:25 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [  184.158529] cgroup: docker-runc (4876) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 16:39:25 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [  184.158532] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  9 16:39:25 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [  184.226759] eth0: renamed from veth186b221
Aug  9 16:39:25 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [  184.271762] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  9 16:39:25 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [  184.273026] docker0: port 1(vethe137a17) entered forwarding state
Aug  9 16:39:25 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [  184.273045] docker0: port 1(vethe137a17) entered forwarding state
Aug  9 16:39:25 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [  184.273069] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  9 16:39:28 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpd[1885]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  9 16:39:28 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpd[1885]: Listen normally on 6 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  9 16:39:28 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpd[1885]: Listen normally on 7 docker0 fe80::42:13ff:feed:9e1c UDP 123
Aug  9 16:39:28 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpd[1885]: peers refreshed
Aug  9 16:39:28 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 ntpd[1885]: new interface(s) found: waking up resolver
Aug  9 16:39:40 travis-job-92445257-ddbe-4dc5-af74-0911bc364b46 kernel: [  199.277844] docker0: port 1(vethe137a17) entered forwarding state
---
travis_time:end:11dcbba8:start=1533832959983280247,finish=1533832959989906051,duration=6625804
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16d621c4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:074ba9eb
travis_time:start:074ba9eb
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:08587e4e
$ dmesg | grep -i kill
