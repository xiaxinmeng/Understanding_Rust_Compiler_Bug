plain
[00:52:17] ....................................................................................................
[00:52:19] ....................................................................................................
[00:52:22] ....................................................................................................
[00:52:25] ....................................................................................................
[00:52:28] ..............iiiiiiiii.............................................................................
[00:52:34] ....................................................................................................
[00:52:37] ....................i...............................................................................
[00:52:40] ..............................i.....................................................................
[00:52:43] ....................................................................................................
---
Testing alloc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:08]    Compiling libc v0.2.43
[01:07:09]    Compiling rand v0.4.2
[01:07:11]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[01:07:13] error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g. `MyStruct<T>`)
[01:07:13]    --> liballoc/pin.rs:137:1
[01:07:13]     |
[01:07:13] 137 | impl<T: Unpin + ?Sized> From<PinBox<T>> for Box<T> {
[01:07:13]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type parameter `T` must be used as the type parameter for some local type
[01:07:13]     |
[01:07:13]     = note: only traits defined in the current crate can be implemented for a type parameter
[01:07:13] error: aborting due to previous error
[01:07:13] 
[01:07:13] For more information about this error, try `rustc --explain E0210`.
[01:07:13] error: Could not compile `alloc`.
[01:07:13] error: Could not compile `alloc`.
[01:07:13] warning: build failed, waiting for other jobs to finish...
a988-f2b38a4eabe9 kernel: [    0.000000]   Device   empty
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] Movable zone start for each node
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] Early memory node ranges
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 18:24:42 travis-job-ab-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] Policy zone: Normal
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] console [ttyS0] enabled
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel:  kernel: [    0.371685] Initializing cgroup subsys memory
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.372393] Initializing cgroup subsys devices
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.373308] Initializing cgroup subsys freezer
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.374138] Initializing cgroup subsys net_cls
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.375104] Initializing cgroup subsys perf_event
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.376096] Initializing cgroup subsys net_prio
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.376914] Initializing cgroup subsys hugetlb
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.377801] Initializing cgroup subsys pids
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.378619] CPU: Physical Processor ID: 0
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.379370] CPU: Processor Core ID: 0
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.380087] mce: CPU supports 32 MCE banks
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.381226] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.382131] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.384910] Freeing SMP alt.0 _SCP Extensions)
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.634484] ACPI: Added _OSI(Processor Aggregator Device)
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.637731] ACPI: Executed 2 blocks of module-level executable AML code
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.661227] ACPI: Interpreter enabled
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.662058] ACPI: (supports S0 S3 S4 S5)
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.662869] ACPI: Using IOAPIC for interrupt routing
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.664285] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.693810] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.695389] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.696520] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.697741] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.700895] PCI host bridge to bus 0000:00
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a9988-f2b38a4eabe9 kernel: [    0.827128] ACPI: bus type USB registered
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.827874] usbcore: registered new interface driver usbfs
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.828701] usbcore: registered new interface driver hub
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.829739] usbcore: registered new device driver usb
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.831204] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.832364] dmi: Firmware registration failed.
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.833653] PCI: Using ACPI for IRQ routing
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.834489] PCI: pci_cache_line_size set to 64 bytes
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.834586] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.834588] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.834757] NetLabel: Initializing
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.835389] NetLabel:  domain hash size = 128
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    0.836274] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 18:24:42 59-a988-f2b38a4eabe9 kernel: [    3.052774] libphy: Fixed MDIO Bus: probed
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.054069] tun: Universal TUN/TAP device driver, 1.6
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.055680] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.096337] PPP generic driver version 2.4.2
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.097692] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.098856] ehci-pci: EHCI PCI platform driver
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.100459] ehci-platform: EHCI generic platform driver
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.101852] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.103721] ohci-pci: OHCI PCI platform driver
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.105210] ohci-platform: OHCI generic platform driver
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.106333] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.108250] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [ eabe9 kernel: [    3.190587] systemd-udevd[119]: starting version 204
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.252713] scsi host0: Virtio SCSI HBA
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.257186] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.262474] AVX version of gcm_enc/dec engaged.
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.263453] AES CTR mode by8 optimization enabled
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.298920] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.298923] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.298925] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.303279] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.304220] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.304362] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.307903]  sda: sda1
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    3.308878] sd 0:0:1ng algorithm sse2x4 gen() 12593 MB/s
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    7.746020] raid6: .... xor() 8949 MB/s, rmw enabled
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    7.746754] raid6: using ssse3x2 recovery algorithm
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    7.748921] xor: automatically using best checksumming function:
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    7.787383]    avx       : 27614.000 MB/sec
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    7.800911] Btrfs loaded
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    7.844504] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    7.846004] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    7.918987] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    7.925853] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    7.926749] EXT4-fs (sda1): recovery complete
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    7.931757] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [    8.155267] random: init: uninitialized urandom read (12 bytes read, 25 bits 706987-27f1-4359-a988-f2b38a4eabe9 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 google-accounts: INFO Starting Google Accounts daemon.
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 google-accounts: INFO Creating a new user account for me.
Aug  9 18:24:42 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  9 18:24:42 travis-job-ab7069b38a4eabe9 acpid: 1 rule loaded
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 acpid: waiting for events: event logging is off
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 google-accounts: INFO Creating a new user account for konstantin.
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 google-accounts: INFO Created user account konstantin.
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 google-accounts: INFO Creating a new user account for carmen.
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 haveged: haveged starting up
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 google-accounts: INFO Created user account carmen.
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 google-accounts: INFO Creating a new user account for maria.
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [   11.975455] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [   11.978063] Bridge firewalling registered
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [   11.987528] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [   11.994811] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [   12.002995] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 google-accounts: INFO Created user account maria.
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 google-accounts: INFO Removing user packer.
Aug  9 18:24:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [   12.323558] floppy0: no floppy controllers found
Aug  9 18:24:44 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [   13.058826] Initializing XFRM netlink socket
Aug  9 18:24:44 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [   13.065093] Netfilter messages via NETLINK v0.30.
Aug  9 18:24:44 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [   13.067548] ctnetlink v0.93: registering with nfnetlink.
Aug  9 18:25:06 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ntpdate[1845]: adjust time server 169.254.169.254 offset 0.005225 sec
Aug  9 18:25:13 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ntpd[1876]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 18:25:13 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ntpd[1877]: proto: precision = 0.103 usec
Aug  9 18:25:13 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ntpd[1877]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 18:25:13 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ntpd[1877]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 18:25:13 travis-job-ab706987-27ftartup-script: INFO Finished running startup scripts.
Aug  9 18:25:14 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ec2: 
Aug  9 18:25:14 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ec2: #############################################################
Aug  9 18:25:14 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 18:25:14 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ec2: 1024 37:df:84:de:16:dd:0e:06:e7:31:49:d4:08:a8:8e:33  root@travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 (DSA)
Aug  9 18:25:14 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ec2: 256 7f:a4:ab:d9:1f:0d:cc:8b:e6:71:b3:ae:f1:ed:79:e2  root@travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 (ECDSA)
Aug  9 18:25:14 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ec2: 256 8e:a9:71:7a:e7:24:65:2a:ca:bd:90:40:44:5b:ae:2e  root@travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 (ED25519)
Aug  9 18:25:14 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ec2: 2048 df:0d:e5:fc:0e:b1:e4:50:c0:47:04:23:5e:6c:f9:1a  root@travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 (RSA)
Aug  9 18:25:14 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 18:25:14 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ec2: #############################################################
Aug  9 18:25:48 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [   76.364006] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 18:28:55 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [  264.307531] device veth17961c7 entered promiscuous mode
Aug  9 18:28:55 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [  264.307583] docker0: port 1(veth17961c7) entered forwarding state
Aug  9 18:28:55 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [  264.307590] docker0: port 1(veth17961c7) entered forwarding state
Aug  9 18:28:55 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [  264.307892] docker0: port 1(veth17961c7) entered disabled state
Aug  9 18:28:56 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [  264.398764] cgroup: docker-runc (4874) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 18:28:56 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [  264.398768] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  9 18:28:56 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [  264.470535] eth0: renamed from veth453567a
Aug  9 18:28:56 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [  264.508300] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  9 18:28:56 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [  264.509393] docker0: port 1(veth17961c7) entered forwarding state
Aug  9 18:28:56 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [  264.509410] docker0: port 1(veth17961c7) entered forwarding state
Aug  9 18:28:56 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [  264.509438] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  9 18:28:59 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ntpd[1877]: Listen normally on 5 docker0 fe80::42:f0ff:fef5:8159 UDP 123
Aug  9 18:28:59 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ntpd[1877]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  9 18:28:59 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ntpd[1877]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  9 18:28:59 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ntpd[1877]: peers refreshed
Aug  9 18:28:59 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 ntpd[1877]: new interface(s) found: waking up resolver
Aug  9 18:29:11 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [  279.533954] docker0: port 1(veth17961c7) entered forwarding state
Aug  9 19:17:01 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 CRON[25987]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  9 19:18:43 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [ 3251.389994] traps: a[5324] trap invalid opcode ip:557afb88bb1b sp:7ffc79d65580 error:0 in a[557afb888000+6000]
Aug  9 19:18:58 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [ 3266.743420] traps: a[8171] trap invalid opcode ip:7f0903a02ea1 sp:7fffbbf61590 error:0 in libstd-2339b911e3c09de8.so[7f09039a8000+172000]
Aug  9 19:18:58 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [ 3266.774093] traps: a[8177] trap invalid opcode ip:7fa71e55bea1 sp:7ffc2c14b4d0 error:0 in libstd-2339b911e3c09de8.so[7fa71e501000+172000]
Aug  9 19:20:23 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [ 3352.006280] traps: a[23090] trap invalid opcode ip:5652d1d65d68 sp:7ffe8fff9290 error:0 in a[5652d1d63000+4000]
Aug  9 19:23:14 travis-job-ab706987-27f1-4359-a988-f2b38a4eabe9 kernel: [ 3522.460893] a[19276]: segfault at 0 ip 00005650c51be
60840 ./src/llvm-emscripten/lib
60444 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools
56324 ./src/llvm/test/MC
55548 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
