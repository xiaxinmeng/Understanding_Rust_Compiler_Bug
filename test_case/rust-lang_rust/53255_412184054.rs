plain
[00:49:54] ....................................................................................................
[00:49:57] ....................................................................................................
[00:49:59] ....................................................................................................
[00:50:02] ....................................................................................................
[00:50:05] ...............iiiiiiiii............................................................................
[00:50:10] ....................................................................................................
[00:50:13] .....................i..............................................................................
[00:50:16] ................................i...................................................................
[00:50:19] ....................................................................................................
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:06e5f8b6
$ sudo tail -n 500 /var/log/syslog
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Nnel: [    0.000000]   DMA zone: 21 pages reserved
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 glo-0787-4e22-96f2-fac60a41a80d kernel: [    0.517988] mce: CPU supports 32 MCE banks
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.519890] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.521732] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.526386] Freeing SMP alternatives memory: 32K
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.537928] ftrace: allocating 32185 entries in 126 pages
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.599165] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.602523] smpboot: Max logical packages: 2
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.605490] x2apic enabled
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.608202] Switched APIC routing to physical x2apic.
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.613087] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.720175] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.724197] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.729613] x86: Booting SMP configuration:
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.731669] .... node  #0, CPUs:      #1
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.733292] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.739586]  #2
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.740776] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.747130]  #3
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.748125] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.753892] x86: Booted up 1 node, 4 CPUs
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.755490] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.759368] devtmpfs: initialized
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.764859] evm: security.selinux
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.766041] evm: security.SMACK64
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.767551] evm: security.SMACK64EXEC
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.768745] evm: security.SMACK64TRANSMUTE
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.769984] evm: security.SMACK64MMAP
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.771115] evm: security.ima
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.772001] evm: security.capability
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.773721] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.776849] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.779376] pinctrl core: initialized pinctrl subsystem
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.781061] RTC time: 18:08:12, date: 08/10/18
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.784431] NET: Registered protocol family 16
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.796240] cpuidle: using governor ladder
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.808259] cpuidle: using governor menu
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.809682] PCCT header not found.
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.811115] ACPI: bus type PCI registered
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.812676] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.814747] PCI: Using configuration type 1 for base access
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.830147] ACPI: Added _OSI(Module Device)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.832031] ACPI: Added _OSI(Processor Device)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.833842] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.835570] ACPI: Added _OSI(Processor Aggregator Device)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.839822] ACPI: Executed 2 blocks of module-level executable AML code
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.865940] ACPI: Interpreter enabled
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.867713] ACPI: (supports S0 S3 S4 S5)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.869343] ACPI: Using IOAPIC for interrupt routing
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.871343] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.906133] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.908232] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.910743] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.913133] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.918742] PCI host bridge to bus 0000:00
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.920409] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.922711] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.924715] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.927078] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.930235] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.932265] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.932747] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.954881] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.978160] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.981637] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    0.992484] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.000705] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.022574] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.031737] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.039494] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.060298] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.064649] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.069530] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.073734] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.078146] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-a41a80d kernel: [    1.121900] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.122034] NetLabel: Initializing
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.123225] NetLabel:  domain hash size = 128
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.124633] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.126600] NetLabel:  unlabeled traffic allowed by default
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.128379] amd_nb: Cannot enumerate AMD northbridges
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.130140] clocksource: Switched to clocksource kvm-clock
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.139405] pnp: PnP ACPI init
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.140918] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.140987] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.141034] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.141083] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    1.141124] pnp 00:04: Plug and Pla2^-0 Joules
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.288257] hw unit of domain package 2^-0 Joules
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.288967] hw unit of domain dram 2^-0 Joules
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.289752] Scanning for low memory corruption every 60 seconds
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.291030] audit: initializing netlink subsys (disabled)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.291880] audit: type=2000 audit(1533924494.876:1): initialized
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.293026] Initialise system trusted keyring
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.293997] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.294916] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.296905] zbud: loaded
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.297625] VFS: Disk quotas dquot_6.6.0
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.298419] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.299950] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac6 6, base_baud = 115200) is a 16550A
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.425475] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.429215] Linux agpgart interface v0.103
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.432466] loop: module loaded
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.433611] libphy: Fixed MDIO Bus: probed
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.434697] tun: Universal TUN/TAP device driver, 1.6
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.436175] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.468689] PPP generic driver version 2.4.2
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.470061] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.471693] ehci-pci: EHCI PCI platform driver
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.472815] ehci-platform: EHCI generic platform driver
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.474353] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.475859] ohci-pci: OHCI PCI platform driver
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.498858] ledtrig-cpu: registered to indicate activity on CPUs
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.500932] NET: Registered protocol family 10
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.502348] NET: Registered protocol family 17
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.503548] Key type dns_resolver registered
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.504944] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.506498] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.508151] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.509784] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.511166] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.513680] registered taskstats version 1
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.514788] Loading compiled-in X.509 certificates
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.516552] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.519296] zswap: loaded using pool lzo/zbud
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.522870] Key type trusted registered
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.527501] Key type encrypted registered
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.528579] ima: No TPM chip found, activating TPM-bypass!
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.530126] evm: HMAC attrs: 0x1
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.531337]   Magic number: 14:736:139
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.532537] acpi device:10: hash matches
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.533800] rtc_cmos 00:00: setting system clock to 2018-08-10 18:08:15 UTC (1533924495)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.536224] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.538073] EDD information not available.
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.539110] PM: Hibernation image not present or could not be loaded.
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    3.540493] Freeing unused kernel memory: 1496K
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f-4e22-96f2-fac60a41a80d kernel: [    8.344903] EXT4-fs (sda1): recovery complete
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    8.350927] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    8.584073] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    8.714260] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    8.760702] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    8.943544] random: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    9.505795] random: cloud-init: uninitialized urandom read (32 bytes read, 45 bits of entropy available)
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    9.637095] systemd-udevd[701]: starting version 204
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    9.746057] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    9.833081] ppdev: user-space parallel port driver
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [    9.953054] random: mktemp: uninitialized urandooogle-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Starting Google Accounts daemon.
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Creating a new user account for me.
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Created user account me.
Aug 10 18:08:23 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Creating a new user account for henrik.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-clock-skew: INFO Synced system time with hardware clock.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Created user account henrik.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Creating a new user account for emma.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Created user account emma.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Creating a new user account for igor.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [   12.162696] random: nonblocking pool is initialized
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Created user account igor.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Created user account konstantinhaase.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Creating a new user account for aj.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Created user account aj.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Creating a new user account for solarce.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Created user account solarce.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Creating a new user account for asari.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Created user account asari.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Creating a new user account for bogdana.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Created user account bogdana.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Creating a new user account for konstantin.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Created user account konstantin.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Creating a new user account for carmen.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Created user account carmen.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Creating a new user account for maria.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [   12.598678] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [   12.602715] Bridge firewalling registered
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [   12.610838] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Created user account maria.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [   12.642046] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d google-accounts: INFO Removing user packer.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [   12.698693] Initializing XFRM netlink socket
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [   12.705596] Netfilter messages via NETLINK v0.30.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [   12.708487] ctnetlink v0.93: registering with nfnetlink.
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [   12.802222] floppy0: no floppy controllers found
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 18:08:24 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 18:08:25 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d cron[1694]: (CRON) INFO (pidfile fd = 3)
Aug 10 18:08:25 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d cron[1725]: (CRON) STARTUP (fork ok)
Aug 10 18:08:25 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d cron[1725]: (CRON) INFO (Running @reboot jobs)
Aug 10 18:08:25 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 18:08:25 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 18:08:25 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d acpid: starting up with netlink and the input layer
Aug 10 18:08:25 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d acpid: 1 rule loaded
Aug 10 18:08:25 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d acpid: waiting for events: event logging is off
Aug 10 18:08:25 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d haveged: haveged starting up
Aug 10 18:08:25 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [   13.224715] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 18:08:48 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ntpdate[1842]: adjust time server 169.254.169.254 offset 0.005352 sec
Aug 10 18:08:55 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ntpd[1854]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 10 18:08:55 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ntpd[1855]: proto: precision = 0.134 usec
Aug 10 18:08:55 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ntpd[1855]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
---
Aug 10 18:08:55 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 10 18:08:55 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d startup-script: INFO startup-script: job 1 at Fri Aug 10 21:18:00 2018
Aug 10 18:08:55 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d startup-script: INFO startup-script: Return code 0.
Aug 10 18:08:55 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d startup-script: INFO Finished running startup scripts.
Aug 10 18:08:55 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ec2: 
Aug 10 18:08:55 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ec2: #############################################################
Aug 10 18:08:55 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 10 18:08:55 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ec2: 1024 1c:05:d3:23:20:38:a2:df:53:d5:52:a6:0c:3d:8d:13  root@travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d (DSA)
Aug 10 18:08:55 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ec2: 256 76:0f:d2:13:0d:11:0c:ea:9d:79:c5:52:7d:49:7a:7f  root@travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d (ECDSA)
Aug 10 18:08:55 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ec2: 256 54:21:3b:20:03:80:05:a2:6f:c6:8c:9c:ec:ec:56:3d  root@travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d (ED25519)
Aug 10 18:08:55 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ec2: 2048 de:d7:c0:cf:1f:05:8d:42:de:8d:9a:25:fe:7c:88:d1  root@travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d (RSA)
Aug 10 18:08:55 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 10 18:08:55 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ec2: #############################################################
Aug 10 18:17:01 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d CRON[2302]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 10 18:21:48 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [  816.760899] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 10 18:27:33 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [ 1161.601639] device veker0 fe80::1 UDP 123
Aug 10 18:27:37 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ntpd[1855]: Listen normally on 6 docker0 fe80::42:64ff:fe03:ce9c UDP 123
Aug 10 18:27:37 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ntpd[1855]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 10 18:27:37 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ntpd[1855]: peers refreshed
Aug 10 18:27:37 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d ntpd[1855]: new interface(s) found: waking up resolver
Aug 10 18:27:48 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [ 1176.812302] docker0: port 1(veth1a8df9f) entered forwarding state
Aug 10 19:12:56 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [ 3884.549132] traps: a[5335] trap invalid opcode ip:55e693d9eb4b sp:7fff677f0560 error:0 in a[55e693d9b000+6000]
Aug 10 19:13:10 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [ 3898.426780] traps: a[8151] trap invalid opcode ip:7f18b77040c1 sp:7ffd84f9a100 error:0 in libstd-2339b911e3c09de8.so[7f18b76a5000+16e000]
Aug 10 19:13:10 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [ 3898.455860] traps: a[8162] trap invalid opcode ip:7f0cd11620c1 sp:7fffe10bd8f0 error:0 in libstd-2339b911e3c09de8.so[7f0cd1103000+16e000]
Aug 10 19:14:27 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d kernel: [ 3975.636569] traps: a[22985] trap invalid opcode ip:563c3cf2ed98 sp:7fff3bcdfa50 error:0 in a[563c3cf2c000+4000]
Aug 10 19:17:01 travis-job-68eca71b-0787-4e22-96f2-fac60a41a80d CRON[18847]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
