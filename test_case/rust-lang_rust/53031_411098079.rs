plain

[00:03:51] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:51] tidy error: /checkout/src/test/codegen/target-cpu-on-functions.rs:15: line longer than 100 chars
[00:03:52] some tidy checks failed
[00:03:52] 
[00:03:52] 
[00:03:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:52] 
[00:03:52] 
[00:03:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:52] Build completed unsuccessfully in 0:00:49
[00:03:52] Build completed unsuccessfully in 0:00:49
[00:03:52] Makefile:79: recipe for target 'tidy' failed
[00:03:52] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01b36c9f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:09c256be
$ sudo tail -n 500 /var/log/syslog
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27D0 000014 (v00 Google)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] kvm-clock: using sched offset of 1705978919 cycles
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] Zone ranges:
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000]   Device   empty
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] Movable zone start for each node
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] Early memory node ranges
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] Policy zone: Normal
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] console [ttyS0] enabled
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.349423] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.351040] pid_max: default: 32768 minimum: 301
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.352025] ACPI: Core revision 20150930
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.359697] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.361972] Security Framework initialized
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.363025] Yama: becoming mindful.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.363806] AppArmor: AppArmor disabled by boot time parameter
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.366971] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.377342] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.382347] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.383872] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.385612] Initializing cgroup subsys io
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.386349] Initializing cgroup subsys memory
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.387043] Initializing cgroup subsys devices
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.387725] Initializing cgroup subsys freezer
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.388530] Initializing cgroup subsys net_cls
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.389449] Initializing cgroup subsys perf_event
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.390255] Initializing cgroup subsys net_prio
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.391408] Initializing cgroup subsys hugetlb
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.392150] Initializing cgroup subsys pids
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.393045] CPU: Physical Processor ID: 0
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.393792] CPU: Processor Core ID: 0
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.394516] mce: CPU supports 32 MCE banks
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.395471] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.396483] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.399630] Freeing SMP alternatives memory: 32K
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.409014] ftrace: allocating 32185 entries in 126 pages
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.459218] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.460559] smpboot: Max logical packages: 2
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.462084] x2apic enabled
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.463948] Switched APIC routing to physical x2apic.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.468642] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.576671] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.578478] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.582631] x86: Booting SMP configuration:
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.583234] .... node  #0, CPUs:      #1
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.585644] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.589112]  #2
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.589878] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.593446]  #3
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.593864] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.597427] x86: Booted up 1 node, 4 CPUs
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.598307] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.600882] devtmpfs: initialized
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.605504] evm: security.selinux
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.606252] evm: security.SMACK64
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.606861] evm: security.SMACK64EXEC
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.607422] evm: security.SMACK64TRANSMUTE
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.608066] evm: security.SMACK64MMAP
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.608611] evm: security.ima
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.609040] evm: security.capability
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.610002] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.611741] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.613201] pinctrl core: initialized pinctrl subsystem
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.614266] RTC time: 15:15:57, date: 08/07/18
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.615968] NET: Registered protocol family 16
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.628708] cpuidle: using governor ladder
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.640705] cpuidle: using governor menu
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.641948] PCCT header not found.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.642676] ACPI: bus type PCI registered
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.643448] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.645265] PCI: Using configuration type 1 for base access
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.657970] ACPI: Added _OSI(Module Device)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.658822] ACPI: Added _OSI(Processor Device)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.659445] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.660366] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.664011] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.688892] ACPI: Interpreter enabled
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.689490] ACPI: (supports S0 S3 S4 S5)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.690067] ACPI: Using IOAPIC for interrupt routing
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.691156] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.721867] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.723137] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.724195] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.725657] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.728218] PCI host bridge to bus 0000:00
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.729013] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.730973] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.732133] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.733405] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.734854] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.735833] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.736287] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.754458] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.773649] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.776443] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.783422] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.789796] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.806093] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.812336] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.817276] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.837824] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.840822] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.843414] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.846007] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.848751] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.871069] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.872363] vgaarb: loaded
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.873292] SCSI subsystem initialized
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.874166] libata version 3.00 loaded.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.874192] ACPI: bus type USB registered
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.875077] usbcore: registered new interface driver usbfs
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.875999] usbcore: registered new interface driver hub
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.877062] usbcore: registered new device driver usb
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.878362] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.879543] dmi: Firmware registration failed.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.880622] PCI: Using ACPI for IRQ routing
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.881318] PCI: pci_cache_line_size set to 64 bytes
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.881426] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.881428] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.881600] NetLabel: Initializing
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.882986] NetLabel:  domain hash size = 128
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.883910] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.884947] NetLabel:  unlabeled traffic allowed by default
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.886074] amd_nb: Cannot enumerate AMD northbridges
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.886918] clocksource: Switched to clocksource kvm-clock
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.894998] pnp: PnP ACPI init
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.896038] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.896112] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.896156] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.896209] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.896338] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.896387] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.896427] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.896611] pnp: PnP ACPI: found 7 devices
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.904523] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.905973] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.905976] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.905978] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.905980] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.906018] NET: Registered protocol family 2
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.907476] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.909033] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.910418] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.911699] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.912911] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.914949] NET: Registered protocol family 1
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.915764] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.916638] PCI: CLS 0 bytes, default 64
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    0.916707] Unpacking initramfs...
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.009184] Freeing initrd memory: 21432K
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.010290] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.011365] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.013429] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.014896] hw unit of domain pp0-core 2^-0 Joules
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.015875] hw unit of domain package 2^-0 Joules
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.016731] hw unit of domain dram 2^-0 Joules
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.017928] Scanning for low memory corruption every 60 seconds
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.019568] audit: initializing netlink subsys (disabled)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.020856] audit: type=2000 audit(1533654959.255:1): initialized
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.022880] Initialise system trusted keyring
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.024045] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.025342] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.028120] zbud: loaded
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.029220] VFS: Disk quotas dquot_6.6.0
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.031498] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.032946] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.034676] fuse init (API version 7.23)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.035740] Key type big_key registered
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.036365] Allocating IMA MOK and blacklist keyrings.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.039058] Key type asymmetric registered
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.039887] Asymmetric key parser 'x509' registered
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.040932] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.042207] io scheduler noop registered
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.043190] io scheduler deadline registered (default)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.044191] io scheduler cfq registered
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.045032] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.046288] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.048091] intel_idle: does not run on family 6 model 45
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.048211] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.049303] ACPI: Power Button [PWRF]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.050153] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.052150] ACPI: Sleep Button [SLPF]
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.053414] GHES: HEST is not enabled!
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.056260] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.057967] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.063695] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.066003] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.072608] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.095760] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.119974] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.143872] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.168184] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.172476] Linux agpgart interface v0.103
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.176382] loop: module loaded
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.177531] libphy: Fixed MDIO Bus: probed
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.178950] tun: Universal TUN/TAP device driver, 1.6
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.180592] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.217196] PPP generic driver version 2.4.2
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.218798] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.221060] ehci-pci: EHCI PCI platform driver
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.223081] ehci-platform: EHCI generic platform driver
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.224548] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.226365] ohci-pci: OHCI PCI platform driver
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.227949] ohci-platform: OHCI generic platform driver
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.229638] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.231354] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.234545] i8042: Warning: Keylock active
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.236811] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.238492] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.240055] mousedev: PS/2 mouse device common for all mice
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.242012] rtc_cmos 00:00: RTC can wake from S4
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.243777] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.246323] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.247839] i2c /dev entries driver
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.249389] device-mapper: uevent: version 1.0.3
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.251073] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.253647] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.256532] NET: Registered protocol family 10
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.258614] NET: Registered protocol family 17
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.260028] Key type dns_resolver registered
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.261758] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.263433] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.264859] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.266511] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.267830] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.270465] registered taskstats version 1
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.271561] Loading compiled-in X.509 certificates
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.273312] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.276081] zswap: loaded using pool lzo/zbud
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.279159] Key type trusted registered
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.283975] Key type encrypted registered
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.285182] ima: No TPM chip found, activating TPM-bypass!
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.286652] evm: HMAC attrs: 0x1
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.287740]   Magic number: 14:846:284
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.289046] rtc_cmos 00:00: setting system clock to 2018-08-07 15:15:59 UTC (1533654959)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.291218] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.293468] EDD information not available.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.294563] PM: Hibernation image not present or could not be loaded.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.296621] Freeing unused kernel memory: 1496K
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.297558] Write protecting the kernel read-only data: 14336k
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.299718] Freeing unused kernel memory: 1956K
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.300956] Freeing unused kernel memory: 92K
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.316393] systemd-udevd[119]: starting version 204
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.376132] scsi host0: Virtio SCSI HBA
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.380736] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.389369] AVX version of gcm_enc/dec engaged.
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.390526] AES CTR mode by8 optimization enabled
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.429661] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.429721] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.429723] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.430000] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.430003] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.430107] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.431873]  sda: sda1
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.432824] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    3.443393] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    4.015093] tsc: Refined TSC clocksource calibration: 2599.788 MHz
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    4.016181] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2579739c1c1, max_idle_ns: 440795232548 ns
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    4.276118] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    6.363103] floppy0: no floppy controllers found
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    7.526962] raid6: sse2x1   gen()  9143 MB/s
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    7.594952] raid6: sse2x1   xor()  7027 MB/s
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    7.662956] raid6: sse2x2   gen() 11242 MB/s
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    7.730970] raid6: sse2x2   xor()  7920 MB/s
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    7.798955] raid6: sse2x4   gen() 12869 MB/s
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    7.866958] raid6: sse2x4   xor()  9068 MB/s
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    7.867873] raid6: using algorithm sse2x4 gen() 12869 MB/s
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    7.868966] raid6: .... xor() 9068 MB/s, rmw enabled
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    7.870228] raid6: using ssse3x2 recovery algorithm
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    7.872742] xor: automatically using best checksumming function:
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    7.910938]    avx       : 22618.000 MB/sec
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    7.926767] Btrfs loaded
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    7.972764] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    7.974056] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    8.066470] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    8.073901] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    8.074997] EXT4-fs (sda1): recovery complete
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    8.080548] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    8.292506] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    8.403652] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    8.457738] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    8.648811] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    9.190340] random: cloud-init: uninitialized urandom read (32 bytes read, 46 bits of entropy available)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    9.323985] systemd-udevd[702]: starting version 204
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    9.432400] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    9.474858] intel_rapl: no valid rapl domains found in package 0
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    9.532482] ppdev: user-space parallel port driver
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    9.621139] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    9.671781] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    9.738804] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [    9.900888] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   10.139677] random: mktemp: uninitialized urandom read (12 bytes read, 61 bits of entropy available)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   10.207411] random: mktemp: uninitialized urandom read (6 bytes read, 62 bits of entropy available)
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   10.281205] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   10.308315] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 15:16:06 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   10.666977] init: failsafe main process (1096) killed by TERM signal
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO Running set_multiqueue.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO Set channels for eth0 to 4.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   11.416720] random: nonblocking pool is initialized
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Starting Google Accounts daemon.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Creating a new user account for me.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Created user account me.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Creating a new user account for henrik.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Created user account henrik.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Creating a new user account for emma.
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 cron[1407]: (CRON) INFO (pidfile fd = 3)
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 cron[1454]: (CRON) STARTUP (fork ok)
Aug  7 15:16:07 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 cron[1454]: (CRON) INFO (Running @reboot jobs)
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Created user account emma.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Creating a new user account for igor.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 acpid: starting up with netlink and the input layer
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 acpid: 1 rule loaded
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 acpid: waiting for events: event logging is off
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Created user account igor.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 haveged: haveged starting up
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Created user account konstantinhaase.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   11.938875] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Creating a new user account for aj.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   11.957528] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Created user account aj.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   12.012000] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Creating a new user account for solarce.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   12.015345] Bridge firewalling registered
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   12.027257] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Created user account solarce.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Creating a new user account for asari.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   12.105804] Initializing XFRM netlink socket
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   12.112280] Netfilter messages via NETLINK v0.30.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   12.114787] ctnetlink v0.93: registering with nfnetlink.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Created user account asari.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Creating a new user account for bogdana.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Created user account bogdana.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Creating a new user account for konstantin.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Created user account konstantin.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Creating a new user account for carmen.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Created user account carmen.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Creating a new user account for maria.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Created user account maria.
Aug  7 15:16:08 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-accounts: INFO Removing user packer.
Aug  7 15:16:09 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 15:16:09 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   12.491034] floppy0: no floppy controllers found
Aug  7 15:16:31 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpdate[1844]: adjust time server 169.254.169.254 offset 0.005540 sec
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpd[1849]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpd[1850]: proto: precision = 0.100 usec
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpd[1850]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpd[1850]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpd[1850]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpd[1850]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpd[1850]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpd[1850]: Listen normally on 3 eth0 10.20.1.169 UDP 123
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpd[1850]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpd[1850]: peers refreshed
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpd[1850]: Listening on routing socket on fd #21 for interface updates
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   42.127175] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 startup-script: INFO Starting startup scripts.
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 startup-script: INFO Found startup-script in metadata.
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 startup-script: INFO startup-script: job 1 at Tue Aug  7 18:26:00 2018
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 startup-script: INFO startup-script: Return code 0.
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 startup-script: INFO Finished running startup scripts.
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ec2: 
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ec2: #############################################################
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ec2: 1024 f8:8f:9f:32:f5:51:f0:8c:f3:ec:6f:27:42:4e:40:66  root@travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 (DSA)
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ec2: 256 6c:a8:da:03:8b:22:50:93:1d:42:cb:5e:21:ce:38:fd  root@travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 (ECDSA)
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ec2: 256 a8:be:1c:bf:f6:17:cc:9d:04:d8:00:6c:1c:e2:cf:cd  root@travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 (ED25519)
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ec2: 2048 e9:5b:3d:cf:03:23:9c:ca:b6:58:73:f2:67:0e:b3:78  root@travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 (RSA)
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 15:16:38 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ec2: #############################################################
Aug  7 15:17:02 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 CRON[3425]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  7 15:17:10 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [   73.840283] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 15:18:15 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [  138.688117] device veth73291aa entered promiscuous mode
Aug  7 15:18:15 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [  138.768005] cgroup: docker-runc (4837) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 15:18:15 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [  138.768008] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 15:18:15 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [  138.831547] eth0: renamed from veth4019b0e
Aug  7 15:18:15 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [  138.867484] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 15:18:15 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [  138.868632] docker0: port 1(veth73291aa) entered forwarding state
Aug  7 15:18:15 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [  138.868650] docker0: port 1(veth73291aa) entered forwarding state
Aug  7 15:18:15 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [  138.868675] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 15:18:18 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpd[1850]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  7 15:18:18 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpd[1850]: Listen normally on 6 docker0 fe80::42:6bff:fefc:59d UDP 123
Aug  7 15:18:18 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpd[1850]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 15:18:18 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpd[1850]: peers refreshed
Aug  7 15:18:18 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 ntpd[1850]: new interface(s) found: waking up resolver
Aug  7 15:18:30 travis-job-f263d559-110f-4b18-bd8f-7553c8ac0de0 kernel: [  153.923389] docker0: port 1(veth73291aa) entered forwarding state
---
travis_time:end:1fb63d2f:start=1533655264554542919,finish=1533655264560660048,duration=6117129
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0cdcdab8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:099c7e5e
travis_time:start:099c7e5e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:14210c00
$ dmesg | grep -i kill
