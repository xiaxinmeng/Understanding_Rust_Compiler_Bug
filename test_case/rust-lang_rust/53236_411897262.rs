plain
[00:05:46]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:05:48]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:05:49]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:05:52]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:06:00] error: no rules expected the token `Some`
[00:06:00]    --> libsyntax/feature_gate.rs:646:56
[00:06:00]     |
[00:06:00] 646 |     (accepted, raw_identifiers, "1.30.0", Some(48589), Some(Edition::Edition2018)),
[00:06:00] 
[00:06:00] error: aborting due to previous error
[00:06:00] 
[00:06:00] error: Could not compile `syntax`.
[00:06:00] error: Could not compile `syntax`.
[00:06:00] 
[00:06:00] Caused by:
[00:06:00]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=64bbe8e4870170a3 -C extra-filename=-64bbe8e4870170a3 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-3907cba388d41ef0.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b6c566856a1e65b9.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-8b624a6d6082b2ff.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-89eed8215142aadd.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-20eb47b9c402fee3.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-6a496b98b1d59ff3.r7213b24 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] kvm-clock: using sched offset of 1834891734 cycles
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 ke: Normal
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 20:59:40 1048576 (order: 11, 8388608 bytes)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.592505] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.596362] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.600623] Initializing cgroup subsys io
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.602391] Initializing cgroup subsys memory
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.604809] Initializing cgroup subsys devices
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.607015] Initializing cgroup subsys freezer
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.609882] Initializing cgroup subsys net_cls
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.612869] Initializing cgroup subsys perf_event
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.615549] Initializing cgroup subsys net_prio
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.619904] Initializing cgroup subsys hugetlb
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.623039] Initializing cgroup subsys pids
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.625607] CPU: Physical Processor ID: 0
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.627809] CPU: Processor Core ID: 0
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.631294] mce: CPU supports 32 MCE banks
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.633591] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.637072] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.643603] Freeing SMP alternatives memory: 32K
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.656915] ftrace: allocating 32185 entries in 126 pages
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.719098] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.722643] smpboot: Max logical packages: 2
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.725253] x2apic enabled
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.728370] Switched APIC routing to physical x2apic.
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.734487] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.845790] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    0.850562] Performance Events: unsupported p6 CPU model 63 no PMUx068000
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    1.120452] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    1.124444] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    1.134005] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    1.143098] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    1.165886] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    1.175439] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    1.183482] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    1.206120] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    1.210506] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    1.215897] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    1.220821] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    1.225752] ACPI: PCI Interr45d6-939e-c93e97213b24 kernel: [    3.519587] hw unit of domain pp0-core 2^-0 Joules
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.521198] hw unit of domain package 2^-0 Joules
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.524175] hw unit of domain dram 2^-16 Joules
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.526061] Scanning for low memory corruption every 60 seconds
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.528693] audit: initializing netlink subsys (disabled)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.531322] audit: type=2000 audit(1533848371.272:1): initialized
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.533997] Initialise system trusted keyring
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.537500] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.540491] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.544153] zbud: loaded
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.545662] VFS: Disk quotas dquot_6.6.0
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.548436] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.551397] squashfs: version 4.0 (2009/01/31-939e-c93e97213b24 kernel: [    3.706071] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.733085] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.739767] Linux agpgart interface v0.103
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.745733] loop: module loaded
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.747854] libphy: Fixed MDIO Bus: probed
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.750285] tun: Universal TUN/TAP device driver, 1.6
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.752617] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.812013] PPP generic driver version 2.4.2
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.815175] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.817722] ehci-pci: EHCI PCI platform driver
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.820687] ehci-platform: EHCI generic platform driver
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.823224] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.826653] ohci-pci: OHCIbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.864603] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.868535] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.872563] NET: Registered protocol family 10
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.874524] NET: Registered protocol family 17
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.876901] Key type dns_resolver registered
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.879405] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.882158] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.885102] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.887461] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.890595] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.895338] registered taskstats version 1
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    3.896934] Loading compiled-in X.509 certificates
Aug  9 20:59:40 travis-job-adfbc20b40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.191080] raid6: sse2x1   gen()  8894 MB/s
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.259145] raid6: sse2x1   xor()  6771 MB/s
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.327085] raid6: sse2x2   gen() 11148 MB/s
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.395091] raid6: sse2x2   xor()  7606 MB/s
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.463169] raid6: sse2x4   gen() 12310 MB/s
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.531086] raid6: sse2x4   xor()  8653 MB/s
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.599093] raid6: avx2x1   gen() 16565 MB/s
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.667095] raid6: avx2x2   gen() 20359 MB/s
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.735091] raid6: avx2x4   gen() 21636 MB/s
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.736660] raid6: using algorithm avx2x4 gen() 21636 MB/s
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.738843] raid6: using avx2x2 recovery algorithm
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.741815] xor: automatically using best checksumming function:
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.783080]    avx       : 21605.000 MB/sec
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.799625] Btrfs loaded
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.863343] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.865962] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.982933] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.992516] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    8.994389] EXT4-fs (sda1): recovery complete
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    9.001998] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    9.284607] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    9.440058] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    9.494972] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [    9.743771] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [   10.432915] random: clo59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  9 20:59:40 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 instance-setup: INFO Queue 0 XPS=1 for /sys/cogle-accounts: INFO Created user account henrik.
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 google-accounts: INFO Creating a new user account for emma.
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 google-accounts: INFO Created user account emma.
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 google-accounts: INFO Creating a new user account for igor.
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 cron[1424]: (CRON) INFO (pidfile fd = 3)
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 cron[1460]: (CRON) STARTUP (fork ok)
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 cron[1460]: (CRON) INFO (Running @reboot jobs)
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 google-accounts: INFO Created user account igor.
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 acpid: starting up with netlink and the input layer
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 acpid: 1 rule loaded
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 acpid: waiting for events: event logging is off
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 google-accounts: INFO Created user account konstantinhaase.
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 haveged: haveged starting up
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 google-accounts: INFO Creating a new user account for aj.
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [   13.242176] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [   13.252904] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 google-accounts: INFO Created user account aj.
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 google-accounts: INFO Creating a new user account for solarce.
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 google-accounts: INFO Created user account solarce.
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 google-accounts: INFO Creating a new user account for asari.
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 google-accounts: INFO Created user account asari.
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 google-accounts: INFO Creating a new user account for bogdana.
Aug  9 20:59:41 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 google-accounts: INFO Created user account bogdan939e-c93e97213b24 ec2: 256 a8:c0:2b:64:61:17:f2:fa:1d:d8:f0:67:51:58:2d:87  root@travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 (ED25519)
Aug  9 21:00:11 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 ec2: 2048 89:e0:72:aa:7b:5b:2d:93:95:09:2c:88:83:60:e3:1e  root@travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 (RSA)
Aug  9 21:00:11 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 21:00:11 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 ec2: #############################################################
Aug  9 21:00:42 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [   74.674579] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 21:01:53 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [  145.134472] device veth2cb27ef entered promiscuous mode
Aug  9 21:01:53 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [  145.134532] docker0: port 1(veth2cb27ef) entered forwarding state
Aug  9 21:01:53 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [  145.134540] docker0: port 1(veth2cb27ef) entered forwarding state
Aug  9 21:01:53 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [  145.134990] docker0: port 1(veth2cb27ef) entered disabled state
Aug  9 21:01:53 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [  145.245533] cgroup: docker-runc (4882) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 21:01:53 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [  145.245536] cgroup: "memory" requires setting use_hierarchy to 113b24 kernel: [  436.066634] vethe08a2ba: renamed from eth0
Aug  9 21:06:44 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [  436.132004] docker0: port 1(veth2cb27ef) entered disabled state
Aug  9 21:06:44 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [  436.133938] device veth2cb27ef left promiscuous mode
Aug  9 21:06:44 travis-job-adfbc20b-d298-45d6-939e-c93e97213b24 kernel: [  436.133941] docker0: port 1(veth2cb27ef) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:05797541
---
travis_time:end:20011b3c:start=1533848804826889454,finish=1533848804835147085,duration=8257631
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13797158
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:058d26d1
$ cat ./obj/build/x86_64-unknown-linux-gnu/nativ
