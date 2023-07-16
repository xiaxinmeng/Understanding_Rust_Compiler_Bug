plain
[00:51:48] ....................................................................................................
[00:51:50] ....................................................................................................
[00:51:53] ....................................................................................................
[00:51:56] ....................................................................................................
[00:51:59] ..............iiiiiiiii.............................................................................
[00:52:05] ....................................................................................................
[00:52:08] ....................i...............................................................................
[00:52:11] ..............................i.....................................................................
[00:52:14] ....................................................................................................
---
[01:18:07]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[01:18:11] error: variable does not need to be mutable
[01:18:11]    --> librustc_data_structures/indexed_set.rs:329:13
[01:18:11]     |
[01:18:11] 329 |         let mut idx_buf = IdxSetBuf::new_filled(i);
[01:18:11]     |             |
[01:18:11]     |             help: remove this `mut`
[01:18:11]     |
[01:18:11]     = note: `-D unused-mut` implied by `-D warnings`
[01:18:11]     = note: `-D unused-mut` implied by `-D warnings`
[01:18:11] 
a1e3-ec8a859a911c kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001  ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @f36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.377092] Initializing cgroup subsys net_prio
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.377938] Initializing cgroup subsys hugetlb
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.378808] Initializing cgroup subsys pids
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.379771] CPU: Physical Processor ID: 0
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.380587] CPU: Processor Core ID: 0
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.381967] mce: CPU supports 32 MCE banks
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.382750] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.383788] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.387349] Freeing SMP alternatives memory: 32K
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.397256] ftrace: allocating 32185 entries in 126 pages
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.451057] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.452431] smpboot: Max logical packages: 2
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.453706] x2apic enabled
Aug  9 19:36:27 travis-y 16
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.612385] cpuidle: using governor ladder
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.624375] cpuidle: using governor menu
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.625272] PCCT header not found.
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.625956] ACPI: bus type PCI registered
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.626582] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.627637] PCI: Using configuration type 1 for base access
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.641337] ACPI: Added _OSI(Module Device)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.642249] ACPI: Added _OSI(Processor Device)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.642906] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.643597] ACPI: Added _OSI(Processor Aggregator Device)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.647037] ACPI: Executed 2 blocks of module-level executable AML code
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.670050] ACPI: Interpreter enabled
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.670879] ACPI: (supports S0 S3 S4 S5)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.671511] ACPI: Using IOAPIC for interrupt routing
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.672564] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.701594] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.702668] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.703924] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.704846] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.707206] PCI host bridge to bus 0000:00
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.708113] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.709329] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.710397] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.711453] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfe270] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.789583] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.791807] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.794087] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.796193] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.817334] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.818511] vgaarb: loaded
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.819287] SCSI subsystem initialized
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.820023] libata version 3.00 loaded.
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.820042] ACPI: bus type USB registered
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.820670] usbcore: registered new interface driver usbfs
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.821433] usbcore: registered new interface driver hub
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.822251] usbcore: registered new device driver usb
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.823787] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.824845] dmi: Firmware registration failed.
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.825655] PCI: Using ACPI for IRQ routing
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.826370] PCI: pci_cache_line_size set to 64 bytes
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.826472] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.826474] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.826593] NetLabel: Initializing
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.827136] NetLabel:  domain hash size = 128
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.827825] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.828804] NetLabel:  unlabeled traffic allowed by default
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.829947] amd_nb: Cannot enumerate AMD northbridges
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.830919] clocksource: Switched to clocksource kvm-clock
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.838096] pnp: PnP ACPI init
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.838724] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.838790] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.838835] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.838884] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.838942] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.838990] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.839031] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.839200] pnp: PnP ACPI: found 7 devices
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.847264] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.849032] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.849035] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.849036] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.849038] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.849066] NET: Registered protocol family 2
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.849991] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.851329] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.852579] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.853590] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.854568] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.856359] NET: Registered protocol family 1
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.857279] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.858641] PCI: CLS 0 bytes, default 64
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    0.858690] Unpacking initramfs...
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.881306] Freeing initrd memory: 21432K
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.882075] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.882998] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.884684] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.885980] hw unit of domain pp0-core 2^-0 Joules
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.886661] hw unit of domain package 2^-0 Joules
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.887315] hw unit of domain dram 2^-16 Joules
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.888101] Scanning for low memory corruption every 60 seconds
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.889916] audit: initializing netlink subsys (disabled)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.890784] audit: type=2000 audit(1533843378.769:1): initialized
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.892109] Initialise system trusted keyring
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.893000] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.893968] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.896137] zbud: loaded
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.896786] VFS: Disk quotas dquot_6.6.0
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.897382] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.898710] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.899926] fuse init (API version 7.23)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.900776] Key type big_key registered
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.901465] Allocating IMA MOK and blacklist keyrings.
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.903394] Key type asymmetric registered
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.904112] Asymmetric key parser 'x509' registered
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.904834] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.906128] io scheduler noop registered
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.906813] io scheduler deadline registered (default)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.907691] io scheduler cfq registered
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.908385] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.909322] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.910332] intel_idle: does not run on family 6 model 63
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.910438] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.911561] ACPI: Power Button [PWRF]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.912150] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.913142] ACPI: Sleep Button [SLPF]
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.914074] GHES: HEST is not enabled!
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.916161] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.917119] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.921348] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.922320] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.926672] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.948950] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.972452] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    2.995496] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.018978] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.022059] Linux agpgart interface v0.103
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.025296] loop: module loaded
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.026760] libphy: Fixed MDIO Bus: probed
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.027977] tun: Universal TUN/TAP device driver, 1.6
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.029433] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.065313] PPP generic driver version 2.4.2
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.06wake from S4
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.092469] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.094754] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.096605] i2c /dev entries driver
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.097903] device-mapper: uevent: version 1.0.3
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.099284] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.101541] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.104114] NET: Registered protocol family 10
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.105853] NET: Registered protocol family 17
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.106975] Key type dns_resolver registered
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.108007] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.109866] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    3.111116] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 19:36:27 travis-job-ocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735223b2, max_idle_ns: 440795277976 ns
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    4.124033] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    6.219084] floppy0: no floppy controllers found
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    7.374942] raid6: sse2x1   gen()  8852 MB/s
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    7.442949] raid6: sse2x1   xor()  6901 MB/s
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    7.510941] raid6: sse2x2   gen() 11281 MB/s
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    7.578979] raid6: sse2x2   xor()  7561 MB/s
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    7.646941] raid6: sse2x4   gen() 13011 MB/s
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    7.714942] raid6: sse2x4   xor()  9092 MB/s
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    7.782946] raid6: avx2x1   gen() 17067 MB/s
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    7.850944] raid6: avx2x2   gen() 20213 MB/s
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    7.918957] raid6: avx2x4   gen() 22953 MB/s
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    7.919764] raid6: using algorithm avx2x4 gen() 22953 MB/s
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    7.920562] raid6: using avx2x2 recovery algorithm
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    7.922601] xor: automatically using best checksumming function:
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    7.962944]    avx       : 27621.000 MB/sec
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    7.977332] Btrfs loaded
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    8.018410] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    8.019744] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    8.097896] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    8.104314] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    8.105191] EXT4-fs (sda1): recovery complete
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    8.109826] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    8.296112] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [    8.408842] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)vis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [   11.420438] random: nonblocking pool is initialized
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c google-accounts: INFO Starting Google Accounts daemon.
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c google-accounts: INFO Creating a new user account for me.
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c google-accounts: INFO Created user account me.
Aug  9 19:36:27 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c google-accounts: INFO Creating a n6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [   12.459052] floppy0: no floppy controllers found
Aug  9 19:36:51 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpdate[1774]: adjust time server 169.254.169.254 offset 0.002839 sec
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpd[1809]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpd[1810]: proto: precision = 0.105 usec
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpd[1810]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpd[1810]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpd[1810]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpd[1810]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpd[1810]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpd[1810]: Listen normally on 3 eth0 10.20.255.56 UDP 123
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpd[1810]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpd[1810]: peers refreshed
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpd[1810]: Listening on routing socket on fd #21 for interface updates
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [   42.087586] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c startup-script: INFO Found startup-script in metadata.
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c startup-script: INFO startup-script: job 1 at Thu Aug  9 22:46:00 2018
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c startup-script: INFO startup-script: Return code 0.
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c startup-script: INFO startup-script: Return code 0.
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c startup-script: INFO Finished running startup scripts.
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ec2: 
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ec2: #############################################################
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ec2: 1024 3e:f4:12:a6:1a:3f:7f:60:5a:4e:4a:fe:6f:35:7b:8d  root@travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c (DSA)
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ec2: 256 14:64:4a:6c:7e:8b:51:be:2d:90:c1:3c:27:e6:04:d0  root@travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c (ECDSA)
Aug  9 19:36:58 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ec2: 256 67:63:1f:90:7b:f2:a1:c2:68:9d:4f:9d:a0:a6:9d:36  root@tr0: port 1(veth6af4176) entered forwarding state
Aug  9 19:39:15 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [  179.339245] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  9 19:39:19 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpd[1810]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  9 19:39:19 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpd[1810]: Listen normally on 6 docker0 fe80::42:24ff:fe88:3145 UDP 123
Aug  9 19:39:19 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpd[1810]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  9 19:39:19 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpd[1810]: peers refreshed
Aug  9 19:39:19 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c ntpd[1810]: new interface(s) found: waking up resolver
Aug  9 19:39:30 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [  194.359859] docker0: port 1(veth6af4176) entered forwarding state
Aug  9 20:17:01 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c CRON[17066]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  9 20:30:40 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [ 3264.538878] traps: a[5308] trap invalid opcode ip:559bac352b1b sp:7ffe8993c000 error:0 in a[559bac34f000+6000]
Aug  9 20:30:56 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [ 3279.928890] traps: a[8169] trap invalid opcode ip:7f7a953153f1 sp:7fffa5f17f50 error:0 in libstd-2339b911e3c09de8.so[7f7a952ba000+172000]
Aug  9 20:30:56 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [ 3279.956232] traps: a[8170] trap invalid opcode ip:7f0e1b53c3f1 sp:7ffc9df4e6f0 error:0 in libstd-2339b911e3c09de8.so[7f0e1b4e1000+172000]
Aug  9 20:32:21 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [ 3365.161990] traps: a[23063] trap invalid opcode ip:56500a583d68 sp:7ffce6e00550 error:0 in a[56500a581000+4000]
Aug  9 20:35:12 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [ 3536.795967] a[19272]: segfault at 0 ip 0000559a2c59a548 sp 00007ffc8ab92650 error 6 in a[559a2c597000+5000]
Aug  9 20:35:22 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [ 3546.532495] a[20014]: segfault at 1 ip 000055c2f74bfb5c sp 00007ffc7b0446c0 error 6 in a[55c2f74bd000+4000]
Aug  9 20:35:26 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [ 3550.670985] traps: a[20386] trap invalid opcode ip:55cfdbdb042c sp:7fffb26f1d90 error:0 in a[55cfdbdad000+7000]
Aug  9 20:56:28 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [ 4812.584074] docker0: port 1(veth6af4176) entered disabled state
Aug  9 20:56:28 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [ 4812.584134] veth62c3e3d: renamed from eth0
Aug  9 20:56:28 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [ 4812.640723] docker0: port 1(veth6af4176) entered disabled state
Aug  9 20:56:28 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [ 4812.642473] device veth6af4176 left promiscuous mode
Aug  9 20:56:28 travis-job-ce6ba6aa-e5ba-4c5c-a1e3-ec8a859a911c kernel: [ 4812.642478] docker0: port 1(veth6af4176) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start4285800 .
2922620 ./obj
---
156748 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
149120 ./src/llvm-emscripten/test
147700 ./obj/build/bootstrap/debug/incremental
133268 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z
133264 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z/s-f3pa1742ie-kmoblm-10j4kdvcqyom
128740 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
127716 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
127712 ./obj/src
34800 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt
