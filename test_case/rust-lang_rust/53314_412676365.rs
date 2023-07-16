plain

[00:04:17] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:17] tidy error: /checkout/src/librustc_mir/borrow_check/nll/type_check/liveness/local_use_map.rs: incorrect license
[00:04:17] tidy error: /checkout/src/librustc_mir/util/liveness.rs:430: line longer than 100 chars
[00:04:18] some tidy checks failed
[00:04:18] 
[00:04:18] 
[00:04:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:18] 
[00:04:18] 
[00:04:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:18] Build completed unsuccessfully in 0:00:49
[00:04:18] Build completed unsuccessfully in 0:00:49
[00:04:18] Makefile:79: recipe for target 'tidy' failed
[00:04:18] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:003783ab
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:01eedb28
$ sudo tail -n 500 /var/log/syslog
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] kvm-clock: using sched offset of 1628129998 cycles
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] Zone ranges:
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000]   Device   empty
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] Movable zone start for each node
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] Early memory node ranges
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] Policy zone: Normal
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] Hierarchical RCU implementation.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] console [ttyS0] enabled
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.447735] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.448947] pid_max: default: 32768 minimum: 301
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.449794] ACPI: Core revision 20150930
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.456428] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.457555] Security Framework initialized
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.458391] Yama: becoming mindful.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.459144] AppArmor: AppArmor disabled by boot time parameter
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.461907] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.471539] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.475939] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.477269] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.478595] Initializing cgroup subsys io
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.479358] Initializing cgroup subsys memory
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.480072] Initializing cgroup subsys devices
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.480828] Initializing cgroup subsys freezer
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.481889] Initializing cgroup subsys net_cls
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.482518] Initializing cgroup subsys perf_event
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.483261] Initializing cgroup subsys net_prio
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.484078] Initializing cgroup subsys hugetlb
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.484934] Initializing cgroup subsys pids
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.485744] CPU: Physical Processor ID: 0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.486441] CPU: Processor Core ID: 0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.487114] mce: CPU supports 32 MCE banks
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.487913] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.488731] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.492225] Freeing SMP alternatives memory: 32K
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.503204] ftrace: allocating 32185 entries in 126 pages
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.559541] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.560948] smpboot: Max logical packages: 2
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.562213] x2apic enabled
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.564478] Switched APIC routing to physical x2apic.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.568449] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.674999] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.677931] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.681387] x86: Booting SMP configuration:
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.682431] .... node  #0, CPUs:      #1
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.683383] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.687950]  #2
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.688780] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.692574]  #3
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.693307] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.698056] x86: Booted up 1 node, 4 CPUs
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.699459] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.703333] devtmpfs: initialized
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.708481] evm: security.selinux
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.709283] evm: security.SMACK64
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.710075] evm: security.SMACK64EXEC
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.711014] evm: security.SMACK64TRANSMUTE
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.712310] evm: security.SMACK64MMAP
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.713588] evm: security.ima
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.714311] evm: security.capability
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.715768] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.718592] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.720936] pinctrl core: initialized pinctrl subsystem
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.722491] RTC time: 21:32:12, date: 08/13/18
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.724711] NET: Registered protocol family 16
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.735034] cpuidle: using governor ladder
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.747056] cpuidle: using governor menu
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.748424] PCCT header not found.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.749700] ACPI: bus type PCI registered
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.751049] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.753931] PCI: Using configuration type 1 for base access
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.768130] ACPI: Added _OSI(Module Device)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.769725] ACPI: Added _OSI(Processor Device)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.771294] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.772954] ACPI: Added _OSI(Processor Aggregator Device)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.778461] ACPI: Executed 2 blocks of module-level executable AML code
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.805147] ACPI: Interpreter enabled
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.806408] ACPI: (supports S0 S3 S4 S5)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.807814] ACPI: Using IOAPIC for interrupt routing
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.809620] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.843465] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.846132] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.848853] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.851067] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.856392] PCI host bridge to bus 0000:00
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.857602] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.860415] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.862841] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.865361] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.868001] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.869717] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.870167] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.891930] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.915595] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.918402] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.927250] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.935132] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.954698] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.963280] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.969883] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.989704] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.993658] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    0.997665] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.001534] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.005161] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.027952] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.030999] vgaarb: loaded
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.032079] SCSI subsystem initialized
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.034108] libata version 3.00 loaded.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.034138] ACPI: bus type USB registered
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.035960] usbcore: registered new interface driver usbfs
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.037948] usbcore: registered new interface driver hub
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.039795] usbcore: registered new device driver usb
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.042021] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.044423] dmi: Firmware registration failed.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.046486] PCI: Using ACPI for IRQ routing
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.048137] PCI: pci_cache_line_size set to 64 bytes
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.048247] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.048250] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.048437] NetLabel: Initializing
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.049951] NetLabel:  domain hash size = 128
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.051441] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.053265] NetLabel:  unlabeled traffic allowed by default
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.055273] amd_nb: Cannot enumerate AMD northbridges
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.056828] clocksource: Switched to clocksource kvm-clock
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.066841] pnp: PnP ACPI init
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.068122] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.068199] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.068247] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.068328] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.068463] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.068509] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.068697] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.068923] pnp: PnP ACPI: found 7 devices
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.077659] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.080330] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.080334] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.080336] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.080337] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.080388] NET: Registered protocol family 2
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.082470] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.085725] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.088015] TCP: Hash tables configured (established 131072 bind 65536)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.090361] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.093254] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.095703] NET: Registered protocol family 1
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.097130] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.099725] PCI: CLS 0 bytes, default 64
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    1.100518] Unpacking initramfs...
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.313924] Freeing initrd memory: 21432K
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.315423] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.317332] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.320716] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.324464] hw unit of domain pp0-core 2^-0 Joules
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.325918] hw unit of domain package 2^-0 Joules
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.327714] hw unit of domain dram 2^-0 Joules
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.329087] Scanning for low memory corruption every 60 seconds
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.332083] audit: initializing netlink subsys (disabled)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.333682] audit: type=2000 audit(1534195934.806:1): initialized
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.335679] Initialise system trusted keyring
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.337430] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.339311] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.343432] zbud: loaded
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.344627] VFS: Disk quotas dquot_6.6.0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.346332] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.348927] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.351707] fuse init (API version 7.23)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.353270] Key type big_key registered
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.354744] Allocating IMA MOK and blacklist keyrings.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.359402] Key type asymmetric registered
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.360655] Asymmetric key parser 'x509' registered
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.362252] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.364545] io scheduler noop registered
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.365870] io scheduler deadline registered (default)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.367602] io scheduler cfq registered
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.369041] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.370749] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.372735] intel_idle: does not run on family 6 model 45
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.372872] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.375030] ACPI: Power Button [PWRF]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.376823] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.379288] ACPI: Sleep Button [SLPF]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.381000] GHES: HEST is not enabled!
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.384608] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.386554] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.393931] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.396200] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.404741] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.428741] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.453548] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.478440] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.503071] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.507872] Linux agpgart interface v0.103
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.511692] loop: module loaded
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.513382] libphy: Fixed MDIO Bus: probed
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.514680] tun: Universal TUN/TAP device driver, 1.6
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.516525] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.563644] PPP generic driver version 2.4.2
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.565771] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.567983] ehci-pci: EHCI PCI platform driver
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.569643] ehci-platform: EHCI generic platform driver
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.571407] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.574470] ohci-pci: OHCI PCI platform driver
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.576531] ohci-platform: OHCI generic platform driver
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.578752] uhci_hcd: USB Universal Host Controller Interface driver
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.581980] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.586822] i8042: Warning: Keylock active
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.589741] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.592212] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.594847] mousedev: PS/2 mouse device common for all mice
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.597939] rtc_cmos 00:00: RTC can wake from S4
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.600628] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.603774] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.606823] i2c /dev entries driver
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.608504] device-mapper: uevent: version 1.0.3
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.610880] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.614513] ledtrig-cpu: registered to indicate activity on CPUs
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.618325] NET: Registered protocol family 10
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.621184] NET: Registered protocol family 17
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.623351] Key type dns_resolver registered
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.625899] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.628364] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.631531] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.634587] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.636978] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.641854] registered taskstats version 1
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.643704] Loading compiled-in X.509 certificates
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.646905] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.651475] zswap: loaded using pool lzo/zbud
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.656672] Key type trusted registered
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.663669] Key type encrypted registered
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.665573] ima: No TPM chip found, activating TPM-bypass!
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.668297] evm: HMAC attrs: 0x1
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.670353]   Magic number: 14:694:550
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.672367] tty ttyS26: hash matches
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.673874] clockevents clockevent1: hash matches
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.676037] acpi PNP0C0F:01: hash matches
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.678348] rtc_cmos 00:00: setting system clock to 2018-08-13 21:32:15 UTC (1534195935)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.682175] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.685219] EDD information not available.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.687365] PM: Hibernation image not present or could not be loaded.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.688892] Freeing unused kernel memory: 1496K
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.690407] Write protecting the kernel read-only data: 14336k
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.693491] Freeing unused kernel memory: 1956K
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.695266] Freeing unused kernel memory: 92K
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.712961] systemd-udevd[118]: starting version 204
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.776862] scsi host0: Virtio SCSI HBA
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.785872] AVX version of gcm_enc/dec engaged.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.787605] AES CTR mode by8 optimization enabled
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.791724] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.801196] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.840521] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.840569] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.840571] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.848877] sd 0:0:1:0: [sda] Write Protect is off
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.850395] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.850626] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.856038]  sda: sda1
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    3.858286] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    4.328952] tsc: Refined TSC clocksource calibration: 2599.775 MHz
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    4.331132] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2579679c25e, max_idle_ns: 440795304456 ns
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    4.638065] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    6.773022] floppy0: no floppy controllers found
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    7.928869] raid6: sse2x1   gen()  9068 MB/s
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    7.996857] raid6: sse2x1   xor()  6536 MB/s
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.064854] raid6: sse2x2   gen() 11441 MB/s
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.132866] raid6: sse2x2   xor()  8004 MB/s
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.200858] raid6: sse2x4   gen() 12869 MB/s
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.268861] raid6: sse2x4   xor()  9087 MB/s
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.269933] raid6: using algorithm sse2x4 gen() 12869 MB/s
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.270866] raid6: .... xor() 9087 MB/s, rmw enabled
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.272002] raid6: using ssse3x2 recovery algorithm
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.274239] xor: automatically using best checksumming function:
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.312857]    avx       : 22282.000 MB/sec
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.327941] Btrfs loaded
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.369575] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.370678] EXT4-fs (sda1): write access will be enabled during recovery
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.444614] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.452241] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.453023] EXT4-fs (sda1): recovery complete
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.457988] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.672080] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.793184] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    8.852218] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    9.043948] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    9.569838] random: cloud-init: uninitialized urandom read (32 bytes read, 47 bits of entropy available)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    9.708150] systemd-udevd[704]: starting version 204
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    9.806822] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    9.868656] intel_rapl: no valid rapl domains found in package 0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    9.922759] intel_rapl: no valid rapl domains found in package 0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [    9.925371] ppdev: user-space parallel port driver
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   10.007297] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   10.056337] random: mktemp: uninitialized urandom read (6 bytes read, 60 bits of entropy available)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   10.113621] random: cloud-init: uninitialized urandom read (32 bytes read, 60 bits of entropy available)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   10.273208] random: cloud-init: uninitialized urandom read (32 bytes read, 60 bits of entropy available)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   10.513544] random: mktemp: uninitialized urandom read (12 bytes read, 63 bits of entropy available)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   10.583741] random: mktemp: uninitialized urandom read (6 bytes read, 64 bits of entropy available)
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   10.658584] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   10.705200] EXT4-fs (sda1): resized filesystem to 7864064
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   11.328272] init: failsafe main process (1096) killed by TERM signal
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO Running set_multiqueue.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO Set channels for eth0 to 4.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 13 21:32:23 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Starting Google Accounts daemon.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Creating a new user account for me.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Created user account me.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Creating a new user account for henrik.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   12.414284] random: nonblocking pool is initialized
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Created user account henrik.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Creating a new user account for emma.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Created user account emma.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Creating a new user account for igor.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Created user account igor.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Created user account konstantinhaase.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Creating a new user account for aj.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Created user account aj.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Creating a new user account for solarce.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Created user account solarce.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Creating a new user account for asari.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   12.792789] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   12.796256] Bridge firewalling registered
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   12.812476] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Created user account asari.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Creating a new user account for bogdana.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   12.851602] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   12.868924] floppy0: no floppy controllers found
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   12.869130] work still pending
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Created user account bogdana.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Creating a new user account for konstantin.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   12.953613] Initializing XFRM netlink socket
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   12.970646] Netfilter messages via NETLINK v0.30.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   12.973550] ctnetlink v0.93: registering with nfnetlink.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Created user account konstantin.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Creating a new user account for carmen.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Created user account carmen.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Creating a new user account for maria.
Aug 13 21:32:24 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Created user account maria.
Aug 13 21:32:25 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Removing user packer.
Aug 13 21:32:25 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e google-accounts: INFO Removing user packer.
Aug 13 21:32:25 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 21:32:25 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 21:32:25 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e cron[1709]: (CRON) INFO (pidfile fd = 3)
Aug 13 21:32:25 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e cron[1746]: (CRON) STARTUP (fork ok)
Aug 13 21:32:25 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e cron[1746]: (CRON) INFO (Running @reboot jobs)
Aug 13 21:32:25 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e acpid: starting up with netlink and the input layer
Aug 13 21:32:25 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e acpid: 1 rule loaded
Aug 13 21:32:25 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e acpid: waiting for events: event logging is off
Aug 13 21:32:25 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e haveged: haveged starting up
Aug 13 21:32:25 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   13.496500] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpd[1846]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpd[1847]: proto: precision = 0.104 usec
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpd[1847]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpd[1847]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpd[1847]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpd[1847]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpd[1847]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpd[1847]: Listen normally on 3 eth0 10.20.0.53 UDP 123
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpd[1847]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpd[1847]: peers refreshed
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpd[1847]: Listening on routing socket on fd #21 for interface updates
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [   18.697187] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e startup-script: INFO Found startup-script in metadata.
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e startup-script: INFO startup-script: job 1 at Tue Aug 14 00:42:00 2018
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e startup-script: INFO startup-script: Return code 0.
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e startup-script: INFO startup-script: Return code 0.
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e startup-script: INFO Finished running startup scripts.
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ec2: 
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ec2: #############################################################
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ec2: 1024 f0:a1:bc:22:de:d8:18:45:65:8c:87:fb:a2:d6:87:d1  root@travis-job-0a5c068c-21a7-4589-8820-45fa5835183e (DSA)
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ec2: 256 6a:78:ba:29:c1:bf:0c:b2:16:ae:97:f4:ea:98:39:a2  root@travis-job-0a5c068c-21a7-4589-8820-45fa5835183e (ECDSA)
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ec2: 256 14:61:72:6c:22:be:24:5c:22:50:d4:60:24:ed:96:df  root@travis-job-0a5c068c-21a7-4589-8820-45fa5835183e (ED25519)
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ec2: 2048 79:50:10:39:9b:df:97:7f:15:95:ae:3a:0e:2f:e7:5b  root@travis-job-0a5c068c-21a7-4589-8820-45fa5835183e (RSA)
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 13 21:32:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ec2: #############################################################
Aug 13 21:32:39 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpdate[2240]: the NTP socket is in use, exiting
Aug 13 21:35:20 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [  188.661968] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 13 21:36:27 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [  255.188265] device veth410f1a7 entered promiscuous mode
Aug 13 21:36:27 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [  255.188374] docker0: port 1(veth410f1a7) entered forwarding state
Aug 13 21:36:27 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [  255.188383] docker0: port 1(veth410f1a7) entered forwarding state
Aug 13 21:36:27 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [  255.188896] docker0: port 1(veth410f1a7) entered disabled state
Aug 13 21:36:27 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [  255.287798] cgroup: docker-runc (4844) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 13 21:36:27 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [  255.287802] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 13 21:36:27 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [  255.367267] eth0: renamed from veth2d9b39c
Aug 13 21:36:27 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [  255.411529] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 13 21:36:27 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [  255.412555] docker0: port 1(veth410f1a7) entered forwarding state
Aug 13 21:36:27 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [  255.412573] docker0: port 1(veth410f1a7) entered forwarding state
Aug 13 21:36:27 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [  255.412601] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 13 21:36:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpd[1847]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 13 21:36:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpd[1847]: Listen normally on 6 docker0 fe80::42:38ff:fe0d:89a9 UDP 123
Aug 13 21:36:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpd[1847]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 13 21:36:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpd[1847]: peers refreshed
Aug 13 21:36:30 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e ntpd[1847]: new interface(s) found: waking up resolver
Aug 13 21:36:42 travis-job-0a5c068c-21a7-4589-8820-45fa5835183e kernel: [  270.466691] docker0: port 1(veth410f1a7) entered forwarding state
---
travis_time:end:161d52c5:start=1534196381011639783,finish=1534196381018014432,duration=6374649
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:072c4f53
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:277e0c78
travis_time:start:277e0c78
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:18956359
$ dmesg | grep -i kill
