plain

[00:03:56] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:56] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:72: TODO is deprecated; use FIXME
[00:03:56] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:100: TODO is deprecated; use FIXME
[00:03:56] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:108: TODO is deprecated; use FIXME
[00:03:56] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:136: TODO is deprecated; use FIXME
[00:03:56] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:164: TODO is deprecated; use FIXME
[00:03:56] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:172: TODO is deprecated; use FIXME
[00:03:56] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:195: TODO is deprecated; use FIXME
[00:03:56] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:283: TODO is deprecated; use FIXME
[00:03:56] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:318: TODO is deprecated; use FIXME
[00:03:57] some tidy checks failed
[00:03:57] 
[00:03:57] 
[00:03:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:57] 
[00:03:57] 
[00:03:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:57] Build completed unsuccessfully in 0:00:53
[00:03:57] Build completed unsuccessfully in 0:00:53
[00:03:57] make: *** [tidy] Error 1
[00:03:57] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:028300e0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:02671ff4
$ sudo tail -n 500 /var/log/syslog
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] kvm-clock: using sched offset of 1576980534 cycles
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] Zone ranges:
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000]   Device   empty
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] Movable zone start for each node
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] Early memory node ranges
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] Policy zone: Normal
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] console [ttyS0] enabled
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.588586] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.593605] pid_max: default: 32768 minimum: 301
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.596270] ACPI: Core revision 20150930
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.604177] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.608536] Security Framework initialized
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.611045] Yama: becoming mindful.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.612998] AppArmor: AppArmor disabled by boot time parameter
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.617937] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.630470] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.638770] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.643999] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.650018] Initializing cgroup subsys io
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.652694] Initializing cgroup subsys memory
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.655707] Initializing cgroup subsys devices
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.658363] Initializing cgroup subsys freezer
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.661270] Initializing cgroup subsys net_cls
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.663718] Initializing cgroup subsys perf_event
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.666203] Initializing cgroup subsys net_prio
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.668601] Initializing cgroup subsys hugetlb
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.671408] Initializing cgroup subsys pids
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.674214] CPU: Physical Processor ID: 0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.676430] CPU: Processor Core ID: 0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.678607] mce: CPU supports 32 MCE banks
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.680555] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.683709] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.689401] Freeing SMP alternatives memory: 32K
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.701644] ftrace: allocating 32185 entries in 126 pages
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.759850] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.764142] smpboot: Max logical packages: 2
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.767378] x2apic enabled
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.770086] Switched APIC routing to physical x2apic.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.775913] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.886169] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.892032] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.898611] x86: Booting SMP configuration:
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.900927] .... node  #0, CPUs:      #1
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.903084] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.908853]  #2
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.910211] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.916407]  #3
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.917549] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.923363] x86: Booted up 1 node, 4 CPUs
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.925356] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.931084] devtmpfs: initialized
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.936477] evm: security.selinux
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.938147] evm: security.SMACK64
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.940117] evm: security.SMACK64EXEC
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.942497] evm: security.SMACK64TRANSMUTE
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.945390] evm: security.SMACK64MMAP
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.947680] evm: security.ima
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.949309] evm: security.capability
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.951674] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.957472] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.961239] pinctrl core: initialized pinctrl subsystem
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.963978] RTC time: 20:34:26, date: 08/12/18
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.968188] NET: Registered protocol family 16
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.982229] cpuidle: using governor ladder
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.994228] cpuidle: using governor menu
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.996406] PCCT header not found.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    0.998639] ACPI: bus type PCI registered
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.000720] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.004103] PCI: Using configuration type 1 for base access
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.019907] ACPI: Added _OSI(Module Device)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.022204] ACPI: Added _OSI(Processor Device)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.024152] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.026866] ACPI: Added _OSI(Processor Aggregator Device)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.031935] ACPI: Executed 2 blocks of module-level executable AML code
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.058853] ACPI: Interpreter enabled
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.060661] ACPI: (supports S0 S3 S4 S5)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.063116] ACPI: Using IOAPIC for interrupt routing
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.065598] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.099055] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.101542] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.104561] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.107546] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.113162] PCI host bridge to bus 0000:00
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.115050] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.117578] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.120249] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.122809] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.125324] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.127358] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.127808] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.147706] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.165210] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.167309] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.173538] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.178738] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.194295] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.199500] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.203771] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.220416] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.222444] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.224427] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.226423] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.228585] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.248537] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.249527] vgaarb: loaded
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.250205] SCSI subsystem initialized
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.250865] libata version 3.00 loaded.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.250893] ACPI: bus type USB registered
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.251470] usbcore: registered new interface driver usbfs
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.252361] usbcore: registered new interface driver hub
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.253256] usbcore: registered new device driver usb
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.254470] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.255502] dmi: Firmware registration failed.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.256265] PCI: Using ACPI for IRQ routing
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.256902] PCI: pci_cache_line_size set to 64 bytes
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.256995] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.256996] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.257116] NetLabel: Initializing
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.257653] NetLabel:  domain hash size = 128
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.258255] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.259000] NetLabel:  unlabeled traffic allowed by default
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.259956] amd_nb: Cannot enumerate AMD northbridges
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.260732] clocksource: Switched to clocksource kvm-clock
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.268324] pnp: PnP ACPI init
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.268936] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.269012] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.269063] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.269119] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.269165] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.269241] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.269290] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.269476] pnp: PnP ACPI: found 7 devices
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.276992] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.278474] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.278477] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.278479] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.278480] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.278514] NET: Registered protocol family 2
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.279512] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.281493] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.282703] TCP: Hash tables configured (established 131072 bind 65536)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.283862] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.284849] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.286763] NET: Registered protocol family 1
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.287750] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.288658] PCI: CLS 0 bytes, default 64
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    1.288714] Unpacking initramfs...
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.258976] Freeing initrd memory: 21432K
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.259824] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.261108] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.262605] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.263981] hw unit of domain pp0-core 2^-0 Joules
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.264635] hw unit of domain package 2^-0 Joules
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.265348] hw unit of domain dram 2^-0 Joules
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.266074] Scanning for low memory corruption every 60 seconds
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.267304] audit: initializing netlink subsys (disabled)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.268190] audit: type=2000 audit(1534106067.799:1): initialized
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.269532] Initialise system trusted keyring
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.270575] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.271484] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.273663] zbud: loaded
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.274367] VFS: Disk quotas dquot_6.6.0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.274947] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.276149] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.277392] fuse init (API version 7.23)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.278568] Key type big_key registered
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.279158] Allocating IMA MOK and blacklist keyrings.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.280954] Key type asymmetric registered
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.281585] Asymmetric key parser 'x509' registered
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.282323] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.283750] io scheduler noop registered
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.284339] io scheduler deadline registered (default)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.285090] io scheduler cfq registered
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.285714] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.286531] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.287472] intel_idle: does not run on family 6 model 45
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.287571] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.288608] ACPI: Power Button [PWRF]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.289200] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.290296] ACPI: Sleep Button [SLPF]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.291132] GHES: HEST is not enabled!
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.293228] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.294185] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.298265] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.299315] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.303657] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.326069] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.349057] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.371706] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.394312] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.397869] Linux agpgart interface v0.103
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.400999] loop: module loaded
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.402105] libphy: Fixed MDIO Bus: probed
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.403418] tun: Universal TUN/TAP device driver, 1.6
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.404940] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.436331] PPP generic driver version 2.4.2
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.437815] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.441661] ehci-pci: EHCI PCI platform driver
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.442715] ehci-platform: EHCI generic platform driver
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.444166] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.445911] ohci-pci: OHCI PCI platform driver
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.447018] ohci-platform: OHCI generic platform driver
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.448438] uhci_hcd: USB Universal Host Controller Interface driver
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.450441] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.453272] i8042: Warning: Keylock active
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.455207] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.456534] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.458056] mousedev: PS/2 mouse device common for all mice
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.459780] rtc_cmos 00:00: RTC can wake from S4
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.461550] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.463467] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.465196] i2c /dev entries driver
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.466420] device-mapper: uevent: version 1.0.3
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.468516] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.471112] ledtrig-cpu: registered to indicate activity on CPUs
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.473601] NET: Registered protocol family 10
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.475477] NET: Registered protocol family 17
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.476484] Key type dns_resolver registered
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.477782] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.479526] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.480919] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.481882] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.483676] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.485616] registered taskstats version 1
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.486561] Loading compiled-in X.509 certificates
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.488192] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.489844] zswap: loaded using pool lzo/zbud
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.492473] Key type trusted registered
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.496146] Key type encrypted registered
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.496937] ima: No TPM chip found, activating TPM-bypass!
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.498066] evm: HMAC attrs: 0x1
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.499081]   Magic number: 14:66:599
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.499886] serial8250 serial8250: hash matches
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.501120] rtc_cmos 00:00: setting system clock to 2018-08-12 20:34:28 UTC (1534106068)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.502938] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.504001] EDD information not available.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.504787] PM: Hibernation image not present or could not be loaded.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.506157] Freeing unused kernel memory: 1496K
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.506973] Write protecting the kernel read-only data: 14336k
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.508637] Freeing unused kernel memory: 1956K
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.509781] Freeing unused kernel memory: 92K
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.523284] systemd-udevd[118]: starting version 204
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.571842] scsi host0: Virtio SCSI HBA
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.577025] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.580289] AVX version of gcm_enc/dec engaged.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.581100] AES CTR mode by8 optimization enabled
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.612674] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.612689] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.612690] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.616892] sd 0:0:1:0: [sda] Write Protect is off
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.617645] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.617761] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.620704]  sda: sda1
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.621976] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    3.657081] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    4.264820] tsc: Refined TSC clocksource calibration: 2600.003 MHz
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    4.265754] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3ef07ae, max_idle_ns: 440795268508 ns
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    4.489869] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    6.569029] floppy0: no floppy controllers found
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    7.744768] raid6: sse2x1   gen()  8686 MB/s
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    7.812748] raid6: sse2x1   xor()  6500 MB/s
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    7.880745] raid6: sse2x2   gen() 10581 MB/s
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    7.948743] raid6: sse2x2   xor()  7127 MB/s
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.016742] raid6: sse2x4   gen() 12608 MB/s
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.084742] raid6: sse2x4   xor()  8895 MB/s
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.087777] raid6: using algorithm sse2x4 gen() 12608 MB/s
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.091099] raid6: .... xor() 8895 MB/s, rmw enabled
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.093961] raid6: using ssse3x2 recovery algorithm
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.098194] xor: automatically using best checksumming function:
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.140741]    avx       : 27115.000 MB/sec
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.157486] Btrfs loaded
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.218523] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.222453] EXT4-fs (sda1): write access will be enabled during recovery
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.314471] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.323546] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.326425] EXT4-fs (sda1): recovery complete
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.334664] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.578819] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.710234] random: mountall: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    8.773352] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    9.001584] random: cloud-init: uninitialized urandom read (32 bytes read, 34 bits of entropy available)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    9.637475] random: cloud-init: uninitialized urandom read (32 bytes read, 41 bits of entropy available)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    9.785010] systemd-udevd[702]: starting version 204
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    9.914335] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [    9.961738] intel_rapl: no valid rapl domains found in package 0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   10.002130] intel_rapl: no valid rapl domains found in package 0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   10.029662] ppdev: user-space parallel port driver
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   10.132389] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   10.187998] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   10.253381] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   10.421257] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   10.705956] random: mktemp: uninitialized urandom read (12 bytes read, 55 bits of entropy available)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   10.788045] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   10.872381] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   10.921286] EXT4-fs (sda1): resized filesystem to 7864064
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   11.173611] init: failsafe main process (1093) killed by TERM signal
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO Running set_multiqueue.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO Set channels for eth0 to 4.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Starting Google Accounts daemon.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 12 20:34:36 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Creating a new user account for me.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Created user account me.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Created user account me.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Creating a new user account for henrik.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Created user account henrik.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Creating a new user account for emma.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Created user account emma.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Creating a new user account for igor.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Created user account igor.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Created user account konstantinhaase.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Creating a new user account for aj.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Created user account aj.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Creating a new user account for solarce.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Created user account solarce.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Creating a new user account for asari.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Created user account asari.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Creating a new user account for bogdana.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   12.591515] random: nonblocking pool is initialized
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Created user account bogdana.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Creating a new user account for konstantin.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   12.665160] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Created user account konstantin.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   12.668992] Bridge firewalling registered
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   12.678248] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Creating a new user account for carmen.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   12.715716] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Created user account carmen.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Creating a new user account for maria.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   12.781689] Initializing XFRM netlink socket
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   12.788617] Netfilter messages via NETLINK v0.30.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   12.791511] ctnetlink v0.93: registering with nfnetlink.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Created user account maria.
Aug 12 20:34:37 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 google-accounts: INFO Removing user packer.
Aug 12 20:34:38 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   13.000997] floppy0: no floppy controllers found
Aug 12 20:34:38 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 12 20:34:38 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 12 20:34:39 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 cron[1710]: (CRON) INFO (pidfile fd = 3)
Aug 12 20:34:39 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 12 20:34:39 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 cron[1742]: (CRON) STARTUP (fork ok)
Aug 12 20:34:39 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 12 20:34:39 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 cron[1742]: (CRON) INFO (Running @reboot jobs)
Aug 12 20:34:39 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 acpid: starting up with netlink and the input layer
Aug 12 20:34:39 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 acpid: 1 rule loaded
Aug 12 20:34:39 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 acpid: waiting for events: event logging is off
Aug 12 20:34:39 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 haveged: haveged starting up
Aug 12 20:34:39 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   14.413474] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpd[1844]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpd[1845]: proto: precision = 0.100 usec
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpd[1845]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpd[1845]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpd[1845]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpd[1845]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpd[1845]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpd[1845]: Listen normally on 3 eth0 10.20.255.15 UDP 123
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpd[1845]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpd[1845]: peers refreshed
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpd[1845]: Listening on routing socket on fd #21 for interface updates
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [   19.605591] init: plymouth-upstart-bridge main process ended, respawning
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 startup-script: INFO Found startup-script in metadata.
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 startup-script: INFO startup-script: job 1 at Sun Aug 12 23:44:00 2018
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 startup-script: INFO startup-script: Return code 0.
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 startup-script: INFO startup-script: Return code 0.
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 startup-script: INFO Finished running startup scripts.
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ec2: 
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ec2: #############################################################
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ec2: 1024 d4:f6:cc:76:aa:1e:3f:03:d2:6f:63:28:23:42:34:79  root@travis-job-fc817534-c622-471d-acb0-7ad98b868c10 (DSA)
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ec2: 256 87:55:6f:4a:f7:cf:f7:5e:ad:52:e8:c6:6c:fc:87:2d  root@travis-job-fc817534-c622-471d-acb0-7ad98b868c10 (ECDSA)
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ec2: 256 7b:69:9b:8e:6d:66:3b:de:d5:7a:26:cf:aa:6a:76:98  root@travis-job-fc817534-c622-471d-acb0-7ad98b868c10 (ED25519)
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ec2: 2048 d7:65:2d:31:d7:eb:fd:5f:98:52:2f:b4:ef:6b:7c:b4  root@travis-job-fc817534-c622-471d-acb0-7ad98b868c10 (RSA)
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 12 20:34:44 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ec2: #############################################################
Aug 12 20:34:53 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpdate[2238]: the NTP socket is in use, exiting
Aug 12 20:36:16 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [  111.352680] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 12 20:37:16 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [  171.161391] device veth9e8ec19 entered promiscuous mode
Aug 12 20:37:16 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [  171.258467] cgroup: docker-runc (4839) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 12 20:37:16 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [  171.258470] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 12 20:37:16 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [  171.330097] eth0: renamed from vethd50c751
Aug 12 20:37:16 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [  171.366411] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 12 20:37:16 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [  171.367590] docker0: port 1(veth9e8ec19) entered forwarding state
Aug 12 20:37:16 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [  171.367625] docker0: port 1(veth9e8ec19) entered forwarding state
Aug 12 20:37:16 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [  171.367653] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 12 20:37:19 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpd[1845]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 12 20:37:19 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpd[1845]: Listen normally on 6 docker0 fe80::42:71ff:feac:1c0e UDP 123
Aug 12 20:37:19 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpd[1845]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 12 20:37:19 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpd[1845]: peers refreshed
Aug 12 20:37:19 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 ntpd[1845]: new interface(s) found: waking up resolver
Aug 12 20:37:31 travis-job-fc817534-c622-471d-acb0-7ad98b868c10 kernel: [  186.413397] docker0: port 1(veth9e8ec19) entered forwarding state
---
travis_time:end:00ea51f0:start=1534106415361917051,finish=1534106415368430134,duration=6513083
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01e2ac06
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00b25902
travis_time:start:00b25902
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:03d8d05d
$ dmesg | grep -i kill
