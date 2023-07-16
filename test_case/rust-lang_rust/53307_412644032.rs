plain
[00:51:10] ....................................................................................................
[00:51:13] ....................................................................................................
[00:51:15] ....................................................................................................
[00:51:19] ....................................................................................................
[00:51:22] ................iiiiiiiii...........................................................................
[00:51:28] ....................................................................................................
[00:51:31] ......................i.............................................................................
[00:51:34] ..................................i.................................................................
[00:51:37] ....................................................................................................
---
[01:09:02] running 2120 tests
[01:09:10] ....................................................................................................
[01:09:19] ....................................................................................................
[01:09:29] ....................................................................................................
[01:09:39] ...................................i.....................................................F..........
[01:09:54] ....................................................................................................
[01:10:01] ....................................................................................................
[01:10:09] ....................................................................................................
[01:10:16] ....................................................................................................
---
[01:11:55] ....................
[01:11:55] failures:
[01:11:55] 
[01:11:55] ---- macros.rs - unreachable (line 455) stdout ----
[01:11:55] error[E0433]: failed to resolve. Maybe a missing `extern crate core;`?
[01:11:55]   --> macros.rs:462:16
[01:11:55]    |
[01:11:55] 10 |     unsafe { ::core::hint::unreachable_unchecked() };
[01:11:55]    |                ^^^^ Maybe a missing `extern crate core;`?
[01:11:55] error[E0308]: mismatched types
[01:11:55]   --> macros.rs:457:35
[01:11:55]    |
[01:11:55]    |
[01:11:55] 5  |   fn divide_by_three(x: u32) -> u32 { // one of the poorest implementations of x/3
[01:11:55] 6  | |     for i in 0.. {
[01:11:55] 6  | |     for i in 0.. {
[01:11:55] 7  | |         if 3*i < i { panic!("u32 overflow"); }
[01:11:55] 8  | |         if x < 3*i { return i-1; }
[01:11:55] 9  | |     }
[01:11:55] 10 | |     unsafe { ::core::hint::unreachable_unchecked() };
[01:11:55]    | |                                                     - help: consider removing this semicolon
[01:11:55]    | |_^ expected u32, found ()
[01:11:55]    |
[01:11:55]    = note: expected type `u32`
[01:11:55]               found type `()`
---
[01:11:55]     macros.rs - unreachable (line 455)
[01:11:55] 
[01:11:55] test result: FAILED. 2116 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out
[01:11:55] 
[01:11:55] ravis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] SRAT:al   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000]   Device   empty
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] Movable zone start for each node
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] Early memory node ranges
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] g 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] console [ttyS0] enabled
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.000000] tsc: Detected ] Initializing cgroup subsys io
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.384289] Initializing cgroup subsys memory
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.385105] Initializing cgroup subsys devices
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.385855] Initializing cgroup subsys freezer
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.386697] Initializing cgroup subsys net_cls
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.387355] Initializing cgroup subsys perf_event
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.388335] Initializing cgroup subsys net_prio
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.389268] Initializing cgroup subsys hugetlb
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.390381] Initializing cgroup subsys pids
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.391445] CPU: Physical Processor ID: 0
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.392147] CPU: Processor Core ID: 0
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.392737] mce: CPU supports 32 MCE banks
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.393665] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.394895] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 13d321c6ca70e1 kernel: [    0.609801] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.611524] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.613117] pinctrl core: initialized pinctrl subsystem
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.614345] RTC time: 18:43:43, date: 08/13/18
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.616158] NET: Registered protocol family 16
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.627481] cpuidle: using governor ladder
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.637921] cpuidle: using governor menu
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.638638] PCCT header not found.
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.639216] ACPI: bus type PCI registered
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.639831] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.641014] PCI: Using configuration type 1 for base access
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.654905] ACPI: Added _OSI(Module Device)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.655956] ACPI: Added _OSI(Processor Device)
Aug 13 18
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.779610] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.794645] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.801696] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.806721] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.823544] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.826106] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.828749] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.831297] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.833807] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.854234] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.855449] vgaarb: loaded
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.856186] SCSI subsystem initialized
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.857305] libata version 3.00 loaded.
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.857334] ACPI: bus type USB registered
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.858008] usbcore: registered new interface driver usbfs
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.858998] usbcore: registered new interface driver hub
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.860032] usbcore: registered new device driver usb
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.861029] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.862234] dmi: Firmware registration failed.
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.863461] PCI: Using ACPI for IRQ routing
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.864322] PCI: pci_cache_line_size set to 64 bytes
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.864420] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.864422] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.864583] NetLabel: Initializing
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.865225] NetLabel:  domain hash size = 128
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.866074] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.867069] NetLabel:  unlabeled traffic allowed by default
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.868261] amd_nb: Cannot enumerate AMD northbridges
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.869573] clocksource: Switched to clocksource kvm-clock
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.877516] pnp: PnP ACPI init
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.878165] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.878243] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.878295] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.878348] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.878390] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.878432] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.878474] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 18:43:53 travis-job-ae53dcef--Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.897658] NET: Registered protocol family 1
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.898706] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.900215] PCI: CLS 0 bytes, default 64
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    0.900278] Unpacking initramfs...
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.919573] Freeing initrd memory: 21432K
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.922148] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.925561] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.933232] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.938838] hw unit of domain pp0-core 2^-0 Joules
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.941989] hw unit of domain package 2^-0 Joules
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.945191] hw unit of domain dram 2^-0 Joules
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.948111] Scanning for low memory corruption every 60 seconds
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.953912] audit: initializing netlink subsys (disabled)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.958071] audit: type=2000 audit(1534185825.124:1): initialized
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.962891] Initialise system trusted keyring
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.966215] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.970844] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.975926] zbud: loaded
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.977867] VFS: Disk quotas dquot_6.6.0
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.979974] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.984674] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.989286] fuse init (API version 7.23)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.993883] Key type big_key registered
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    2.996407] Allocating IMA MOK and blacklist keyrings.
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.005826] Key type asymmetric registered
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.010408] Asymmetric key parser 'x509' registered
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.014415] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.021957] io scheduler noop registered
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.024789] io scheduler deadline registered (default)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.029038] io scheduler cfq registered
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.031897] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.036181] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.040935] intel_idle: does not run on family 6 model 45
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.041041] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.046733] ACPI: Power Button [PWRF]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.049377] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.054342] ACPI: Sleec6ca70e1 kernel: [    3.232165] loop: module loaded
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.234869] libphy: Fixed MDIO Bus: probed
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.238072] tun: Universal TUN/TAP device driver, 1.6
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.241545] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.305656] PPP generic driver version 2.4.2
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.309556] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.314670] ehci-pci: EHCI PCI platform driver
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.317706] ehci-platform: EHCI generic platform driver
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.321918] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.327303] ohci-pci: OHCI PCI platform driver
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.330123] ohci-platform: OHCI generic platform driver
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.333809] uhci_hcd: USB Universal Host Controller Interface driver
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.337867] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.342754] i8042: Warning: Keylock active
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.347205] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.350714] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.354615] mousedev: PS/2 mouse device common for all mice
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.359370] rtc_cmos 00:00: RTC can wake from S4
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.363452] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.367961] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.372244] i2c /dev entries driver
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.375247] device-mapper: uevent: version 1.0.3
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.378368] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.385316] ledtrig-cpu: registered to indicate activity on CPUs
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.392106] NET: Registered protocol family 10
Aug 13 18:43:53 travis-job-ae53dcef-56UA
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.773887]  sda: sda1
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.775399] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.945729] tsc: Refined TSC clocksource calibration: 2600.001 MHz
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    3.950783] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3ce1c4c, max_idle_ns: 440795206275 ns
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    4.432528] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    6.581976] floppy0: no floppy controllers found
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    7.753630] raid6: sse2x1   gen()  8458 MB/s
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    7.821655] raid6: sse2x1   xor()  6518 MB/s
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    7.889661] raid6: sse2x2   gen() 10503 MB/s
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    7.957663] raid6: sse2x2   xor()  7076 MB/s
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    8.025643] raid6: sse2x4   gen() 12306 MB/s
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [    8.093659] raid6: sse2x4   xor()  8771 MB/s
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [   10.481392] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [   10.656398] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [   10.966196] random: mktemp: uninitialized urandom read (12 bytes read, 54 bits of entropy available)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [   11.055047] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [   11.154539] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [   11.213004] EXT4-fs (sda1): resized filesystem to 7864064
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [   11.674300] init: failsafe main process (1094) killed by TERM signal
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 13 18:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO Running set_multiqueue.
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO Set channels for eth0 to 4.
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 google-accounts: INFO Starting Google Accounts daemon.
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 13 18:43:54 travis-job-ae53dcef-56e3-4d66-b24.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 13 18:44:02 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 ntpd[1740]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 13 18:44:02 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 ntpd[1740]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 18:44:02 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 ntpd[1740]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 13 18:44:02 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 ntpd[1740]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 13 18:44:02 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 ntpd[1740]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 13 18:44:02 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 ntpd[1740]: Listen normally on 3 eth0 10.20.0.82 UDP 123
Aug 13 18:44:02 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 ntpd[1740]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 13 18:44:02 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 ntpd[1740]: peers refreshed
Aug 13 18:44:02 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 ntpd[1740]: Listening on routing socket on fd #21 for interface updates
Aug 13 18:44:02 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [   20.301575] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 18:44:02 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 startup-script: INFO Found startup-script in metadata.
Aug 13 18:44:02 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 startup-script: INFO Found startup-script in metadata.
Aug 13 18:44:02 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 startup-script: INFO startup-script: warning: commands will be Aug 13 18:48:15 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [  272.890663] docker0: port 1(veth67d86b5) entered forwarding state
Aug 13 18:48:15 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [  272.890696] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 13 18:48:18 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 ntpd[1740]: Listen normally on 5 docker0 fe80::42:9cff:fee1:b129 UDP 123
Aug 13 18:48:18 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 ntpd[1740]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 13 18:48:18 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 ntpd[1740]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 13 18:48:18 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 ntpd[1740]: peers refreshed
Aug 13 18:48:18 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 ntpd[1740]: new interface(s) found: waking up resolver
Aug 13 18:48:30 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [  287.925968] docker0: port 1(veth67d86b5) entered forwarding state
Aug 13 19:17:01 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 CRON[16262]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 13 19:39:09 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [ 3326.795210] traps: a[5075] trap invalid opcode ip:559345335a9b sp:7fff4f564f50 error:0 in a[559345332000+6000]
Aug 13 19:39:24 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [ 3342.517709] traps: a[7925] trap invalid opcode ip:7f67e2383491 sp:7fff83a98f20 error:0 in libstd-2339b911e3c09de8.so[7f67e2323000+16f000]
Aug 13 19:39:24 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [ 3342.550326] traps: a[7926] trap invalid opcode ip:7f89172b6491 sp:7ffec8a89f30 error:0 in libstd-2339b911e3c09de8.so[7f8917256000+16f000]
Aug 13 19:40:51 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [ 3429.268286] traps: a[22821] trap invalid opcode ip:55f4eac1dd98 sp:7fff3c371bf0 error:0 in a[55f4eac1b000+4000]
Aug 13 19:43:44 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [ 3601.779449] a[18800]: segfault at 0 ip 000055f77b538463 sp 00007ffedd49dfb0 error 6 in a[55f77b535000+5000]
Aug 13 19:43:53 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [ 3611.444891] a[19555]: segfault at 1 ip 00005593ef5afb8c sp 00007ffc14c8a6b0 error 6 in a[5593ef5ad000+4000]
Aug 13 19:43:58 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [ 3615.591964] traps: a[19925] trap invalid opcode ip:56222ee5042c sp:7ffcb98cfd10 error:0 in a[56222ee4d000+7000]
Aug 13 19:59:15 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [ 4533.492305] veth0553620: renamed from eth0
Aug 13 19:59:15 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [ 4533.513403] docker0: port 1(veth67d86b5) entered disabled state
Aug 13 19:59:15 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [ 4533.553421] docker0: port 1(veth67d86b5) entered disabled state
Aug 13 19:59:15 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [ 4533.555032] device veth67d86b5 left promiscuous mode
Aug 13 19:59:15 travis-job-ae53dcef-56e3-4d66-b2f1-d321c6ca70e1 kernel: [ 4533.555035] docker0: port 1(veth67d86b5) entered disabled state
[
