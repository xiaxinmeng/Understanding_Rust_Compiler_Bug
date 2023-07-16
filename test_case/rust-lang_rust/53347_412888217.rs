plain
[00:22:44]    Compiling crossbeam-epoch v0.3.1
[00:22:46]    Compiling polonius-engine v0.5.0
[00:22:47]    Compiling tempfile v3.0.2
[00:22:48]    Compiling chalk-engine v0.6.0
[00:22:48] error[E0433]: failed to resolve. global paths cannot start with `crate`
[00:22:48]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.6.0/src/logic.rs:1169:11
[00:22:48]      |
[00:22:48] 1169 |         ::crate::maybe_grow_stack(|| self.pursue_strand(depth, strand))
[00:22:48]      |           ^^^^^ global paths cannot start with `crate`
[00:22:49]    Compiling rls-span v0.4.0
[00:22:51]    Compiling rustc_apfloat v0.0.0 (file:///checkout/src/librustc_apfloat)
[00:22:52]    Compiling env_logger v0.5.10
[00:22:52] error: aborting due to previous error
[00:22:52] error: aborting due to previous error
[00:22:52] 
[00:22:52] For more information about this error, try `rustc --explain E0433`.
[00:22:52] error: Could not compile `chalk-engine`.
[00:22:52] 
[00:22:52] Caused by:
[00:22:52]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name chalk_engine /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.6.0/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=30c883ec86c7b095 -C extra-filename=-30c883ec86c7b095 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern chalk_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_macros-1ec71b9a040fef8a.rlib --extern rustc_hash=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hash-9bd093322605ad94.rlib --cap-lints allow` (exit code: 1)
[00:22:52] warning: build failed, waiting for other jobs to finish...
02212000, 0x02212fff] PGTABLE
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 000000015e603d3 kernel: [    0.000000] Early memory node ranges
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aue603d3 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] Policy zone: Normal
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] console [ttyS0] enabled
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.000000] tsc: Detected 2299.998 MHz processor
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.315090] Calibrating delay loop (skipped) preset value.. 4599.99 BogoMIPS (lpj=9199992)
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.316506] pid_max: default: 32768 minimum: 301
Aug 14 13:54:47 io  0x0d00-0xffff window]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.679312] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.680452] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.681926] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.682966] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.683425] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.697712] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.710723] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.712338] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.717705] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.722729] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    0.733900] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 13:54:47 travis-job-b9cd9985alizing netlink subsys (disabled)
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.801987] audit: type=2000 audit(1534254879.648:1): initialized
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.803152] Initialise system trusted keyring
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.804078] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.805037] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.807228] zbud: loaded
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.808005] VFS: Disk quotas dquot_6.6.0
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.808653] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.809904] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.811098] fuse init (API version 7.23)
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.812050] Key type big_key registered
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.812695] Allocating IMA MOK and blacklist keyrings.
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.814765] Key type asymmetric registered
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.815383] Asymmetric key parser 'x509' registered
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.816260] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.817414] io scheduler noop registered
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.818047] io scheduler deadline registered (default)
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.818873] io scheduler cfq registered
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.819490] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.820281] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.821225] intel_idle: does not run on family 6 model 63
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.821321] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.822562] ACPI: Power Button [PWRF]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.823202] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.824422] ACPI: Sleep Button [SLPF]
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    2.825337] GHES: HEST is not enabled!
Au-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    3.795919] tsc: Refined TSC clocksource calibration: 2300.000 MHz
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    3.796899] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735223b2, max_idle_ns: 440795277976 ns
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    4.037497] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    6.111928] floppy0: no floppy controllers found
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    7.255825] raid6: sse2x1   gen()  8858 MB/s
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    7.323819] raid6: sse2x1   xor()  6868 MB/s
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    7.391806] raid6: sse2x2   gen() 11255 MB/s
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    7.459817] raid6: sse2x2   xor()  7576 MB/s
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    7.527836] raid6: sse2x4   gen() 13077 MB/s
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    7.595800] raid6: sse2x4   xor()  9103 MB/s
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    7.663816] raid6: avx2x1   gen() 17184 MB/s
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    7.731802] raid6: avx2x2   gen() 20091 MB/s
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [    7q/31/smp_affinity_list: real affinity 3
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 14 13:54:47 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [   10.907213] random: nonblocking pool is initialized
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 google-clock-skew: INFO Synced system time with hardware clock.
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 google-accounts: INFO Starting Google Accounts daemon.
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 google-accounts: INFO Creating a new user account for me.
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 google-accounts: INFO Created user account me.
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 google-accounts: INFO Creating a new user account for bogdana.
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 google-accounts: INFO Created user account bogdana.
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 google-accounts: INFO Creating a new user account for aj.
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 google-accounts: INFO Created user account aj.
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 google-accounts: INFO Creating a new user account for asari.
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 google-accounts: INFO Created user account asari.
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 google-accounts: INFO Removing user packer.
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 cron[1433]: (CRON) INFO (pidfile fd = 3)
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 cron[1477]: (CRON) STARTUP (fork ok)
Aug 14 13:54:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 cron[1477]: (CRON) INFO (Runnin65:34:6e:7e:c0:64  root@travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 (ECDSA)
Aug 14 13:55:18 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 ec2: 256 5f:31:5b:64:bd:70:4e:3b:be:58:29:17:ec:67:57:d7  root@travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 (ED25519)
Aug 14 13:55:18 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 ec2: 2048 79:eb:80:39:30:36:97:04:4b:b1:25:ab:3c:a9:6d:6d  root@travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 (RSA)
Aug 14 13:55:18 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 14 13:55:18 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 ec2: #############################################################
Aug 14 13:55:48 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [   71.633992] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 14 13:56:51 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [  133.904508] device vethcd2df84 entered promiscuous mode
Aug 14 13:56:51 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [  133.904597] docker0: port 1(vethcd2df84) entered forwarding state
Aug 14 13:56:51 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [  133.904606] docker0: port 1(vethcd2df84) entered forwarding state
Aug 14 13:56:51 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [  133.906026] docker0: port 1(vethcd2df84) entered disabled state
Aug 14 13:56:51 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [  133.991952] cgroup: docker-runc (4754) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 13:56:51 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [  133.991955] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 13:56:51 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [  134.059520] eth0: renamed from veth6fa229e
Aug 14 13:56:51 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [  134.093639] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 13:56:51 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [  134.094579] docker0: port 1(vethcd2df84) entered forwarding state
Aug 14 13:56:51 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [  134.094596] docker0: port 1(vethcd2df84) entered forwarding state
Aug 14 13:56:51 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [  134.094612] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 14 13:56:54 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 ntpd[1777]: Listen normally on 5 docker0 fe80::42:7cff:fed9:eba8 UDP 123
Aug 14 13:56:54 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 ntpd[1777]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 14 13:56:54 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 ntpd[1777]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 14 13:56:54 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 ntpd[1777]: peers refreshed
Aug 14 13:56:54 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 ntpd[1777]: new interface(s) found: waking up resolver
Aug 14 13:57:06 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [  149.147440] docker0: port 1(vethcd2df84) entered forwarding state
Aug 14 14:17:01 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 CRON[13765]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 14 14:18:54 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [ 1457.360223] veth6fa229e: renamed from eth0
Aug 14 14:18:54 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [ 1457.400100] docker0: port 1(vethcd2df84) entered disabled state
Aug 14 14:18:54 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [ 1457.424916] docker0: port 1(vethcd2df84) entered disabled state
Aug 14 14:18:54 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [ 1457.426538] device vethcd2df84 left promiscuous mode
Aug 14 14:18:54 travis-job-b9cd9985-fd98-478b-bf4b-6a6fd5e603d3 kernel: [ 1457.426541] docker0: port 1(vethcd2df84) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:03fa2582
