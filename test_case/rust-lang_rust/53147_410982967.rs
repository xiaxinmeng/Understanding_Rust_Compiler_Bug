plain

[00:04:22] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:22] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:62: TODO is deprecated; use FIXME
[00:04:22] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:90: TODO is deprecated; use FIXME
[00:04:22] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:98: TODO is deprecated; use FIXME
[00:04:22] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:121: TODO is deprecated; use FIXME
[00:04:22] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:209: TODO is deprecated; use FIXME
[00:04:22] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:244: TODO is deprecated; use FIXME
[00:04:24] some tidy checks failed
[00:04:24] 
[00:04:24] 
[00:04:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:24] 
[00:04:24] 
[00:04:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:24] Build completed unsuccessfully in 0:00:51
[00:04:24] Build completed unsuccessfully in 0:00:51
[00:04:24] Makefile:79: recipe for target 'tidy' failed
[00:04:24] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03a5f1c2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:065e1f3c
$ sudo tail -n 500 /var/log/syslog
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: RSDP 0x00000000000F27D0 000014 (v00 Google)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] kvm-clock: using sched offset of 1380019825 cycles
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] Zone ranges:
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000]   Device   empty
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] Movable zone start for each node
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] Early memory node ranges
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] Policy zone: Normal
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] console [ttyS0] enabled
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.329054] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.330715] pid_max: default: 32768 minimum: 301
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.331916] ACPI: Core revision 20150930
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.338712] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.340063] Security Framework initialized
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.340896] Yama: becoming mindful.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.341544] AppArmor: AppArmor disabled by boot time parameter
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.344100] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.353805] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.358407] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.359790] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.361235] Initializing cgroup subsys io
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.362176] Initializing cgroup subsys memory
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.362967] Initializing cgroup subsys devices
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.363776] Initializing cgroup subsys freezer
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.364628] Initializing cgroup subsys net_cls
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.365426] Initializing cgroup subsys perf_event
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.366136] Initializing cgroup subsys net_prio
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.367339] Initializing cgroup subsys hugetlb
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.368259] Initializing cgroup subsys pids
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.369028] CPU: Physical Processor ID: 0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.369641] CPU: Processor Core ID: 0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.370275] mce: CPU supports 32 MCE banks
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.371136] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.371910] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.374994] Freeing SMP alternatives memory: 32K
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.383561] ftrace: allocating 32185 entries in 126 pages
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.431043] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.432601] smpboot: Max logical packages: 2
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.433893] x2apic enabled
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.435697] Switched APIC routing to physical x2apic.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.439411] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.548235] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.550198] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.553251] x86: Booting SMP configuration:
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.554085] .... node  #0, CPUs:      #1
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.554968] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.558243]  #2
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.558743] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.562779]  #3
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.563204] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.566502] x86: Booted up 1 node, 4 CPUs
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.567116] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.569307] devtmpfs: initialized
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.573464] evm: security.selinux
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.574056] evm: security.SMACK64
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.574605] evm: security.SMACK64EXEC
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.575203] evm: security.SMACK64TRANSMUTE
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.575986] evm: security.SMACK64MMAP
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.576533] evm: security.ima
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.576987] evm: security.capability
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.577799] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.579353] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.580749] pinctrl core: initialized pinctrl subsystem
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.581658] RTC time:  8:33:48, date: 08/07/18
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.583296] NET: Registered protocol family 16
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.596276] cpuidle: using governor ladder
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.608274] cpuidle: using governor menu
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.609109] PCCT header not found.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.609812] ACPI: bus type PCI registered
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.610383] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.611531] PCI: Using configuration type 1 for base access
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.625271] ACPI: Added _OSI(Module Device)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.626244] ACPI: Added _OSI(Processor Device)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.627144] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.628319] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.631728] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.655092] ACPI: Interpreter enabled
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.655831] ACPI: (supports S0 S3 S4 S5)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.656422] ACPI: Using IOAPIC for interrupt routing
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.657246] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.686616] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.687549] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.688546] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.689557] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.692402] PCI host bridge to bus 0000:00
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.693010] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.694214] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.695224] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.696410] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.697617] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.698567] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.698994] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.713503] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.728960] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.730545] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.736190] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.740712] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.754145] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.759394] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.763503] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.776085] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.779088] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.782104] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.784622] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.786973] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.807253] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.808433] vgaarb: loaded
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.809232] SCSI subsystem initialized
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.810446] libata version 3.00 loaded.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.810476] ACPI: bus type USB registered
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.811272] usbcore: registered new interface driver usbfs
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.812356] usbcore: registered new interface driver hub
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.813370] usbcore: registered new device driver usb
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.814930] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.816113] dmi: Firmware registration failed.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.817257] PCI: Using ACPI for IRQ routing
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.818081] PCI: pci_cache_line_size set to 64 bytes
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.818181] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.818183] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.818321] NetLabel: Initializing
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.819127] NetLabel:  domain hash size = 128
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.820021] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.821075] NetLabel:  unlabeled traffic allowed by default
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.822454] amd_nb: Cannot enumerate AMD northbridges
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.823472] clocksource: Switched to clocksource kvm-clock
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.831217] pnp: PnP ACPI init
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.831951] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.832020] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.832065] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.832115] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.832157] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.832218] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.832262] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.832421] pnp: PnP ACPI: found 7 devices
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.840318] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.841886] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.841889] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.841891] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.841892] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.841929] NET: Registered protocol family 2
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.843004] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.844537] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.845819] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.847002] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.848429] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.849482] NET: Registered protocol family 1
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.850142] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.851326] PCI: CLS 0 bytes, default 64
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    0.851381] Unpacking initramfs...
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.861617] Freeing initrd memory: 21432K
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.862453] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.863886] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.866524] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.869149] hw unit of domain pp0-core 2^-0 Joules
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.870211] hw unit of domain package 2^-0 Joules
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.871050] hw unit of domain dram 2^-0 Joules
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.871891] Scanning for low memory corruption every 60 seconds
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.873341] audit: initializing netlink subsys (disabled)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.874484] audit: type=2000 audit(1533630830.177:1): initialized
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.875898] Initialise system trusted keyring
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.877027] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.878120] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.880352] zbud: loaded
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.881086] VFS: Disk quotas dquot_6.6.0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.881775] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.883504] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.884947] fuse init (API version 7.23)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.885735] Key type big_key registered
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.886627] Allocating IMA MOK and blacklist keyrings.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.888991] Key type asymmetric registered
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.889747] Asymmetric key parser 'x509' registered
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.890829] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.892244] io scheduler noop registered
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.893002] io scheduler deadline registered (default)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.894425] io scheduler cfq registered
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.895145] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.896064] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.897262] intel_idle: does not run on family 6 model 45
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.897374] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.898959] ACPI: Power Button [PWRF]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.899649] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.900762] ACPI: Sleep Button [SLPF]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.901884] GHES: HEST is not enabled!
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.905946] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.908471] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.913282] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.914703] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.920151] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.942861] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.966730] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    2.990614] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.013977] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.018362] Linux agpgart interface v0.103
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.022036] loop: module loaded
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.023047] libphy: Fixed MDIO Bus: probed
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.023880] tun: Universal TUN/TAP device driver, 1.6
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.025252] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.058490] PPP generic driver version 2.4.2
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.059951] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.061524] ehci-pci: EHCI PCI platform driver
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.062376] ehci-platform: EHCI generic platform driver
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.063460] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.064691] ohci-pci: OHCI PCI platform driver
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.065981] ohci-platform: OHCI generic platform driver
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.067288] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.068977] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.071795] i8042: Warning: Keylock active
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.073655] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.074792] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.076058] mousedev: PS/2 mouse device common for all mice
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.077638] rtc_cmos 00:00: RTC can wake from S4
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.079048] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.080459] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.081891] i2c /dev entries driver
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.082730] device-mapper: uevent: version 1.0.3
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.083839] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.085454] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.087884] NET: Registered protocol family 10
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.089126] NET: Registered protocol family 17
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.090340] Key type dns_resolver registered
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.091500] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.094182] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.097059] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.099147] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.100985] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.103002] registered taskstats version 1
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.104278] Loading compiled-in X.509 certificates
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.106059] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.108406] zswap: loaded using pool lzo/zbud
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.111531] Key type trusted registered
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.116117] Key type encrypted registered
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.117424] ima: No TPM chip found, activating TPM-bypass!
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.118929] evm: HMAC attrs: 0x1
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.120124]   Magic number: 14:413:573
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.121506] rtc_cmos 00:00: setting system clock to 2018-08-07 08:33:50 UTC (1533630830)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.124420] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.126329] EDD information not available.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.127744] PM: Hibernation image not present or could not be loaded.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.129147] Freeing unused kernel memory: 1496K
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.129836] Write protecting the kernel read-only data: 14336k
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.131970] Freeing unused kernel memory: 1956K
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.132963] Freeing unused kernel memory: 92K
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.147380] systemd-udevd[119]: starting version 204
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.204516] scsi host0: Virtio SCSI HBA
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.209450] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.215555] AVX version of gcm_enc/dec engaged.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.217451] AES CTR mode by8 optimization enabled
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.253915] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.253932] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.253934] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.254219] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.254221] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.254271] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.256930]  sda: sda1
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.257923] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.279778] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.871628] tsc: Refined TSC clocksource calibration: 2599.999 MHz
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    3.872788] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3b2ad7e, max_idle_ns: 440795282337 ns
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    4.112618] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    6.211674] floppy0: no floppy controllers found
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.359490] raid6: sse2x1   gen()  8966 MB/s
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.427491] raid6: sse2x1   xor()  6718 MB/s
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.495490] raid6: sse2x2   gen() 11016 MB/s
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.563489] raid6: sse2x2   xor()  7363 MB/s
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.631500] raid6: sse2x4   gen() 12780 MB/s
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.699527] raid6: sse2x4   xor()  8996 MB/s
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.700661] raid6: using algorithm sse2x4 gen() 12780 MB/s
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.701427] raid6: .... xor() 8996 MB/s, rmw enabled
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.702445] raid6: using ssse3x2 recovery algorithm
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.704837] xor: automatically using best checksumming function:
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.743486]    avx       : 27597.000 MB/sec
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.757844] Btrfs loaded
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.804897] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.806679] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.875853] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.884817] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.886061] EXT4-fs (sda1): recovery complete
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    7.892266] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    8.107080] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    8.214877] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    8.264185] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    8.573494] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    9.113327] random: cloud-init: uninitialized urandom read (32 bytes read, 44 bits of entropy available)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    9.241764] systemd-udevd[701]: starting version 204
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    9.345098] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    9.426840] intel_rapl: no valid rapl domains found in package 0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    9.477877] intel_rapl: no valid rapl domains found in package 0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    9.510414] ppdev: user-space parallel port driver
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    9.567874] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    9.614217] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    9.685306] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [    9.849567] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   10.113564] random: mktemp: uninitialized urandom read (12 bytes read, 59 bits of entropy available)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   10.184112] random: mktemp: uninitialized urandom read (6 bytes read, 60 bits of entropy available)
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   10.256317] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   10.294284] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   10.836047] init: failsafe main process (1093) killed by TERM signal
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO Running set_multiqueue.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO Set channels for eth0 to 4.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Starting Google Accounts daemon.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Creating a new user account for me.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   11.576322] random: nonblocking pool is initialized
Aug  7 08:33:58 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Created user account me.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Creating a new user account for henrik.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Created user account henrik.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Creating a new user account for emma.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Created user account emma.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Creating a new user account for igor.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Created user account igor.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Created user account konstantinhaase.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Creating a new user account for aj.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Created user account aj.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Creating a new user account for solarce.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc cron[1443]: (CRON) INFO (pidfile fd = 3)
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc cron[1489]: (CRON) STARTUP (fork ok)
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc cron[1489]: (CRON) INFO (Running @reboot jobs)
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Created user account solarce.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc acpid: starting up with netlink and the input layer
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc acpid: 1 rule loaded
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc acpid: waiting for events: event logging is off
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Creating a new user account for asari.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Created user account asari.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc haveged: haveged starting up
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Creating a new user account for bogdana.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   12.098666] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Created user account bogdana.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   12.113375] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Creating a new user account for konstantin.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Created user account konstantin.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Creating a new user account for carmen.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Created user account carmen.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   12.231271] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   12.237399] Bridge firewalling registered
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Creating a new user account for maria.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   12.253112] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Created user account maria.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-accounts: INFO Removing user packer.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   12.322100] Initializing XFRM netlink socket
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   12.329695] Netfilter messages via NETLINK v0.30.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   12.332764] ctnetlink v0.93: registering with nfnetlink.
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   12.419581] floppy0: no floppy controllers found
Aug  7 08:33:59 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   12.419739] work still pending
Aug  7 08:34:00 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 08:34:22 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpdate[1843]: adjust time server 169.254.169.254 offset 0.005315 sec
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpd[1878]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpd[1879]: proto: precision = 0.133 usec
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpd[1879]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpd[1879]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpd[1879]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpd[1879]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpd[1879]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpd[1879]: Listen normally on 3 eth0 10.20.0.250 UDP 123
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpd[1879]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpd[1879]: peers refreshed
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpd[1879]: Listening on routing socket on fd #21 for interface updates
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [   42.282689] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc startup-script: INFO Starting startup scripts.
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc startup-script: INFO Found startup-script in metadata.
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc startup-script: INFO startup-script: job 1 at Tue Aug  7 11:44:00 2018
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc startup-script: INFO startup-script: Return code 0.
Aug  7 08:34:29 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc startup-script: INFO Finished running startup scripts.
Aug  7 08:34:30 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ec2: 
Aug  7 08:34:30 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ec2: #############################################################
Aug  7 08:34:30 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 08:34:30 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ec2: 1024 7b:47:b3:0e:73:47:20:21:6e:b5:78:0d:d9:5b:96:1f  root@travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc (DSA)
Aug  7 08:34:30 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ec2: 256 c4:4b:83:2a:34:39:52:dd:f3:e2:0b:f7:44:8c:a8:28  root@travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc (ECDSA)
Aug  7 08:34:30 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ec2: 256 91:cc:bd:dd:8b:f6:5d:62:87:a9:66:c4:72:6a:da:e1  root@travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc (ED25519)
Aug  7 08:34:30 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ec2: 2048 22:90:ce:ab:45:0a:0a:09:e4:bf:53:72:de:96:79:c2  root@travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc (RSA)
Aug  7 08:34:30 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 08:34:30 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ec2: #############################################################
Aug  7 08:35:49 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [  122.248801] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 08:36:54 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [  186.823412] device veth2e4ad41 entered promiscuous mode
Aug  7 08:36:54 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [  186.917929] cgroup: docker-runc (4871) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 08:36:54 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [  186.917932] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 08:36:54 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [  186.981422] eth0: renamed from vethc905242
Aug  7 08:36:54 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [  187.016082] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 08:36:54 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [  187.017221] docker0: port 1(veth2e4ad41) entered forwarding state
Aug  7 08:36:54 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [  187.017237] docker0: port 1(veth2e4ad41) entered forwarding state
Aug  7 08:36:54 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [  187.017264] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 08:36:57 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpd[1879]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  7 08:36:57 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpd[1879]: Listen normally on 6 docker0 fe80::42:d6ff:fe9f:2c5b UDP 123
Aug  7 08:36:57 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpd[1879]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 08:36:57 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpd[1879]: peers refreshed
Aug  7 08:36:57 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc ntpd[1879]: new interface(s) found: waking up resolver
Aug  7 08:37:09 travis-job-89b484da-e43d-45d7-b649-d01fefa3c5cc kernel: [  202.029135] docker0: port 1(veth2e4ad41) entered forwarding state
---
travis_time:end:130465a4:start=1533631215502012093,finish=1533631215507302762,duration=5290669
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c34195d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12ecf620
travis_time:start:12ecf620
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:10979482
$ dmesg | grep -i kill
