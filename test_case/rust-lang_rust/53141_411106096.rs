plain
[01:19:49] Verifying status of rust-by-example...
[01:19:49] Verifying status of rls...
[01:19:49] This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
[01:19:49] 
[01:19:49] ⚠️ We detected that this PR updated 'rls', but its tests failed.
[01:19:49] 
[01:19:49] If you do intend to update 'rls', please check the error messages above and
[01:19:49] commit another update.
[01:19:49] 
[01:19:49] If you do NOT intend to update 'rls', please ensure you did not accidentally
[01:19:49] change the submodule at 'src/tools/rls'. You may ask your reviewer for the
[01:19:49] proper steps.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:115eac70
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:35c14ec9
$ sudo tail -n 500 /var/log/syslog
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   4 disabled
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   5 disabled
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   6 disabled
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   7 disabled
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Using GB pages for direct mapping
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] kvm-clock: using sched offset of 1378342162 cycles
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Zone ranges:
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   Device   empty
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Movable zone start for each node
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Early memory node ranges
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Policy zone: Normal
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] console [ttyS0] enabled
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.311757] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.313031] pid_max: default: 32768 minimum: 301
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.313708] ACPI: Core revision 20150930
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.320149] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.321247] Security Framework initialized
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.321824] Yama: becoming mindful.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.322390] AppArmor: AppArmor disabled by boot time parameter
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.324959] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.333723] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.338100] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.339283] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.340777] Initializing cgroup subsys io
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.341369] Initializing cgroup subsys memory
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.342172] Initializing cgroup subsys devices
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.342991] Initializing cgroup subsys freezer
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.343676] Initializing cgroup subsys net_cls
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.344415] Initializing cgroup subsys perf_event
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.345157] Initializing cgroup subsys net_prio
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.346089] Initializing cgroup subsys hugetlb
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.346873] Initializing cgroup subsys pids
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.347759] CPU: Physical Processor ID: 0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.348418] CPU: Processor Core ID: 0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.349815] mce: CPU supports 32 MCE banks
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.350541] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.351587] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.354428] Freeing SMP alternatives memory: 32K
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.362765] ftrace: allocating 32185 entries in 126 pages
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.409400] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.410433] smpboot: Max logical packages: 2
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.411647] x2apic enabled
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.413116] Switched APIC routing to physical x2apic.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.416546] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.523016] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.524841] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.527416] x86: Booting SMP configuration:
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.528094] .... node  #0, CPUs:      #1
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.529048] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.533088]  #2
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.533623] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.538207]  #3
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.538705] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.542938] x86: Booted up 1 node, 4 CPUs
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.543694] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.546008] devtmpfs: initialized
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.550435] evm: security.selinux
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.550972] evm: security.SMACK64
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.551511] evm: security.SMACK64EXEC
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.552262] evm: security.SMACK64TRANSMUTE
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.553576] evm: security.SMACK64MMAP
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.554103] evm: security.ima
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.554635] evm: security.capability
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.555688] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.557422] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.558624] pinctrl core: initialized pinctrl subsystem
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.559585] RTC time: 14:30:20, date: 08/07/18
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.561098] NET: Registered protocol family 16
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.571046] cpuidle: using governor ladder
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.583049] cpuidle: using governor menu
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.583675] PCCT header not found.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.584279] ACPI: bus type PCI registered
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.584838] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.585929] PCI: Using configuration type 1 for base access
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.600027] ACPI: Added _OSI(Module Device)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.600889] ACPI: Added _OSI(Processor Device)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.601648] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.602434] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.605688] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.628677] ACPI: Interpreter enabled
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.629385] ACPI: (supports S0 S3 S4 S5)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.629965] ACPI: Using IOAPIC for interrupt routing
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.630714] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.660622] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.661586] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.662784] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.663757] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.666218] PCI host bridge to bus 0000:00
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.666861] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.667971] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.669047] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.670315] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.671376] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.672204] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.672594] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.686381] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.701638] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.703313] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.708305] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.712526] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.724538] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.729780] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.733936] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.746381] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.749111] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.751355] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.753463] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.756008] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.776272] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.777319] vgaarb: loaded
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.778080] SCSI subsystem initialized
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.778741] libata version 3.00 loaded.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.778762] ACPI: bus type USB registered
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.779404] usbcore: registered new interface driver usbfs
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.780151] usbcore: registered new interface driver hub
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.780994] usbcore: registered new device driver usb
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.781888] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.783054] dmi: Firmware registration failed.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.783973] PCI: Using ACPI for IRQ routing
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.784656] PCI: pci_cache_line_size set to 64 bytes
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.784752] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.784753] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.784871] NetLabel: Initializing
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.785448] NetLabel:  domain hash size = 128
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.786056] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.786885] NetLabel:  unlabeled traffic allowed by default
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.787913] amd_nb: Cannot enumerate AMD northbridges
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.788686] clocksource: Switched to clocksource kvm-clock
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.795694] pnp: PnP ACPI init
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.796266] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.796338] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.796389] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.796444] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.796491] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.796541] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.796587] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.796772] pnp: PnP ACPI: found 7 devices
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.804011] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.805548] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.805550] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.805552] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.805553] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.805582] NET: Registered protocol family 2
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.806388] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.807574] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.808598] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.809654] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.810581] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.812314] NET: Registered protocol family 1
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.813277] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.814247] PCI: CLS 0 bytes, default 64
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    0.814296] Unpacking initramfs...
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.819290] Freeing initrd memory: 21432K
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.820099] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.821174] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.822657] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.824013] hw unit of domain pp0-core 2^-0 Joules
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.824788] hw unit of domain package 2^-0 Joules
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.825654] hw unit of domain dram 2^-16 Joules
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.826508] Scanning for low memory corruption every 60 seconds
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.828233] audit: initializing netlink subsys (disabled)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.829434] audit: type=2000 audit(1533652222.977:1): initialized
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.830648] Initialise system trusted keyring
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.831559] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.832604] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.834809] zbud: loaded
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.835511] VFS: Disk quotas dquot_6.6.0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.836238] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.837443] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.838726] fuse init (API version 7.23)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.840014] Key type big_key registered
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.840786] Allocating IMA MOK and blacklist keyrings.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.842834] Key type asymmetric registered
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.843606] Asymmetric key parser 'x509' registered
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.844341] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.845832] io scheduler noop registered
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.846496] io scheduler deadline registered (default)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.847266] io scheduler cfq registered
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.848009] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.848966] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.850084] intel_idle: does not run on family 6 model 63
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.850177] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.851458] ACPI: Power Button [PWRF]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.852130] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.853377] ACPI: Sleep Button [SLPF]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.854237] GHES: HEST is not enabled!
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.856566] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.857567] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.862416] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.863546] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.868157] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.890677] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.913618] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.936594] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.959649] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.962663] Linux agpgart interface v0.103
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.965590] loop: module loaded
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.966322] libphy: Fixed MDIO Bus: probed
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.967173] tun: Universal TUN/TAP device driver, 1.6
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    2.968170] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.003419] PPP generic driver version 2.4.2
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.004532] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.005561] ehci-pci: EHCI PCI platform driver
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.006268] ehci-platform: EHCI generic platform driver
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.007439] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.008513] ohci-pci: OHCI PCI platform driver
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.009618] ohci-platform: OHCI generic platform driver
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.010700] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.011697] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.014037] i8042: Warning: Keylock active
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.015551] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.016417] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.017644] mousedev: PS/2 mouse device common for all mice
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.018916] rtc_cmos 00:00: RTC can wake from S4
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.019947] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.021381] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.022364] i2c /dev entries driver
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.022876] device-mapper: uevent: version 1.0.3
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.023734] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.025003] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.027096] NET: Registered protocol family 10
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.029071] NET: Registered protocol family 17
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.030088] Key type dns_resolver registered
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.031054] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.032186] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.033352] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.034834] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.035814] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.037406] registered taskstats version 1
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.037865] Loading compiled-in X.509 certificates
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.039359] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.041443] zswap: loaded using pool lzo/zbud
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.044188] Key type trusted registered
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.048055] Key type encrypted registered
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.048710] ima: No TPM chip found, activating TPM-bypass!
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.049614] evm: HMAC attrs: 0x1
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.050430]   Magic number: 14:499:535
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.050887] tty ttyS15: hash matches
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.051675] rtc_cmos 00:00: setting system clock to 2018-08-07 14:30:23 UTC (1533652223)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.053673] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.054659] EDD information not available.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.055648] PM: Hibernation image not present or could not be loaded.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.056971] Freeing unused kernel memory: 1496K
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.057679] Write protecting the kernel read-only data: 14336k
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.059518] Freeing unused kernel memory: 1956K
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.060549] Freeing unused kernel memory: 92K
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.073679] systemd-udevd[119]: starting version 204
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.126832] scsi host0: Virtio SCSI HBA
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.134726] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.138437] AVX2 version of gcm_enc/dec engaged.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.139086] AES CTR mode by8 optimization enabled
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.177973] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.178007] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.178009] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.178172] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.178174] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.178222] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.179465]  sda: sda1
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.180095] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.217245] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.824762] tsc: Refined TSC clocksource calibration: 2300.001 MHz
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    3.825954] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735f0517, max_idle_ns: 440795237604 ns
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    4.054686] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    6.136903] floppy0: no floppy controllers found
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.284731] raid6: sse2x1   gen()  8728 MB/s
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.352704] raid6: sse2x1   xor()  6809 MB/s
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.420710] raid6: sse2x2   gen() 11101 MB/s
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.488708] raid6: sse2x2   xor()  7487 MB/s
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.556712] raid6: sse2x4   gen() 12513 MB/s
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.624704] raid6: sse2x4   xor()  8882 MB/s
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.692706] raid6: avx2x1   gen() 17078 MB/s
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.760707] raid6: avx2x2   gen() 19814 MB/s
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.828703] raid6: avx2x4   gen() 22903 MB/s
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.829356] raid6: using algorithm avx2x4 gen() 22903 MB/s
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.830173] raid6: using avx2x2 recovery algorithm
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.832265] xor: automatically using best checksumming function:
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.872697]    avx       : 27229.000 MB/sec
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.886103] Btrfs loaded
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.923915] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.925047] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.991854] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.998484] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    7.999310] EXT4-fs (sda1): recovery complete
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    8.004002] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    8.221550] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    8.349779] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    8.399540] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    8.605212] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    9.153880] random: cloud-init: uninitialized urandom read (32 bytes read, 47 bits of entropy available)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    9.287372] systemd-udevd[702]: starting version 204
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    9.402112] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    9.476472] intel_rapl: no valid rapl domains found in package 0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    9.529657] ppdev: user-space parallel port driver
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    9.622681] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    9.673596] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    9.741711] random: cloud-init: uninitialized urandom read (32 bytes read, 60 bits of entropy available)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [    9.907209] random: cloud-init: uninitialized urandom read (32 bytes read, 60 bits of entropy available)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   10.172715] random: mktemp: uninitialized urandom read (12 bytes read, 62 bits of entropy available)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   10.245325] random: mktemp: uninitialized urandom read (6 bytes read, 63 bits of entropy available)
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   10.320018] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   10.363335] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   10.630361] init: failsafe main process (1096) killed by TERM signal
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO Running set_multiqueue.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO Set channels for eth0 to 4.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   11.331754] random: nonblocking pool is initialized
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 google-accounts: INFO Starting Google Accounts daemon.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 google-accounts: INFO Creating a new user account for me.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 google-accounts: INFO Created user account me.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 google-accounts: INFO Creating a new user account for bogdana.
Aug  7 14:30:31 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 google-accounts: INFO Created user account bogdana.
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 google-accounts: INFO Creating a new user account for aj.
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 google-accounts: INFO Created user account aj.
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 google-accounts: INFO Creating a new user account for asari.
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 google-accounts: INFO Created user account asari.
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 cron[1424]: (CRON) INFO (pidfile fd = 3)
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 cron[1461]: (CRON) STARTUP (fork ok)
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 cron[1461]: (CRON) INFO (Running @reboot jobs)
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 google-accounts: INFO Removing user packer.
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 acpid: starting up with netlink and the input layer
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 acpid: 1 rule loaded
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 acpid: waiting for events: event logging is off
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 haveged: haveged starting up
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   11.846970] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   11.856254] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   12.031657] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   12.034781] Bridge firewalling registered
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   12.043414] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   12.109020] Initializing XFRM netlink socket
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   12.115244] Netfilter messages via NETLINK v0.30.
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   12.118116] ctnetlink v0.93: registering with nfnetlink.
Aug  7 14:30:32 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   12.488868] floppy0: no floppy controllers found
Aug  7 14:30:55 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpdate[1768]: adjust time server 169.254.169.254 offset 0.004715 sec
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpd[1804]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpd[1805]: proto: precision = 0.100 usec
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpd[1805]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpd[1805]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpd[1805]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpd[1805]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpd[1805]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpd[1805]: Listen normally on 3 eth0 10.20.0.109 UDP 123
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpd[1805]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpd[1805]: peers refreshed
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpd[1805]: Listening on routing socket on fd #21 for interface updates
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   42.029402] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 startup-script: INFO Found startup-script in metadata.
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 startup-script: INFO startup-script: job 1 at Tue Aug  7 17:41:00 2018
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 startup-script: INFO startup-script: Return code 0.
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 startup-script: INFO startup-script: Return code 0.
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 startup-script: INFO Finished running startup scripts.
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ec2: 
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ec2: #############################################################
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ec2: 1024 40:e2:69:8b:94:d8:e7:be:07:36:02:57:da:ec:e5:cc  root@travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 (DSA)
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ec2: 256 cc:cc:7d:ea:9d:26:4b:1e:75:7f:94:d7:42:96:88:de  root@travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 (ECDSA)
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ec2: 256 64:63:57:17:db:7a:5b:08:e4:09:1c:63:78:fe:01:e0  root@travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 (ED25519)
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ec2: 2048 d1:b9:72:7e:7f:4b:54:fb:4e:5b:fb:28:7f:32:69:1d  root@travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 (RSA)
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 14:31:02 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ec2: #############################################################
Aug  7 14:31:33 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [   73.557307] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 14:32:07 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [  107.368093] device veth7b10614 entered promiscuous mode
Aug  7 14:32:07 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [  107.454986] cgroup: docker-runc (4811) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 14:32:07 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [  107.454988] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 14:32:07 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [  107.522683] eth0: renamed from veth8d47ab2
Aug  7 14:32:07 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [  107.561378] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 14:32:07 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [  107.562314] docker0: port 1(veth7b10614) entered forwarding state
Aug  7 14:32:07 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [  107.562330] docker0: port 1(veth7b10614) entered forwarding state
Aug  7 14:32:07 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [  107.562357] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 14:32:11 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpd[1805]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  7 14:32:11 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpd[1805]: Listen normally on 6 docker0 fe80::42:37ff:fe70:56b3 UDP 123
Aug  7 14:32:11 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpd[1805]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 14:32:11 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpd[1805]: peers refreshed
Aug  7 14:32:11 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 ntpd[1805]: new interface(s) found: waking up resolver
Aug  7 14:32:22 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 kernel: [  122.578491] docker0: port 1(veth7b10614) entered forwarding state
Aug  7 15:17:01 travis-job-909b1172-9a10-48c7-925d-9c6f42234b68 CRON[2443]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
---
travis_time:end:0f025b60:start=1533657085942936954,finish=1533657085951221708,duration=8284754
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06aa8a06
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2d476df0
travis_time:start:2d476df0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0041d318
$ dmesg | grep -i kill
