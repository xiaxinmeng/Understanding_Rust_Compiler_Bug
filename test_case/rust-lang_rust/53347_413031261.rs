plain
[00:20:47]    Compiling smallvec v0.6.3
[00:20:47]    Compiling crossbeam-epoch v0.3.1
[00:20:49]    Compiling polonius-engine v0.5.0
[00:20:49]    Compiling chalk-engine v0.6.0
[00:20:49] error[E0433]: failed to resolve. global paths cannot start with `crate`
[00:20:49]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.6.0/src/logic.rs:1169:11
[00:20:49]      |
[00:20:49] 1169 |         ::crate::maybe_grow_stack(|| self.pursue_strand(depth, strand))
[00:20:49]      |           ^^^^^ global paths cannot start with `crate`
[00:20:49]    Compiling tempfile v3.0.2
[00:20:50]    Compiling rls-span v0.4.0
[00:20:51]    Compiling rustc_apfloat v0.0.0 (file:///checkout/src/librustc_apfloat)
[00:20:51]    Compiling env_logger v0.5.10
[00:20:51]    Compiling env_logger v0.5.10
[00:20:53] error: aborting due to previous error
[00:20:53] 
[00:20:53] For more information about this error, try `rustc --explain E0433`.
[00:20:53] error: Could not compile `chalk-engine`.
[00:20:53] 
[00:20:53] Caused by:
[00:20:53]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name chalk_engine /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.6.0/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=30c883ec86c7b095 -C extra-filename=-30c883ec86c7b095 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern chalk_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_macros-1ec71b9a040fef8a.rlib --extern rustc_hash=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hash-9bd093322605ad94.rlib --cap-lints allow` (exit code: 1)
[00:20:53] warning: build failed, waiting for other jobs to finish...
-40c5-971b-010c1cb893a7 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Au224 pages used for memmap
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.000000] ACPI: IRQ5 used bys: 32768 (order: 6, 262144 bytes)
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.467416] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.469817] Initializing cgroup subsys io
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.470984] Initializing cgroup subsys memory
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.472452] Initializing cgroup subsys devices
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.474071] Initializing cgroup subsys freezer
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.475717] Initializing cgroup subsys net_cls
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.476941] Initializing cgroup subsys perf_event
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.478314] Initializing cgroup subsys net_prio
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.479787] Initializing cgroup subsys hugetlb
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.481491] Initializing cgroup subsys pids
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.483427] CPU: Physical Processor ID: 0
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.484583] CPU: Processor Core ID: 0
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.487042] mce: CPU supports 32 MCE banks
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.488383] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.490220] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.494649] Freeing SMP alternatives memory: 32K
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.506233] ftrace: allocating 32185 entries in 126 pages
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.565317] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.568075] smpboot: Max logical packages: 2
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.570019] x2apic enabled
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.572465] Switched APIC routing to physical x2apic.
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.577517] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.686438] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.689496] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.694195] x86: Booting SMP configuration:
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.695461] .... node  #0, CPUs:      #1
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.697030] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.703052]  #2
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.703848] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.709602]  #3
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.710766] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.716289] x86: Booted up 1 node, 4 CPUs
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.717515] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.721522] devtmpfs: initialized
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.726909] evm: security.selinux
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.727926] evm: security.SMACK64
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.729061] evm: security.SMACK64EXEC
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.730060] evm: security.SMACK64TRANSMUTE
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.731286] evm: security.SMACK64MMAP
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.732787] evm: security.ima
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.733848] evm: security.capability
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.735396] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.738408] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.741440] pinctrl core: initialized pinctrl subsystem
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.743285] RTC time: 21:42:08, date: 08/14/18
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.745960] NET: Registered protocol family 16
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.758482] cpuidle: using governor ladder
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.770488] cpuidle: using governor menu
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.771802] PCCT header not found.
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.772980] ACPI: bus type PCI registered
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.774314] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    0.776752] PCI: Using configuration ty66341] hw unit of domain package 2^-0 Joules
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.267828] hw unit of domain dram 2^-0 Joules
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.269336] Scanning for low memory corruption every 60 seconds
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.271966] audit: initializing netlink subsys (disabled)
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.273722] audit: type=2000 audit(1534282930.300:1): initialized
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.276022] Initialise system trusted keyring
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.277829] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.280056] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.283203] zbud: loaded
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.284504] VFS: Disk quotas dquot_6.6.0
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.286104] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.288796] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.291095] fuse init (API version 7.23)
Aug 14 21:42:17 travis-job-f099cb893a7 kernel: [    3.439701] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.443890] Linux agpgart interface v0.103
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.448013] loop: module loaded
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.449171] libphy: Fixed MDIO Bus: probed
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.450539] tun: Universal TUN/TAP device driver, 1.6
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.452246] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.492506] PPP generic driver version 2.4.2
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.493996] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.495936] ehci-pci: EHCI PCI platform driver
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.497345] ehci-platform: EHCI generic platform driver
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.498888] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.500795] ohci-pci: OHCI PCI platform driver
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.502116] ohci-platform: OHCI generic platform driver
Aug 14 21:42:745-40c5-971b-010c1cb893a7 kernel: [    3.579496] Freeing unused kernel memory: 1496K
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.580929] Write protecting the kernel read-only data: 14336k
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.583512] Freeing unused kernel memory: 1956K
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.585360] Freeing unused kernel memory: 92K
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.600634] systemd-udevd[119]: starting version 204
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.664419] scsi host0: Virtio SCSI HBA
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.670634] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.676865] AVX version of gcm_enc/dec engaged.
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.678351] AES CTR mode by8 optimization enabled
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.713205] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.713245] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.713249] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.713416] sd 0:0:1:0: [sda] Write Protect is off
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.713418] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.713460] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.714668]  sda: sda1
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.715268] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    3.719109] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    4.266630] tsc: Refined TSC clocksource calibration: 2499.777 MHz
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    4.268816] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24086721a4d, max_idle_ns: 440795248663 ns
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    4.552629] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    6.654675] floppy0: no floppy controllers found
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    7.830497] raid6: sse2x1   gen()  9170 MB/s
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [    7.898493] raid6: sse2x1   xor()  7033 MB/s
Aug 14 21:42:17 travis-job-f099f521-a745-40c5-971b-ing daemon.
Aug 14 21:42:18 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 21:42:18 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Starting Google Accounts daemon.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-clock-skew: INFO Synced system time with hardware clock.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Creating a new user account for me.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Created user account me.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Creating a new user account for henrik.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Created user account henrik.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Creating a new user account for emma.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 cron[1410]: (CRON) INFO (pidfile fd = 3)
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 cron[1455]: (CRON) STARTUP (fork ok)
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Created user account emma.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 cron[1455]: (CRON) INFO (Running @reboot jobs)
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Creating a new user account for igor.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 acpid: starting up with netlink and the input layer
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 acpid: 1 rule loaded
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 acpid: waiting for events: event logging is off
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Created user account igor.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 haveged: haveged starting up
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Created user account konstantinhaase.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Creating a new user account for aj.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [   11.952309] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Created user account aj.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [   11.962073] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Creating a new user account for solarce.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Created user account solarce.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Creating a new user account for asari.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Created user account asari.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Creating a new user account for bogdana.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Created user account bogdana.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Creating a new user account for konstantin.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Created user account konstantin.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 google-accounts: INFO Creating a new user account for carmen.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [   12.173070] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [   12.177670] Bridge firewalling registered
Aug 14 21:42:19 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [   12.190021] nf_conntrack version 0.5.0 (65536 bu83]: new interface(s) found: waking up resolver
Aug 14 21:44:41 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [  154.514135] docker0: port 1(vethf7171ab) entered forwarding state
Aug 14 22:04:25 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [ 1338.571309] vethc51a41d: renamed from eth0
Aug 14 22:04:25 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [ 1338.584354] docker0: port 1(vethf7171ab) entered disabled state
Aug 14 22:04:26 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [ 1338.633294] docker0: port 1(vethf7171ab) entered disabled state
Aug 14 22:04:26 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [ 1338.635127] device vethf7171ab left promiscuous mode
Aug 14 22:04:26 travis-job-f099f521-a745-40c5-971b-010c1cb893a7 kernel: [ 1338.635130] docker0: port 1(vethf7171ab) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:2e6f07fa
