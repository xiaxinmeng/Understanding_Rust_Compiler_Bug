plain
[00:47:57] ....................................................................................................
[00:48:00] ....................................................................................................
[00:48:03] ....................................................................................................
[00:48:06] ....................................................................................................
[00:48:08] ..............iiiiiiiii.............................................................................
[00:48:14] ....................................................................................................
[00:48:18] ....................i...............................................................................
[00:48:21] ...............................i....................................................................
[00:48:24] ....................................................................................................
---
[01:12:41]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[01:12:44] error[E0308]: mismatched types
[01:12:44]   --> librustc_data_structures/snapshot_map/test.rs:23:21
[01:12:44]    |
[01:12:44] 23 |     map.rollback_to(snapshot);
[01:12:44]    |                     |
[01:12:44]    |                     |
[01:12:44]    |                     expected reference, found struct `snapshot_map::Snapshot`
[01:12:44]    |                     help: consider borrowing here: `&snapshot`
[01:12:44]    |
[01:12:44]    = note: expected type `&snapshot_map::Snapshot`
[01:12:44]               found type `snapshot_map::Snapshot`
[01:12:44] error[E0308]: mismatched types
[01:12:44]   --> librustc_data_structures/snapshot_map/test.rs:36:21
[01:12:44]    |
[01:12:44]    |
[01:12:44] 36 |     map.rollback_to(snapshot1);
[01:12:44]    |                     |
[01:12:44]    |                     |
[01:12:44]    |                     expected reference, found struct `snapshot_map::Snapshot`
[01:12:44]    |                     help: consider borrowing here: `&snapshot1`
[01:12:44]    |
[01:12:44]    = note: expected type `&snapshot_map::Snapshot`
[01:12:44]               found type `snapshot_map::Snapshot`
[01:12:44] error[E0308]: mismatched types
[01:12:44]   --> librustc_data_structures/snapshot_map/test.rs:46:16
[01:12:44]    |
[01:12:44]    |
[01:12:44] 46 |     map.commit(snapshot2);
[01:12:44]    |                |
[01:12:44]    |                |
[01:12:44]    |                expected reference, found struct `snapshot_map::Snapshot`
[01:12:44]    |                help: consider borrowing here: `&snapshot2`
[01:12:44]    |
[01:12:44]    = note: expected type `&snapshot_map::Snapshot`
[01:12:44]               found type `snapshot_map::Snapshot`
[01:12:44] error[E0308]: mismatched types
[01:12:44]   --> librustc_data_structures/snapshot_map/test.rs:48:21
[01:12:44]    |
[01:12:44]    |
[01:12:44] 48 |     map.rollback_to(snapshot1);
[01:12:44]    |                     |
[01:12:44]    |                     |
[01:12:44]    |                     expected reference, found struct `snapshot_map::Snapshot`
[01:12:44]    |                     help: consider borrowing here: `&snapshot1`
[01:12:44]    |
[01:12:44]    = note: expected type `&snapshot_map::Snapshot`
[01:12:44]               found type `snapshot_map::Snapshot`
[01:12:44] error: aborting due to 4 previous errors
[01:12:44] 
[01:12:44] For more information about this error, try `rustc --explain E0308`.
[01:12:44] error: Could not compile `rustc_data_structures`.
[01:12:44] error: Could not compile `rustc_data_structures`.
[01:12:44] 
[01:12:44] To learn more, run the command again with --verbose.
[01:12:44] 
[01:12:44] 
[01:12:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_data_structures" "--" "--quiet"
[01:12:44] 
[01:12:44] 
[01:12:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:44] Build completed unsuccessfully in 0:27:33
[01:12:44] Build completed unsuccessfully in 0:27:33
[01:12:44] make: *** [check] Error 1
[01:12:44] Makefile:58: recipe for target 'check' failed
job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.000000] ACPI: INT_SRC_OVR 7f-4163-abca-fd8cfa12cc3a kernel: [    0.894566] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.896785] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.899274] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.903844] PCI host bridge to bus 0000:00
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.905029] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.907015] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.909503] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.912021] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.914489] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.916703] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    0.917140] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 16:26:19 travis-job-2436a417-16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.063889] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.087180] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.089540] vgaarb: loaded
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.091089] SCSI subsystem initialized
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.092549] libata version 3.00 loaded.
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.092578] ACPI: bus type USB registered
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.094053] usbcore: registered new interface driver usbfs
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.096079] usbcore: registered new interface driver hub
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.098127] usbcore: registered new device driver usb
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.099930] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.102201] dmi: Firmware registration failed.
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.103988] PCI: Using ACPI for IRQ routing
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.105866] PCI: pci_cache_line_size set to 64 bytes
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.105968] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.105971] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.106096] NetLabel: Initializing
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.107095] NetLabel:  domain hash size = 128
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.108369] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.110075] NetLabel:  unlabeled traffic allowed by default
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.112128] amd_nb: Cannot enumerate AMD northbridges
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.114007] clocksource: Switched to clocksource kvm-clock
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.123170] pnp: PnP ACPI init
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.124577] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.124678] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.124722] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.124769] pytes)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.141913] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.144693] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.146633] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.148968] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.151725] NET: Registered protocol family 1
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.153507] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.155658] PCI: CLS 0 bytes, default 64
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    1.155720] Unpacking initramfs...
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.328494] Freeing initrd memory: 21432K
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.330287] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.332363] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.335843] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.338945] hw unit of domain pp0-core 2^-0 Joules
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.340387] hw unit of domain package 2^-0 Joules
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.342075] hw unit of domain dram 2^-0 Joules
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.343881] Scanning for low memory corruption every 60 seconds
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.346357] audit: initializing netlink subsys (disabled)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.348121] audit: type=2000 audit(1533831971.035:1): initialized
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.350792] Initialise system trusted keyring
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.352775] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.354895] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.358340] zbud: loaded
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.359632] VFS: Disk quotas dquot_6.6.0
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.361197] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.363739] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.366294] fuse init (API version 7.23)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.367695] Key type big_key registered
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.369142] Allocating IMA MOK and blacklist keyrings.
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.373017] Key type asymmetric registered
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.374553] Asymmetric key parser 'x509' registered
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.376313] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.378772] io scheduler noop registered
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.380124] io scheduler deadline registered (default)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.382281] io scheduler cfq registered
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.383964] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.386157] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.388668] intel_idle: does not run on family 6 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.488566] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.512207] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.516096] Linux agpgart interface v0.103
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.519387] loop: module loaded
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.520387] libphy: Fixed MDIO Bus: probed
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.521465] tun: Universal TUN/TAP device driver, 1.6
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.522886] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.575249] PPP generic driver version 2.4.2
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.576595] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.577919] ehci-pci: EHCI PCI platform driver
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.579301] ehci-platform: EHCI generic platform driver
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.580804] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9c3a kernel: [    3.602271] device-mapper: uevent: version 1.0.3
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.603458] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.605192] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.606924] NET: Registered protocol family 10
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.608510] NET: Registered protocol family 17
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.609691] Key type dns_resolver registered
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.611205] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.612471] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.613625] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.614865] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.616394] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.618880] registered taskstats version 1
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kerK
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.641402] Write protecting the kernel read-only data: 14336k
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.642995] Freeing unused kernel memory: 1956K
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.643875] Freeing unused kernel memory: 92K
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.658526] systemd-udevd[119]: starting version 204
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.723796] scsi host0: Virtio SCSI HBA
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.730065] AVX version of gcm_enc/dec engaged.
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.730149] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.734295] AES CTR mode by8 optimization enabled
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.759929] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.760001] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.760003] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    3.760160] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8es deleted
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    8.462780] EXT4-fs (sda1): recovery complete
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    8.469736] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    8.695894] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    8.830584] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    8.883984] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    9.122506] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    9.702213] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    9.846403] systemd-udevd[702]: starting version 204
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [    9.961566] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [   10.049077] ppdev: user-space parallel port driver
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [   10.146586] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [   10.203975] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [   10.271026] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [   10.435699] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [   10.767655] random: mktemp: uninitialized urandom read (12 bytes read, 58 bits of entropy available)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [   10.846659] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [   10.922239] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [   10.968832] EXT4-fs (sda1): resized filesystem to 7864064
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a kernel: [   11.186672] init: failsafe main process (1094) killed by TERM signal
Aug  9 16:26:19 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  9 16:26:19 travis-job-2436a417-5ounts: INFO Created user account konstantinhaase.
Aug  9 16:26:20 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a google-accounts: INFO Creating a new user account for aj.
Aug  9 16:26:20 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a cron[1435]: (CRON) INFO (pidfile fd = 3)
Aug  9 16:26:20 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a cron[1478]: (CRON) STARTUP (fork ok)
Aug  9 16:26:20 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a cron[1478]: (CRON) INFO (Running @reboot jobs)
Aug  9 16:26:20 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 16:26:20 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 16:26:20 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a google-accounts: INFO Created user account aj.
Aug  9 16:26:20 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a acpid: starting up with netlink and the input layer
Aug  9 16:26:20 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a acpid: 1 rule loaded
Aug  9 16:26:20 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a acpid: waiting for events: event logging is off
Aug  9 16:26:20 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a google-accounts: INFO Creating a new user account for solarce.
Aug  9 16:26:20 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a haveged: haveged starting up
Aug  9 16:26:20 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a google-accounts: INFO Created user account solarce.
Aug  9 16:26:20 travis-job-2436a417-537f-4163-abca-fd8cfa12cc3a pollinate: system was previously seeded at [2017-       7.4G  4.0K  7.4G   1% /dev
/dev/sda1        30G   11G   18G  37% /
none            4.0K     0  4.0K   0% /sys/fs/cgroup
none            5.0M     0  5.0M   0% /run/lock
none            7.4G     0  7.4G   0% /run/shm
