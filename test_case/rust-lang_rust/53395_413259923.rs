plain
tidy check
[00:04:50] * 555 error codes
[00:04:50] * highest error code: E0712
[00:04:50] * 220 features
[00:04:50] tidy error: The Unstable Book has a 'library feature' section 'future-atomic-orderings' which doesn't correspond to an unstable library feature
[00:04:50] tidy error: The Unstable Book has a 'library feature' section 'io-error-internals' which doesn't correspond to an unstable library feature
[00:04:51] some tidy checks failed
[00:04:51] 
[00:04:51] 
[00:04:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:51] 
[00:04:51] 
[00:04:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:51] Build completed unsuccessfully in 0:00:52
[00:04:51] Build completed unsuccessfully in 0:00:52
[00:04:51] Makefile:79: recipe for target 'tidy' failed
[00:04:51] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05da2918
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:24d75f40
$ sudo tail -n 500 /var/log/syslog
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] kvm-clock: using sched offset of 1656449222 cycles
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] Zone ranges:
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000]   Device   empty
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] Movable zone start for each node
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] Early memory node ranges
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] Policy zone: Normal
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] console [ttyS0] enabled
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.486361] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.489948] pid_max: default: 32768 minimum: 301
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.491824] ACPI: Core revision 20150930
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.499735] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.503176] Security Framework initialized
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.506775] Yama: becoming mindful.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.508467] AppArmor: AppArmor disabled by boot time parameter
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.513015] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.525531] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.531649] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.535330] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.538559] Initializing cgroup subsys io
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.540079] Initializing cgroup subsys memory
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.541879] Initializing cgroup subsys devices
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.543876] Initializing cgroup subsys freezer
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.545723] Initializing cgroup subsys net_cls
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.547639] Initializing cgroup subsys perf_event
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.549329] Initializing cgroup subsys net_prio
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.550948] Initializing cgroup subsys hugetlb
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.552629] Initializing cgroup subsys pids
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.555197] CPU: Physical Processor ID: 0
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.556406] CPU: Processor Core ID: 0
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.559256] mce: CPU supports 32 MCE banks
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.561424] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.563550] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.568282] Freeing SMP alternatives memory: 32K
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.580555] ftrace: allocating 32185 entries in 126 pages
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.642118] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.645469] smpboot: Max logical packages: 2
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.647566] x2apic enabled
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.650563] Switched APIC routing to physical x2apic.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.656268] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.764867] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.769448] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.774378] x86: Booting SMP configuration:
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.776867] .... node  #0, CPUs:      #1
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.778871] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.785526]  #2
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.786790] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.793487]  #3
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.794330] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.800495] x86: Booted up 1 node, 4 CPUs
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.802579] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.807167] devtmpfs: initialized
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.812536] evm: security.selinux
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.813795] evm: security.SMACK64
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.815135] evm: security.SMACK64EXEC
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.817895] evm: security.SMACK64TRANSMUTE
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.820111] evm: security.SMACK64MMAP
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.822306] evm: security.ima
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.823456] evm: security.capability
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.825507] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.830539] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.833514] pinctrl core: initialized pinctrl subsystem
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.836157] RTC time: 16:32:55, date: 08/15/18
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.839163] NET: Registered protocol family 16
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.852976] cpuidle: using governor ladder
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.864982] cpuidle: using governor menu
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.867186] PCCT header not found.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.868551] ACPI: bus type PCI registered
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.870695] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.873199] PCI: Using configuration type 1 for base access
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.891113] ACPI: Added _OSI(Module Device)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.893816] ACPI: Added _OSI(Processor Device)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.895456] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.897038] ACPI: Added _OSI(Processor Aggregator Device)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.902363] ACPI: Executed 2 blocks of module-level executable AML code
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.930772] ACPI: Interpreter enabled
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.933179] ACPI: (supports S0 S3 S4 S5)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.935490] ACPI: Using IOAPIC for interrupt routing
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.937822] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.973355] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.976592] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.979113] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.982486] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.988507] PCI host bridge to bus 0000:00
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.991074] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.994866] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    0.997862] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.000571] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.004343] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.007092] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.007593] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.032487] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.058478] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.061681] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.071050] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.078977] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.101951] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.110851] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.118600] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.140299] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.145655] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.151672] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.156942] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.161105] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.184058] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.186930] vgaarb: loaded
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.188092] SCSI subsystem initialized
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.190335] libata version 3.00 loaded.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.190379] ACPI: bus type USB registered
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.191976] usbcore: registered new interface driver usbfs
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.194328] usbcore: registered new interface driver hub
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.196343] usbcore: registered new device driver usb
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.198497] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.201502] dmi: Firmware registration failed.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.204445] PCI: Using ACPI for IRQ routing
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.205702] PCI: pci_cache_line_size set to 64 bytes
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.205812] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.205814] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.205988] NetLabel: Initializing
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.207317] NetLabel:  domain hash size = 128
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.209002] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.210722] NetLabel:  unlabeled traffic allowed by default
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.213041] amd_nb: Cannot enumerate AMD northbridges
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.214626] clocksource: Switched to clocksource kvm-clock
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.223575] pnp: PnP ACPI init
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.225322] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.225416] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.225459] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.225510] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.225551] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.225589] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.225628] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.225795] pnp: PnP ACPI: found 7 devices
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.235172] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.237873] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.237875] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.237876] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.237878] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.237940] NET: Registered protocol family 2
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.239554] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.242433] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.244769] TCP: Hash tables configured (established 131072 bind 65536)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.246700] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.248814] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.251367] NET: Registered protocol family 1
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.252785] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.255007] PCI: CLS 0 bytes, default 64
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    1.255876] Unpacking initramfs...
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.446653] Freeing initrd memory: 21432K
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.448723] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.451607] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.454732] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.457663] hw unit of domain pp0-core 2^-0 Joules
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.459186] hw unit of domain package 2^-0 Joules
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.461996] hw unit of domain dram 2^-0 Joules
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.463536] Scanning for low memory corruption every 60 seconds
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.466180] audit: initializing netlink subsys (disabled)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.468241] audit: type=2000 audit(1534350778.086:1): initialized
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.470776] Initialise system trusted keyring
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.473087] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.475521] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.480145] zbud: loaded
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.481653] VFS: Disk quotas dquot_6.6.0
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.483677] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.487023] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.489885] fuse init (API version 7.23)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.491972] Key type big_key registered
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.493311] Allocating IMA MOK and blacklist keyrings.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.500124] Key type asymmetric registered
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.502007] Asymmetric key parser 'x509' registered
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.503994] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.507214] io scheduler noop registered
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.508920] io scheduler deadline registered (default)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.511833] io scheduler cfq registered
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.513138] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.515119] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.517872] intel_idle: does not run on family 6 model 62
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.517992] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.521053] ACPI: Power Button [PWRF]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.522259] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.524996] ACPI: Sleep Button [SLPF]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.527984] GHES: HEST is not enabled!
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.531667] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.533469] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.543331] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.545238] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.554565] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.578370] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.603644] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.628160] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.652978] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.657982] Linux agpgart interface v0.103
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.662276] loop: module loaded
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.664075] libphy: Fixed MDIO Bus: probed
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.666073] tun: Universal TUN/TAP device driver, 1.6
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.668171] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.729391] PPP generic driver version 2.4.2
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.731123] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.734121] ehci-pci: EHCI PCI platform driver
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.736169] ehci-platform: EHCI generic platform driver
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.738713] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.741580] ohci-pci: OHCI PCI platform driver
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.743237] ohci-platform: OHCI generic platform driver
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.745755] uhci_hcd: USB Universal Host Controller Interface driver
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.748496] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.753351] i8042: Warning: Keylock active
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.756598] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.759090] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.761859] mousedev: PS/2 mouse device common for all mice
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.764163] rtc_cmos 00:00: RTC can wake from S4
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.766754] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.769764] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.772106] i2c /dev entries driver
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.773460] device-mapper: uevent: version 1.0.3
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.775253] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.779124] ledtrig-cpu: registered to indicate activity on CPUs
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.782390] NET: Registered protocol family 10
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.785723] NET: Registered protocol family 17
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.787607] Key type dns_resolver registered
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.789755] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.792180] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.794540] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.798254] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.801763] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.807006] registered taskstats version 1
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.809238] Loading compiled-in X.509 certificates
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.812531] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.817815] zswap: loaded using pool lzo/zbud
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.822609] Key type trusted registered
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.830196] Key type encrypted registered
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.832053] ima: No TPM chip found, activating TPM-bypass!
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.834745] evm: HMAC attrs: 0x1
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.836607]   Magic number: 14:314:540
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.838337]  node: hash matches
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.839678] rtc_cmos 00:00: setting system clock to 2018-08-15 16:32:58 UTC (1534350778)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.842833] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.845755] EDD information not available.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.847358] PM: Hibernation image not present or could not be loaded.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.848924] Freeing unused kernel memory: 1496K
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.851191] Write protecting the kernel read-only data: 14336k
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.855273] Freeing unused kernel memory: 1956K
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.858640] Freeing unused kernel memory: 92K
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.876782] systemd-udevd[119]: starting version 204
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.949127] scsi host0: Virtio SCSI HBA
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.965071] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.968743] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.973012] AVX version of gcm_enc/dec engaged.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    3.974729] AES CTR mode by8 optimization enabled
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    4.017343] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    4.017797] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    4.017799] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    4.018573] sd 0:0:1:0: [sda] Write Protect is off
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    4.018576] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    4.018685] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    4.021008]  sda: sda1
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    4.022252] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    4.462763] tsc: Refined TSC clocksource calibration: 2499.759 MHz
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    4.465888] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24085654d7b, max_idle_ns: 440795285893 ns
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    4.809141] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    6.934856] floppy0: no floppy controllers found
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.114803] raid6: sse2x1   gen()  8584 MB/s
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.182681] raid6: sse2x1   xor()  6858 MB/s
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.250709] raid6: sse2x2   gen() 11073 MB/s
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.318738] raid6: sse2x2   xor()  7628 MB/s
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.386686] raid6: sse2x4   gen() 12291 MB/s
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.454694] raid6: sse2x4   xor()  8521 MB/s
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.456227] raid6: using algorithm sse2x4 gen() 12291 MB/s
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.458739] raid6: .... xor() 8521 MB/s, rmw enabled
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.461813] raid6: using ssse3x2 recovery algorithm
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.465688] xor: automatically using best checksumming function:
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.506805]    avx       : 22118.000 MB/sec
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.523709] Btrfs loaded
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.577493] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.581154] EXT4-fs (sda1): write access will be enabled during recovery
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.650908] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.663934] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.666252] EXT4-fs (sda1): recovery complete
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.673168] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.853706] random: init: uninitialized urandom read (12 bytes read, 23 bits of entropy available)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    8.960384] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    9.013052] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    9.212508] random: cloud-init: uninitialized urandom read (32 bytes read, 34 bits of entropy available)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    9.842471] random: cloud-init: uninitialized urandom read (32 bytes read, 42 bits of entropy available)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [    9.997527] systemd-udevd[701]: starting version 204
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   10.126379] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   10.230866] ppdev: user-space parallel port driver
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   10.371646] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   10.437164] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   10.503771] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   10.681102] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   10.954252] random: mktemp: uninitialized urandom read (12 bytes read, 56 bits of entropy available)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   11.038594] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   11.123325] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   11.165595] EXT4-fs (sda1): resized filesystem to 7864064
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   11.607627] init: failsafe main process (1093) killed by TERM signal
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO Running set_multiqueue.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO Set channels for eth0 to 4.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 15 16:33:06 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 15 16:33:07 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 16:33:07 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 16:33:07 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Starting Google Accounts daemon.
Aug 15 16:33:07 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 15 16:33:07 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Creating a new user account for me.
Aug 15 16:33:07 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Created user account me.
Aug 15 16:33:07 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Creating a new user account for henrik.
Aug 15 16:33:07 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Created user account henrik.
Aug 15 16:33:07 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Creating a new user account for emma.
Aug 15 16:33:07 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Created user account emma.
Aug 15 16:33:07 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Creating a new user account for igor.
Aug 15 16:33:07 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Created user account igor.
Aug 15 16:33:07 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 15 16:33:07 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   12.878841] random: nonblocking pool is initialized
Aug 15 16:33:07 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Created user account konstantinhaase.
Aug 15 16:33:07 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Creating a new user account for aj.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-clock-skew: INFO Synced system time with hardware clock.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Created user account aj.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Creating a new user account for solarce.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   13.029139] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   13.033664] Bridge firewalling registered
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   13.047051] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Created user account solarce.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   13.082987] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Creating a new user account for asari.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Created user account asari.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   13.171652] Initializing XFRM netlink socket
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   13.182382] Netfilter messages via NETLINK v0.30.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Creating a new user account for bogdana.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   13.185362] ctnetlink v0.93: registering with nfnetlink.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   13.201039] floppy0: no floppy controllers found
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   13.201290] work still pending
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Created user account bogdana.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Creating a new user account for konstantin.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Created user account konstantin.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Creating a new user account for carmen.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Created user account carmen.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Creating a new user account for maria.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Created user account maria.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 google-accounts: INFO Removing user packer.
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 cron[1706]: (CRON) INFO (pidfile fd = 3)
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 cron[1738]: (CRON) STARTUP (fork ok)
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 cron[1738]: (CRON) INFO (Running @reboot jobs)
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 acpid: starting up with netlink and the input layer
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 acpid: 1 rule loaded
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 acpid: waiting for events: event logging is off
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 haveged: haveged starting up
Aug 15 16:33:08 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   13.880282] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 16:33:13 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpd[1842]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 15 16:33:13 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpd[1843]: proto: precision = 0.103 usec
Aug 15 16:33:13 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpd[1843]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 15 16:33:13 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpd[1843]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 15 16:33:13 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpd[1843]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 15 16:33:13 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpd[1843]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 15 16:33:13 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpd[1843]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 15 16:33:13 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpd[1843]: Listen normally on 3 eth0 10.20.0.68 UDP 123
Aug 15 16:33:13 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpd[1843]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 15 16:33:13 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpd[1843]: peers refreshed
Aug 15 16:33:13 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpd[1843]: Listening on routing socket on fd #21 for interface updates
Aug 15 16:33:14 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [   19.049849] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 16:33:14 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 startup-script: INFO Found startup-script in metadata.
Aug 15 16:33:14 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 15 16:33:14 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 startup-script: INFO startup-script: job 1 at Wed Aug 15 19:43:00 2018
Aug 15 16:33:14 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 startup-script: INFO startup-script: Return code 0.
Aug 15 16:33:14 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 startup-script: INFO startup-script: Return code 0.
Aug 15 16:33:14 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 startup-script: INFO Finished running startup scripts.
Aug 15 16:33:14 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ec2: 
Aug 15 16:33:14 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ec2: #############################################################
Aug 15 16:33:14 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 15 16:33:14 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ec2: 1024 bb:42:8d:66:e4:d0:9f:6b:af:12:e4:7e:73:a5:89:0e  root@travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 (DSA)
Aug 15 16:33:14 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ec2: 256 62:bf:6f:3d:0f:1d:c5:81:d9:a0:22:a8:58:e6:11:2f  root@travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 (ECDSA)
Aug 15 16:33:14 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ec2: 256 01:3f:98:a7:ba:7f:13:0d:07:61:20:97:63:39:24:6c  root@travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 (ED25519)
Aug 15 16:33:14 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ec2: 2048 90:6f:81:c2:7f:28:e7:4f:7f:02:e6:ce:b2:41:07:bc  root@travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 (RSA)
Aug 15 16:33:14 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 15 16:33:14 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ec2: #############################################################
Aug 15 16:33:23 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpdate[2238]: the NTP socket is in use, exiting
Aug 15 16:35:32 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [  157.026512] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 15 16:37:24 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [  269.899357] device veth9415710 entered promiscuous mode
Aug 15 16:37:24 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [  269.899411] docker0: port 1(veth9415710) entered forwarding state
Aug 15 16:37:24 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [  269.899416] docker0: port 1(veth9415710) entered forwarding state
Aug 15 16:37:24 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [  269.899903] docker0: port 1(veth9415710) entered disabled state
Aug 15 16:37:25 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [  270.017166] cgroup: docker-runc (4940) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 15 16:37:25 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [  270.017169] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 15 16:37:25 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [  270.104718] eth0: renamed from vetha3a0ad3
Aug 15 16:37:25 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [  270.155105] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 15 16:37:25 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [  270.156668] docker0: port 1(veth9415710) entered forwarding state
Aug 15 16:37:25 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [  270.156693] docker0: port 1(veth9415710) entered forwarding state
Aug 15 16:37:25 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [  270.156716] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 15 16:37:28 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpd[1843]: Listen normally on 5 docker0 fe80::42:b2ff:fe27:f8d3 UDP 123
Aug 15 16:37:28 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpd[1843]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 15 16:37:28 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpd[1843]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 15 16:37:28 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpd[1843]: peers refreshed
Aug 15 16:37:28 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 ntpd[1843]: new interface(s) found: waking up resolver
Aug 15 16:37:40 travis-job-f16ced9d-a02d-4fdb-99ef-75b4d0d0e385 kernel: [  285.196013] docker0: port 1(veth9415710) entered forwarding state
---
travis_time:end:032fb9b0:start=1534351224893117445,finish=1534351224902481024,duration=9363579
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:018c79f1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:21bf0f38
travis_time:start:21bf0f38
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0bb62be6
$ dmesg | grep -i kill
