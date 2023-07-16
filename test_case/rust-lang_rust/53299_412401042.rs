plain
[00:59:00] ....................................................................................................
[00:59:02] ....................................................................................................
[00:59:05] ....................................................................................................
[00:59:08] ....................................................................................................
[00:59:11] ...............iiiiiiiii............................................................................
[00:59:17] ....................................................................................................
[00:59:21] .....................i..............................................................................
[00:59:24] ................................i...................................................................
[00:59:27] ....................................................................................................
---
[01:16:03] running 2121 tests
[01:16:11] ....................................................................................................
[01:16:20] ....................................................................................................
[01:16:30] ....................................................................................................
[01:16:39] ...................................i.......................................................F........
[01:16:54] ....................................................................................................
[01:17:02] ....................................................................................................
[01:17:10] ....................................................................................................
[01:17:17] ....................................................................................................
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:005d8782
$ sudo tail -n 500 /var/log/syslog
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] Using GB pages for direct mapping
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-aOOG 00000001)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] kvm-ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] Policy zone: Normal
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
0060] Security Framework initialized
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.380645] Yama: becoming mindful.
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.381215] AppArmor: AppArmor disabled by boot time parameter
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.383883] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.395257] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.400400] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.401810] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.403241] Initializing cgroup subsys io
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.403826] Initializing cgroup subsys memory
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.404603] Initializing cgroup subsys devices
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.405425] Initializing cgroup subsys freezer
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.406303] Initializing cgroup subsys net_cls
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.406939] Initializing cgroup subsys perf_event
Aug 13 02:bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.718213] ACPI: Using IOAPIC for interrupt routing
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.719091] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.748860] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.750012] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.751168] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.752277] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.754714] PCI host bridge to bus 0000:00
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.755870] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.757097] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.758785] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.759999] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window] Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.879535] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.882242] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.884836] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.887934] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.909092] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.910384] vgaarb: loaded
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.911039] SCSI subsystem initialized
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.911836] libata version 3.00 loaded.
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.911873] ACPI: bus type USB registered
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.912595] usbcore: registered new interface driver usbfs
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.913664] usbcore: registered new interface driver hub
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.914735] usbcore: registered new device driver usb
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.915815] ioremap er   0.931753] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.931830] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.931873] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.931920] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.931959] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.931997] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.932034] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.932196] pnp: PnP ACPI: found 7 devices
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.939876] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.941766] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.941768] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    0.941769] pci_bus 0000:00: resource 6 [mem00:04.0: virtio_pci: leaving for legacy driver
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.118545] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.141166] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.165325] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.189026] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.212802] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.216604] Linux agpgart interface v0.103
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.220691] loop: module loaded
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.222143] libphy: Fixed MDIO Bus: probed
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.223564] tun: Universal TUN/TAP device driver, 1.6
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.224887] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.276178] PPP generic driver version 2.4.2
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.277679] ehci_hcd:31e-a479-4f2457a10e2c kernel: [    3.324201] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.325819] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.328393] registered taskstats version 1
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.329457] Loading compiled-in X.509 certificates
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.331721] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.334539] zswap: loaded using pool lzo/zbud
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.338121] Key type trusted registered
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.342866] Key type encrypted registered
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.343979] ima: No TPM chip found, activating TPM-bypass!
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.345203] evm: HMAC attrs: 0x1
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.346606]   Magic number: 14:468:308
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.348483] tty tty37: hash matches
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.349741] rtc_cmos 00:00: setting system clock to 2018-08-13 02:18:14 UTC (1534126694)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.352055] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.353763] EDD information not available.
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.355257] PM: Hibernation image not present or could not be loaded.
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.357184] Freeing unused kernel memory: 1496K
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.358242] Write protecting the kernel read-only data: 14336k
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.361609] Freeing unused kernel memory: 1956K
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.362873] Freeing unused kernel memory: 92K
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.377684] systemd-udevd[118]: starting version 204
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.444732] scsi host0: Virtio SCSI HBA
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.450970] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.458088] AVX version of gcm_enc/dec engaged.
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.459510] AES CTR mode by8 optimization enabled
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.497511] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.497512] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.497514] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.497703] sd 0:0:1:0: [sda] Write Protect is off
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.497705] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.497756] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.499455]  sda: sda1
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.500286] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    3.508745] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    4.070964] tsc: Refined TSC clocksource calibration: 2499.774 MHz
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    4.072703] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24086447509, max_idle_ns: 440795232708 ns
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    4.345064] input: ImExPS/2 Generi8] Btrfs loaded
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    8.122512] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    8.126831] EXT4-fs (sda1): write access will be enabled during recovery
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    8.218893] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    8.234266] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    8.236434] EXT4-fs (sda1): recovery complete
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    8.244468] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    8.514805] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    8.665947] random: mountall: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    8.730714] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    8.970271] random: cloud-init: uninitialized urandom read (32 bytes read, 33 bits of entropy available)
Aug 13 02:18:22 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [    9.680934] random: cloud-init: uninitialized urandomc startup-script: INFO startup-script: job 1 at Mon Aug 13 05:28:00 2018
Aug 13 02:18:30 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c startup-script: INFO Finished running startup scripts.
Aug 13 02:18:30 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c ec2: 
Aug 13 02:18:30 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c ec2: 
Aug 13 02:18:30 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c ec2: #############################################################
Aug 13 02:18:30 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 13 02:18:30 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c ec2: 1024 9c:5b:5d:c8:4d:4e:d4:63:f0:15:53:33:6c:7b:01:23  root@travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c (DSA)
Aug 13 02:18:30 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c ec2: 256 fd:42:23:2a:e2:09:08:df:06:4d:d3:ec:68:34:9f:0b  root@travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c (ECDSA)
Aug 13 02:18:30 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c ec2: 256 f5:bf:9d:cf:d4:30:10:87:b8:d0:52:fd:0a:16:fc:52  root@travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c (ED25519)
Aug 13 02:18:30 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c ec2: 2048 85:64:a6:9f:d8:6a:1e:19:c6:e8:4c:7f:30:96:e2:10  root@travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c (RSA)
Aug 13 02:18:30 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 13 02:18:30 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c ec2: #############################################################
Aug 13 02:18:38 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c ntpdate[1858]: the NTP socket is in use, exiting
Aug 13 02:21:20 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [  189.277836] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 13 02:27:36 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [  566.014707] device veth36fa7fd entered promiscuous mode
Aug 13 02:27:36 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [  566.014776] docker0: port 1(veth36fa7fd) entered forwarding state
Aug 13 02:27:36 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [  566.014783] docker0: port 1(veth36fa7fd) entered forwarding state
Aug 13 02:27:36 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [  566.015269] docker0: port 1(veth36fa7fd) entered disabled state
Aug 13 02:27:36 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [  566.105955] cgroup: docker-runc (4792) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 13 02:27:36 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [  566.105959] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 13 02:27:36 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [  566.189530] eth0: renamed from vethb1fafab
Aug 13 02:27:37 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [  566.234368] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 13 02:27:37 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [  566.235881] docker0: port 1(veth36fa7fd) entered forwarding state
Aug 13 02:27:37 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [  566.235899] docker0: port 1(veth36fa7fd) entered forwarding state
Aug 13 02:27:37 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [  566.235918] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 13 02:27:39 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c ntpd[1764]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 13 02:27:39 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c ntpd[1764]: Listen normally on 6 docker0 fe80::42:c6ff:fed2:f286 UDP 123
Aug 13 02:27:39 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c ntpd[1764]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 13 02:27:39 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c ntpd[1764]: peers refreshed
Aug 13 02:27:39 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c ntpd[1764]: new interface(s) found: waking up resolver
Aug 13 02:27:52 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [  581.265147] docker0: port 1(veth36fa7fd) entered forwarding state
Aug 13 03:17:01 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c CRON[21596]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 13 03:20:59 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [ 3768.829046] traps: a[5207] trap invalid opcode ip:558c6dab6a9b sp:7fff90d52bd0 error:0 in a[558c6dab3000+6000]
Aug 13 03:21:15 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [ 3784.283325] traps: a[8054] trap invalid opcode ip:7fbf05355691 sp:7ffddd7a0df0 error:0 in libstd-2339b911e3c09de8.so[7fbf052f5000+16f000]
Aug 13 03:21:15 travis-job-01b43bce-0dec-431e-a479-4f2457a10e2c kernel: [ 3784.314187] traps: a[8055] trap invalid opcode ip:7ff800ee5691 sp:7fffab306a30 errore:start:047008b0
