plain
[00:22:07]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:22:07]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:22:08]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:22:08]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:22:35] error[E0711]: feature `iterator_flatten` is declared stable since 1.29.0, but was previously declared stable since 1.29
[00:22:35]      |
[00:22:35]      |
[00:22:35] 1113 |     #[stable(feature = "iterator_flatten", since = "1.29.0")]
[00:22:35] 
[00:22:36] error: aborting due to previous error
[00:22:36] 
[00:22:36] For more information about this error, try `rustc --explain E0711`.
---
[00:22:36] travis_time:end:stage1-std:start=1534349393338404794,finish=1534349432474014023,duration=39135609229

[00:22:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:22:36] expected success, got: exit code: 101
[00:22:36] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:22:36] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:22:36] failed to run: /checkout/obj/bufcd1b5b18f kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.000000] Hierarchical RCU implementation.
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.000000] console [ttyS0] enabled
Aug 15 15:4es)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.512818] Initializing cgroup subsys io
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.514732] Initializing cgroup subsys memory
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.517087] Initializing cgroup subsys devices
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.519557] Initializing cgroup subsys freezer
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.521141] Initializing cgroup subsys net_cls
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.522937] Initializing cgroup subsys perf_event
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.525269] Initializing cgroup subsys net_prio
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.527924] Initializing cgroup subsys hugetlb
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.530796] Initializing cgroup subsys pids
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.532929] CPU: Physical Processor ID: 0
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.534503] CPU: Processor Core ID: 0
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.537589] mce: CPU supports 32 MCE banks
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.539691] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.542410] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.547507] Freeing SMP alternatives memory: 32K
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.559683] ftrace: allocating 32185 entries in 126 pages
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.622040] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.625218] smpboot: Max logical packages: 2
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.627648] x2apic enabled
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.630666] Switched APIC routing to physical x2apic.
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.636346] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.746617] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.750974] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.756629] x86: Booting SMP configuration:
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.757989] .... node  #0, CPUs:      #1
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.75941-9265-befcd1b5b18f kernel: [    0.862174] ACPI: Added _OSI(Processor Device)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.863824] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.865121] ACPI: Added _OSI(Processor Aggregator Device)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.869379] ACPI: Executed 2 blocks of module-level executable AML code
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.894912] ACPI: Interpreter enabled
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.896502] ACPI: (supports S0 S3 S4 S5)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.897912] ACPI: Using IOAPIC for interrupt routing
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.899520] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.932790] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.936171] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.940177] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    0.942563] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space und265-befcd1b5b18f kernel: [    1.030081] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.037730] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.059604] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.069534] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.077398] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.100212] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.104562] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.109223] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.114186] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.118056] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.140581] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.142812] vgaarb: loaded
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.144178] SCSI su27] NetLabel:  domain hash size = 128
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.162722] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.165018] NetLabel:  unlabeled traffic allowed by default
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.168024] amd_nb: Cannot enumerate AMD northbridges
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.169905] clocksource: Switched to clocksource kvm-clock
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.180264] pnp: PnP ACPI init
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.181900] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.181993] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.182038] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.182087] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.182128] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.182171] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.182211] pnp 00:0g 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.206984] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.210494] NET: Registered protocol family 1
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.212347] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.215814] PCI: CLS 0 bytes, default 64
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    1.215888] Unpacking initramfs...
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.469850] Freeing initrd memory: 21432K
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.471748] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.474191] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.478198] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.481701] hw unit of domain pp0-core 2^-0 Joules
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.483794] hw unit of domain package 2^-0 Joules
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.485154] hw unit of domain dram 2^-0 Joules
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.486694] Scanning for low memory corruption every 60 seconds
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.490782] audit: initializing netlink subsys (disabled)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.493019] audit: type=2000 audit(1534347938.161:1): initialized
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.496562] Initialise system trusted keyring
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.499709] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.503362] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.507516] zbud: loaded
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.509073] VFS: Disk quotas dquot_6.6.0
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.511003] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.513989] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.516526] fuse init (API version 7.23)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.519256] Key type big_key registered
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.520768] Allocating IMA Linux agpgart interface v0.103
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.697579] loop: module loaded
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.699500] libphy: Fixed MDIO Bus: probed
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.700999] tun: Universal TUN/TAP device driver, 1.6
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.702617] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.759067] PPP generic driver version 2.4.2
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.761386] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.764249] ehci-pci: EHCI PCI platform driver
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.766122] ehci-platform: EHCI generic platform driver
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.768353] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.770521] ohci-pci: OHCI PCI platform driver
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.772872] ohci-platform: OHCI generic platform driver
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.775541] uhci_hcd: USB Universal Host Controller Interface driver
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-bef    3.811377] NET: Registered protocol family 10
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.813168] NET: Registered protocol family 17
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.815036] Key type dns_resolver registered
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.817082] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.819343] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.822353] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.824707] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.826929] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.831471] registered taskstats version 1
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.834108] Loading compiled-in X.509 certificates
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.836845] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.840428] zswap: loaded using pool lzo/zbud
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [    3.8y available)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [   10.870261] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [   11.147394] random: mktemp: uninitialized urandom read (12 bytes read, 58 bits of entropy available)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [   11.236756] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [   11.325272] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [   11.374188] EXT4-fs (sda1): resized filesystem to 7864064
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [   11.858226] init: failsafe main process (1093) killed by TERM signal
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f instance-setup: INFO Running set_multiqueue.
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f instance-setup: INFO Set channels for eth0 to 4.
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f instance-setup: INFO /proc/irq/25/smp_affinity_18f instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 15 15:45:46 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 15 15:45:47 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 15:45:47 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 15:45:47 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 15 15:45:47 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-accounts: INFO Starting Google Accounts daemon.
Aug 15 15:45:47 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-accounts: INFO Creating a new user account for me.
Aug 15 15:45:47 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-accounts: INFO Created user account me.
Aug 15 15:45:47 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-accounts: INFO Creating a new user account for henrik.
Aug 15 15:45:47 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-accounts: INFO Created user account henrik.
Aug 15 15:45:47 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-accounts: INFO Creating a new user account for emma.
Aug 15 15:45:47 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-accounts: INFO Created user account emma.
Aug 15 15:45:47 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-accounts: INFO Creating a new user account for igor.
Aug 15 15:45:48 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-clock-skew: INFO Synced system time with hardware clock.
Aug 15 15:45:48 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [   12.968684] random: nonblocking pool is initialized
Aug 15 15:45:48 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-accounts: INFO Created user account igor.
Aug 15 15:45:48 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 15 15:45:48 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-accounts: INFO Created user account konstantinhaase.
Aug 15 15:45:48 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-accounts: INFO Creating a new user account for aj.
Aug 15 15:45:48 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-accounts: INFO Created user account aj.
Aug 15 15:45:48 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f google-accounts: INFO Creating a new user account for solarce.
Aug 15 15:45:48 travis-job-be4ba6b5:48 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f cron[1729]: (CRON) INFO (Running @reboot jobs)
Aug 15 15:45:48 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f acpid: starting up with netlink and the input layer
Aug 15 15:45:48 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f acpid: 1 rule loaded
Aug 15 15:45:48 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f acpid: waiting for events: event logging is off
Aug 15 15:45:48 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f haveged: haveged starting up
Aug 15 15:45:48 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [   13.895567] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 15:46:11 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f ntpdate[1848]: adjust time server 169.254.169.254 offset 0.005230 sec
Aug 15 15:46:18 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f ntpd[1883]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 15 15:46:18 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f ntpd[1884]: proto: precision = 0.106 usec
Aug 15 15:46:18 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f ntpd[1884]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 15 15:46:18 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f ntpd[1884]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 15 15:46:18 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f ntpd[1884]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 15 15:46:18 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f ntpd[1884]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 15 15:46:18 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f ntpd[1884]: Listen normally on 2 lo 127.0.0.1b5b18f ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 15 15:46:19 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f ec2: 1024 49:cb:04:a9:4a:e0:7b:3d:fb:5d:9b:2f:d9:bf:2e:96  root@travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f (DSA)
Aug 15 15:46:19 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f ec2: 256 7f:10:5f:a9:5b:88:27:ec:df:c7:a5:81:6b:3f:32:e5  root@travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f (ECDSA)
Aug 15 15:46:19 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f ec2: 256 63:1a:2d:a5:c9:79:4c:ec:59:17:75:0d:1e:d9:dd:55  root@travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f (ED25519)
Aug 15 15:46:19 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f ec2: 2048 d3:78:9d:e3:34:ec:3f:6b:42:d9:ef:85:51:60:1c:62  root@travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f (RSA)
Aug 15 15:46:19 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 15 15:46:19 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f ec2: #############################################################
Aug 15 15:47:54 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [  139.747648] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 15 15:50:42 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [  307.442956] device vethb09b40c entered promiscuous mode
Aug 15 15:50:42 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [  307.443008] docker0: port 1(vethb09b40c) entered forwarding state
Aug 15 15:50:42 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18f kernel: [  307.443014] docker0: port 1(vethb09b40c) entered forwarding state
Aug 15 15:50:42 travis-job-be4ba6b8-295a-4ea1-9265-befcd1b5b18-linux-gnu/native/jemalloc/src
34588 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib
34524 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build
34496 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build
34376 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects
