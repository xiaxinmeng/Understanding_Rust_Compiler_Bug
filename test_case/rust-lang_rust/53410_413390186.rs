plain
[00:48:19] ....................................................................................................
[00:48:22] .................................................................................................i..
[00:48:25] ....................................................................................................
[00:48:28] ....................................................................................................
[00:48:30] ..............................................iiiiiiiii.............................................
[00:48:36] ....................................................................................................
[00:48:40] ....................................................................................................
[00:48:42] ..........................i.........................................................................
[00:48:45] ............................i...............................................i.i..ii.................
---
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:49] 
[00:55:49] running 90 tests
[00:56:03] .........................................................F................................
[00:56:03] 
[00:56:03] ---- [incremental] incremental/issue-49595/issue_49595.rs stdout ----
[00:56:03] 
[00:56:03] 
[00:56:03] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:56:03] status: exit code: 101
[00:56:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-49595/issue_49595.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue_49595/issue_4-89d4-8b8521d6bb01 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x10000000 ranges
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4ug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] Policy zone: Normal
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.000000] Memory: 153kernel: [    0.640599] ftrace: allocating 32185 entries in 126 pages
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.700148] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.702319] smpboot: Max logical packages: 2
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.704219] x2apic enabled
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.706306] Switched APIC routing to physical x2apic.
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.711936] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.820872] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.826305] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.832102] x86: Booting SMP configuration:
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.833754] .... node  #0, CPUs:      #1
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.835668] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.842394]  #2
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    0.843194] kvm-clock: cpu 2, msr 3:ffff1081,travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    1.264930] usbcore: registered new interface driver usbfs
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    1.267286] usbcore: registered new interface driver hub
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    1.270450] usbcore: registered new device driver usb
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    1.273634] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    1.276083] dmi: Firmware registration failed.
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    1.278115] PCI: Using ACPI for IRQ routing
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    1.279954] PCI: pci_cache_line_size set to 64 bytes
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    1.280064] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    1.280067] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    1.280216] NetLabel: Initializing
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    1.282187] NetLabel:  domain hash size = 128
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    1.284227] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    1.285766] NetLabel:  unlab16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    1.336775] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    1.339867] PCI: CLS 0 bytes, default 64
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    1.339955] Unpacking initramfs...
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.560234] Freeing initrd memory: 21432K
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.562221] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.565536] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.569827] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.572936] hw unit of domain pp0-core 2^-0 Joules
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.574467] hw unit of domain package 2^-0 Joules
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.575957] hw unit of domain dram 2^-16 Joules
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.577789] Scanning for low memory corruption every 60 seconds
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.581146] audit: initializing netlink subsys (disabled)
Aug 16 00:16:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.798447] tun: Universal TUN/TAP device driver, 1.6
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.800089] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.870872] PPP generic driver version 2.4.2
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.873821] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.878152] ehci-pci: EHCI PCI platform driver
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.879932] ehci-platform: EHCI generic platform driver
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.882127] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.884704] ohci-pci: OHCI PCI platform driver
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.886392] ohci-platform: OHCI generic platform driver
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.889036] uhci_hcd: USB Universal Host Controller Interface driver
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.892520] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    3.896710] i8042: Warning: Keylock active
Aug 16 00:16:03 travis-job-d2cf025c-62be-00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    8.810049] raid6: avx2x4   gen() 20572 MB/s
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    8.812446] raid6: using algorithm avx2x4 gen() 20572 MB/s
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    8.815412] raid6: using avx2x2 recovery algorithm
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    8.819969] xor: automatically using best checksumming function:
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    8.862074]    avx       : 21594.000 MB/sec
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    8.879175] Btrfs loaded
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    8.941397] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    8.944369] EXT4-fs (sda1): write access will be enabled during recovery
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    9.020774] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    9.030492] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    9.032372] EXT4-fs (sda1): recovery complete
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [    9.041385] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel:andom read (6 bytes read, 50 bits of entropy available)
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [   11.329049] random: cloud-init: uninitialized urandom read (32 bytes read, 50 bits of entropy available)
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [   11.535307] random: cloud-init: uninitialized urandom read (32 bytes read, 50 bits of entropy available)
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [   11.884909] random: mktemp: uninitialized urandom read (12 bytes read, 53 bits of entropy available)
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [   11.989594] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [   12.096291] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [   12.158055] EXT4-fs (sda1): resized filesystem to 7864064
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [   12.663881] init: failsafe main process (1096) killed by TERM signal
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 instance-setup: INFO Running set_multiqueue.
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 instance-setup: INFO Set channels for eth0 to 4.
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 16 00:16:03 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 instance-d6bb01 google-accounts: INFO Created user account me.
Aug 16 00:16:04 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 google-accounts: INFO Creating a new user account for aj.
Aug 16 00:16:04 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 google-accounts: INFO Created user account aj.
Aug 16 00:16:04 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 google-accounts: INFO Creating a new user account for carmen.
Aug 16 00:16:04 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 google-clock-skew: INFO Synced system time with hardware clock.
Aug 16 00:16:04 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 google-accounts: INFO Created user account carmen.
Aug 16 00:16:04 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 google-accounts: INFO Removing user packer.
Aug 16 00:16:04 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [   13.938166] floppy0: no floppy controllers found
Aug 16 00:16:04 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 00:16:04 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 00:16:04 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [   14.176047] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 16 00:16:04 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [   14.179225] Bridge firewalling registered
Aug 16 00:16:04 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [   14.190467] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
0:16:12 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 16 00:16:12 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 ec2: #############################################################
Aug 16 00:16:19 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 ntpdate[2146]: the NTP socket is in use, exiting
Aug 16 00:16:54 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [   64.484917] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 16 00:17:01 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 CRON[4128]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 16 00:18:36 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [  166.567938] device veth270ce62 entered promiscuous mode
Aug 16 00:18:36 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [  166.665319] cgroup: docker-runc (4848) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 16 00:18:36 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [  166.665322] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 16 00:18:37 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [  166.732517] eth0: renamed from veth3b757f6
Aug 16 00:18:37 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [  166.769765] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 16 00:18:37 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [  166.771137] docker0: port 1(veth270ce62) entered forwarding state
Aug 16 00:18:37 travis-job-d2cf025c-62be-47f5-89d4-8b8521d6bb01 kernel: [  166.771160] doc\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10a2bca8
travis_time:start:10a2bca8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:1ea01aa8
$ dmesg | grep -i kill
