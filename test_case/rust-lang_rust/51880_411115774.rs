plain
[00:13:33]    Compiling rustc_metadata_utils v0.0.0 (file:///checkout/src/librustc_metadata_utils)
[00:13:33]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:13:33]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:34]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:13:35] error[E0599]: no method named `as_ref` found for type `std::iter::FlatMap<std::option::Iter<'_, &rustc::hir::GenericArgs>, std::slice::Iter<'_, rustc::hir::GenericArg>, [closure@librustc_typeck/astconv.rs:271:57: 271:96]>` in the current scope
[00:13:35]    --> librustc_typeck/astconv.rs:293:38
[00:13:35]     |
[00:13:35] 293 |                                 args.as_ref().for_each(drop); // Exhaust the iterator.
[00:13:35] 
[00:13:40] error: aborting due to previous error
[00:13:40] 
[00:13:40] For more information about this error, try `rustc --explain E0599`.
[00:13:40] For more information about this error, try `rustc --explain E0599`.
[00:13:40] error: Could not compile `rustc_typeck`.
[00:13:40] 
[00:13:40] Caused by:
[00:13:40]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_typeck librustc_typeck/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=dd0829b60ff380dc -C extra-filename=-dd0829b60ff380dc --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-b94d00b991f5336f.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b6c566856a1e65b9.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-e4f798202172c031.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-072369dae17b1893.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-780c150b6c3acb38.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-2e835ba951928190.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-20eb47b9c402fee3.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-1ed4d2103c0d7730.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-899ce576c6b4bcbf.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-01a673445b66da02/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-3c585aa15bfc4e69/out` (exit code: 1)
[00:13:40] warning: build failed, waiting for other jobs to finish...
 Movable zone start for each node
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] Early memory node ranges
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 16:01:50 travis-job-138a8acb-6c68-433a66520ce kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] Policy zone: Normal
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] console [ttyS0] enabled
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.326347] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f Initializing cgroup subsys devices
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.360190] Initializing cgroup subsys freezer
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.360839] Initializing cgroup subsys net_cls
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.361764] Initializing cgroup subsys perf_event
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.362435] Initializing cgroup subsys net_prio
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.363216] Initializing cgroup subsys hugetlb
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.363942] Initializing cgroup subsys pids
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.364765] CPU: Physical Processor ID: 0
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.365348] CPU: Processor Core ID: 0
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.365957] mce: CPU supports 32 MCE banks
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.366808] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.367644] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.370356] Freeing SMP alternatives memory: 32K
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.378737] ftrace: allocating 32185 entries in 126 pages
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.425003] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.426274] smpboot: Max logical packages: 2
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.427565] x2apic enabled
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.429703] Switched APIC routing to physical x2apic.
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.433432] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.541420] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.543449] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.546293] x86: Booting SMP configuration:
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.547162] .... node  #0, CPUs:      #1
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.548024] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.551410]  #2
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.552084] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 16:01:50 travis-job-138a8acb-8f-b3f7-a8a3a66520ce kernel: [    0.688007] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.689061] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.690207] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.691788] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.692877] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.693286] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.711089] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.725946] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.727484] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.732801] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.737401] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.749101] pci 0000:b-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.812307] amd_nb: Cannot enumerate AMD northbridges
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.813249] clocksource: Switched to clocksource kvm-clock
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.821151] pnp: PnP ACPI init
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.821769] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.821852] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.821901] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.821957] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.822001] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.822047] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.822091] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.822270] pnp: PnP ACPI: found 7 devices
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.830754] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns:e kernel: [    0.841642] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.842843] PCI: CLS 0 bytes, default 64
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    0.843634] Unpacking initramfs...
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.848455] Freeing initrd memory: 21432K
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.849536] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.850650] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.852319] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.853984] hw unit of domain pp0-core 2^-0 Joules
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.854896] hw unit of domain package 2^-0 Joules
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.855811] hw unit of domain dram 2^-0 Joules
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.856759] Scanning for low memory corruption every 60 seconds
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.858353] audit: initializing netlink subsys (disabled)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: 8a3a66520ce kernel: [    2.875955] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.877338] io scheduler noop registered
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.877966] io scheduler deadline registered (default)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.878698] io scheduler cfq registered
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.879350] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.880391] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.881711] intel_idle: does not run on family 6 model 45
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.881810] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.883085] ACPI: Power Button [PWRF]
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.883853] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.885031] ACPI: Sleep Button [SLPF]
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.886012] GHES: HEST is not enabled!
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    2.888147] ACPI: PCI Interrupt LiKBD port at 0x60,0x64 irq 1
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.050762] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.051573] mousedev: PS/2 mouse device common for all mice
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.053768] rtc_cmos 00:00: RTC can wake from S4
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.054802] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.056026] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.057112] i2c /dev entries driver
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.057747] device-mapper: uevent: version 1.0.3
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.058351] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.060245] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.062450] NET: Registered protocol family 10
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.063400] NET: Registered protocol family 17
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.064068] Key type dns_resolver registered
Aug  7 16:01:50 travis-job-138a8acb138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.177456] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.179356] AVX version of gcm_enc/dec engaged.
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.180814] AES CTR mode by8 optimization enabled
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.212803] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.212818] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.214922] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.216047] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.216925] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.217037] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.219930]  sda: sda1
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.221578] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    3.254078] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 16:01:3f7-a8a3a66520ce kernel: [    7.679919] raid6: using ssse3x2 recovery algorithm
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    7.683520] xor: automatically using best checksumming function:
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    7.721319]    avx       : 27867.000 MB/sec
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    7.734767] Btrfs loaded
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    7.777857] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    7.778984] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    7.849988] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    7.856737] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    7.860254] EXT4-fs (sda1): recovery complete
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    7.867919] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    8.072838] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    8.190898] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug  7 16:016520ce kernel: [    9.693153] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [    9.993232] random: mktemp: uninitialized urandom read (12 bytes read, 61 bits of entropy available)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [   10.063903] random: mktemp: uninitialized urandom read (6 bytes read, 62 bits of entropy available)
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [   10.132395] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [   10.172347] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [   10.515327] init: failsafe main process (1093) killed by TERM signal
Aug  7 16:01:50 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce instance-setup: INFO Running set_multiqueue.
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce instance-setup: INFO Set channels for eth0 to 4.
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b33
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Starting Google Accounts daemon.
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Creating a new user account for me.
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [   11.250507] random: nonblocking pool is initialized
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Created user account me.
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Creating a new user account for henrik.
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Created user account henrik.
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Creating a new user account for emma.
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Created user account emma.
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Creating a new user account for igor.
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Created user account igor.
Aug  7 16:01:51 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Created user account konstantinhaase.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Creating a new user account for aj.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Created user account aj.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce cron[1442]: (CRON) INFO (pidfile fd = 3)
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce cron[1491]: (CRON) STARTUP (fork ok)
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce cron[1491]: (CRON) INFO (Running @reboot jobs)
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Creating a new user account for solarce.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce acpid: starting up with netlink and the input layer
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce acpid: 1 rule loaded
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce acpid: waiting for events: event logging is off
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Created user account solarce.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Creating a new user account for asari.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce haveged: haveged starting up
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Created user account asari.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Creating a new user account for bogdana.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [   11.813059] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Created user account bogdana.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [   11.831062] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Creating a new user account for konstantin.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Created user account konstantin.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [   11.911490] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Creating a new user account for carmen.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [   11.915261] Bridge firewalling registered
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [   11.927843] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Created user account carmen.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce google-accounts: INFO Creating a new user account for maria.
Aug  7 16:01:52 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [   12.005979] Initializing XFRM netlink socket
Aug  7 16:01:52 travis-job-138a8acveth4383c13: renamed from eth0
Aug  7 16:19:59 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [ 1099.479916] docker0: port 1(veth80911db) entered disabled state
Aug  7 16:20:00 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [ 1099.526692] docker0: port 1(veth80911db) entered disabled state
Aug  7 16:20:00 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [ 1099.528435] device veth80911db left promiscuous mode
Aug  7 16:20:00 travis-job-138a8acb-6c68-438f-b3f7-a8a3a66520ce kernel: [ 1099.528438] docker0: port 1(veth80911db) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:06080fb8
---
travis_time:end:18ae6dc1:start=1533658800587967234,finish=1533658800593716260,duration=5749026
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1cbf879c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:214d4fa0
travis_time:start:214d4fa0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
