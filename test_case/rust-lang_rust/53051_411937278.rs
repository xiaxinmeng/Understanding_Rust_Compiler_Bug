plain
[00:48:00] ....................................................................................................
[00:48:02] ....................................................................................................
[00:48:05] ....................................................................................................
[00:48:08] ....................................................................................................
[00:48:10] ...............iiiiiiiii............................................................................
[00:48:16] ....................................................................................................
[00:48:19] .....................i..............................................................................
[00:48:22] ...............................i....................................................................
[00:48:25] ....................................................................................................
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:1484c676
$ sudo tail -n 500 /var/log/syslog
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] kvm-clock: using sched offset of 1434777784 cycles
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] Zone ranges:
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000]   Device   empty
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] Movable zone start for each node
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] Early memory node ranges
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7 9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] console [ttyS0] enabled
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.320110] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.321602] pid_max: default: 32768 minimum: 301
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.322397] ACPI: Core revision 20150930
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.328685] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.330038] Security Framework initialized
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.330915] Yama: becoming mindful.
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.331623] AppArmor: AppArmor disabled by boot time parameter
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.334236] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.343627] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.348209] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.349526] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.351038] Initializing cgroup subsys io
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.351851] Initializing cgroup subsys memory
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.352872] Initializing cgroup subsys devices
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.353581] Initializing cgroup subsys freezer
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.354227] Initializing cgroup subsys net_cls
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.354837] Initializing cgroup subsys perf_event
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.355630] Initializing cgroup subsys net_prio
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.356534] Initializing cgroup subsys hugetlb
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.357200] Initializing cgroup subsys pids
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-0 [bus 00-ff])
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.674382] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.675631] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.676894] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.679221] PCI host bridge to bus 0000:00
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.680040] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.681079] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.682201] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.683388] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.684590] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.685510] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.685923] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.705793] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.730519] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.732258] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.739036] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.744274] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.759152] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.764617] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.769305] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.782061] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.784372] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.786395] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.788866] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.791114] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.811014] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.812277] vgaarb: loaded
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.813148] SCSI subsystem initialized
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.813965] libata version 3.00 loaded.
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.813991] ACPI: bus type USB registered
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.814688] usbcore: registered new interface driver usbfs
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.815565] usbcore: registered new interface driver hub
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.816385] usbcore: registered new device driver usb
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.817351] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.818293] dmi: Firmware registration failed.
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.819186] PCI: Using ACPI for IRQ routing
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.819902] PCI: pci_cache_l3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.835193] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.835235] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.835276] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.835317] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.835476] pnp: PnP ACPI: found 7 devices
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.842717] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.844560] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.844563] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.844565] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.844566] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.844674] NET: Registered protocol family 2
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.845541] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.847679] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.849115] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.850761] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.851849] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.853811] NET: Registered protocol family 1
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.854603] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.855536] PCI: CLS 0 bytes, default 64
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    0.855589] Unpacking initramfs...
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.834878] Freeing initrd memory: 21432K
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.836104] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.837062] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.838575] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.839880] hw unit of domain pp0-core 2^-0 Joules
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.840587] hw unit of domain package 2^-0 Joules
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.841289] hw unit of domain dram 2^-0 Joules
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.842116] Scanning for low memory corruption every 60 seconds
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.843649] audit: initializing netlink subsys (disabled)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.844812] audit: type=2000 audit(1533855539.672:1): initialized
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.846131] Initialise system trusted keyring
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.847188] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.848281] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.850472] zbud: loaded
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.851196] VFS: Disk quotas dquot_6.6.0
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.851851] VFS: Dquot-cache hash table entriesogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.865400] intel_idle: does not run on family 6 model 45
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.865494] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.866738] ACPI: Power Button [PWRF]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.867336] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.868329] ACPI: Sleep Button [SLPF]
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.869167] GHES: HEST is not enabled!
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.871336] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.872344] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.877075] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.878392] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.883102] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.905595] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.928706] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.953187] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.976099] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.980115] Linux agpgart interface v0.103
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.983698] loop: module loaded
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.985673] libphy: Fixed MDIO Bus: probed
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.986530] tun: Universal TUN/TAP device driver, 1.6
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    2.987577] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.020805] PPP generic driver version 2.4.2
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.022046] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.023657] ehci-pci: EHCI PCI platform driver
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.024769] ehci-platform: EHCI generic platform driver
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.025848] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.027111] ohci-pci: OHCI PCI platform driver
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.028641] ohci-platform: OHCI generic platform driver
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.029854] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.031428] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.034086] i8042: Warning: Keylock active
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.036200] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.037308] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.038391] mousedev: PS/2 mouse device common for all mice
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.039917] rtc_cmos 00:00: RTC can wake from S4
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.041151] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.042524] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.043817] i2c /dev entries driver
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.044776] device-mapper: uevent: version 1.0.3
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.045857] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.047860] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.049500] NET: Registered protocol family 10
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.050558] NET: Registered protocol family 17
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.051551] Key type dns_resolver registered
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.052756] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.053862] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.055425] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.056827] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.058449] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.060836] registered taskstats version 1
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.061956] Loading compiled-in X.509 certificates
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.063848] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.065840] zswap: loaded using pool lzo/zbud
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.069026] Key type trusted registered
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.073120] Key type encrypted registered
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.073775] ima: No TPM chip found, activating TPM-bypass!
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.075023] evm: HMAC attrs: 0x1
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.076023]   Magic number: 14:465:1007
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.077095] memory memory46: hash matches
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.078278] rtc_cmos 00:00: setting system clock to 2018-08-09 22:59:00 UTC (1533855540)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    3.080668] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 22:59:07 travis-job--47e3-b4cc-a7cc76fcfa3a kernel: [    7.773897] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    7.845061] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    7.852332] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    7.853375] EXT4-fs (sda1): recovery complete
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    7.858106] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    8.096879] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    8.208508] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    8.261727] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    8.448170] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    8.947292] random: cloud-init: uninitialized urandom read (32 bytes read, 46 bits of entropy available)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    9.069257] systemd-udevd[702]: starting version 204
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    9.177213] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    9.248835] intel_rapl: no valid rapl domains found in package 0
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    9.279322] intel_rapl: no valid rapl domains found in package 0
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    9.327625] ppdev: user-space parallel port driver
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    9.407529] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    9.455436] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    9.523246] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    9.686054] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [    9.946661] random: mktemp: uninitialized urandom read (12 bytes read, 61 bits of entropy available)
Aug  9 22:59:07 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [   10.018059] random: mktemp: uninitialized urandom read (6 bytes read, 62 bits of entropy available)
Aug  9 22:59:07 :59:08 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 22:59:08 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 22:59:08 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 22:59:08 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  9 22:59:08 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  9 22:59:08 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  9 22:59:08 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  9 22:59:08 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  9 22:59:08 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  9 22:59:08 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  9 22:59:08 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  9 22:59:08 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a instance-setup: INFO Queue 2 XPS=4 for
Aug  9 22:59:08 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a google-accounts: INFO Created user account asari.
Aug  9 22:59:08 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a google-accounts: INFO Removing user packer.
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a cron[1435]: (CRON) INFO (pidfile fd = 3)
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a cron[1471]: (CRON) STARTUP (fork ok)
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a cron[1471]: (CRON) INFO (Running @reboot jobs)
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a acpid: starting up with netlink and the input layer
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a acpid: 1 rule loaded
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a acpid: waiting for events: event logging is off
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a haveged: haveged starting up
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [   11.853289] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [   11.861856] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [   11.866340] Bridge firewalling registered
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [   11.867173] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [   11.878034] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [   11.943228] Initializing XFRM netlink socket
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [   11.952566] Netfilter messages via NETLINK v0.30.
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [   11.955538] ctnetlink v0.93: registering with nfnetlink.
Aug  9 22:59:09 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [   12.246825] floppy0: no floppy controllers found
Aug  9 22:59:32 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a ntpdate[1786]: adjust time server 169.254.169.254 offset 0.001716 sec
Aug  9 22:59:38 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a ntpd[1822]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 22:59:38 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a ntpd[1823]: proto: precision = 0.142 usec
Aug  9 22:59:38 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a ntpd[1823]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 22:59:38 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a ntpd[1823]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 22:59:38 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a ntpd[1823]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 22:59:38 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a ntpd[1823]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 22:59:38 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a ntpd[1823]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 22:59:38 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a ntpd[1823]: Listen normally on 3 eth0 10.20.0.163 UDP 123
Aug  9 22:59:38 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a ntpd[1823]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 22:59:38 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a ntpd[1823]: peers refreshed
Aug  9 22:59:38 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a ntpd[1823]: Listening on routing socket on fd #21 for interface updates
Aug  9 22:59:39 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a kernel: [   42.026195] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 22:59:39 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a startup-script: INFO Found startup-script in metadata.
Aug  9 22:59:39 travis-job-2d8fbba3-12e8-47e3-b4cc-a7cc76fcfa3a startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  9 22:59:39 travis-job-2d8fbba3-12eravis_fold:start:after_failure.2
travis_time:start:0e90056e
