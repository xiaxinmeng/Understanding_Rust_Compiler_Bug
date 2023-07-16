plain
[00:04:00]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:04:01] error: expected expression, found `i8`
[00:04:01]     --> libcore/num/mod.rs:348:21
[00:04:01]      |
[00:04:01] 348  | let n = ", $rot_op, $SelfT, ";
[00:04:01] ...
[00:04:01] ...
[00:04:01] 2012 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xai8" }
[00:04:01] 
[00:04:01] error: aborting due to previous error
[00:04:01] 
[00:04:01] error: Could not compile `core`.
[00:04:01] error: Could not compile `core`.
[00:04:01] 
[00:04:01] Caused by:
[00:04:01]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=bd44783aadae9ca1 -C extra-filename=-bd44783aadae9ca1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 1)
[00:04:01] warning: build failed, waiting for other jobs to finish...
 -n 500 /var/log/syslog
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 11:09:16 travis242-b45c-f224663f1cdf kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
A679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] Policy zone: Normal
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] Hierarchical RCU implementation.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] console [ttyS0] enabled
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.652765] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.659499] pid_max: default: 32768 minimum: 301
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.662590] ACPI: Core revision 20150930
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.671566] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.675926] Security Framework initialized
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.678216] Yama: becoming mindful.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.680410] AppArmor: AppArmor disabled by boot time parameter
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.685446] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.698784] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.707410] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.713038] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.720429] Initializing cgroup subsys io
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.723094] Initializing cgroup subsys memory
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.727394] Initializing cgroup subsys devices
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.731647] Initializing cgroup subsys freezer
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.734986] Initializing cgroup subsys net_cls
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.738388] Initializing cgroup subsys perf_event
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.742619] Initializing cgroup subsys net_prio
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    0.745810] Inib3dc-4242-b45c-f224663f1cdf kernel: [    1.037290] evm: security.selinux
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.039876] evm: security.SMACK64
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.041894] evm: security.SMACK64EXEC
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.044150] evm: security.SMACK64TRANSMUTE
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.046721] evm: security.SMACK64MMAP
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.048735] evm: security.ima
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.050658] evm: security.capability
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.054100] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.059756] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.064950] pinctrl core: initialized pinctrl subsystem
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.069194] RTC time: 11:09:05, date: 08/09/18
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.073878] NET: Registered protocol family 16
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.085550] cpuidle: using governor ladder
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.097400] cpuidle: using governor menu
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.100066] PCCT header not found.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.103555] ACPI: bus type PCI registered
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.105543] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.108951] PCI: Using configuration type 1 for base access
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.127334] ACPI: Added _OSI(Module Device)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.130538] ACPI: Added _OSI(Processor Device)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.133727] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.136933] ACPI: Added _OSI(Processor Aggregator Device)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.142823] ACPI: Executed 2 blocks of module-level executable AML code
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.169737] ACPI: Interpreter enabled
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.172050] ACPI: (supports S0 S3 S4 S5)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.174241] ACPI: Using IOAPIC for interrupt routing
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.177448] PCI: U24663f1cdf kernel: [    1.257421] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.257878] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.286242] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.312249] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.316953] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.328511] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.336539] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.359411] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.369085] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.377258] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.400513] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.406813] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.414304] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.421515] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.428106] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.450665] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.454149] vgaarb: loaded
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.455667] SCSI subsystem initialized
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.458124] libata version 3.00 loaded.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.458148] ACPI: bus type USB registered
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.460986] usbcore: registered new interface driver usbfs
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.464091] usbcore: registered new interface driver hub
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.468393] usbcore: registered new device driver usb
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.471897] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.475836] dmi: Firmware registration failed.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.479290] PCI: Using ACPI for IRQ routing
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.481997] PCI: pci_cache_line_size set to 64 bytes
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.482136] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.482138] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.482266] NetLabel: Initializing
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.484651] NetLabel:  domain hash size = 128
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.487973] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.491741] NetLabel:  unlabeled traffic allowed by default
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.496454] amd_nb: Cannot enumerate AMD northbridges
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.500024] clocksource: Switched to clocksource kvm-clock
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.511901] pnp: PnP ACPI init
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.514448] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.514522] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.514566] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.514620] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.514663] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.514740] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.514782] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.514948] pnp: PnP ACPI: found 7 devices
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.526253] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.532270] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.532273] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.532275] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    1.532276] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 4242-b45c-f224663f1cdf kernel: [    3.674621] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.681224] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.686888] hw unit of domain pp0-core 2^-0 Joules
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.689994] hw unit of domain package 2^-0 Joules
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.692903] hw unit of domain dram 2^-16 Joules
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.695388] Scanning for low memory corruption every 60 seconds
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.699808] audit: initializing netlink subsys (disabled)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.702654] audit: type=2000 audit(1533812947.077:1): initialized
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.706590] Initialise system trusted keyring
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.710015] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.713326] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.717375] zbud: loaded
Aug  9 11:09:16 travis-job-9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.767109] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.770333] intel_idle: does not run on family 6 model 63
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.770423] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.775075] ACPI: Power Button [PWRF]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.777119] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.782008] ACPI: Sleep Button [SLPF]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.784841] GHES: HEST is not enabled!
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.791263] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.794622] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.807262] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.811395] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.823075] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.848654] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.876003] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.903372] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.931415] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.938771] Linux agpgart interface v0.103
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.945899] loop: module loaded
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.947930] libphy: Fixed MDIO Bus: probed
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.949727] tun: Universal TUN/TAP device driver, 1.6
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    3.953100] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.010227] PPP generic driver version 2.4.2
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.014177] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.018855] ehci-pci: EHCI PCI platform driver
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.021411] ehci-platform: EHCI generic platform driver
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.024482] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.027855] ohci-pci: OHCI PCI platform driver
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.030606] ohci-platform: OHCI generic platform driver
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.034072] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.038201] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.043864] i8042: Warning: Keylock active
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.047280] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.050034] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.054152] mousedev: PS/2 mouse device common for all mice
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.058109] rtc_cmos 00:00: RTC can wake from S4
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.060920] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.065709] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.070100] i2c /dev entries driver
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.072730] device-mapper: uevent: version 1.0.3
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.076174] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.081929] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.087301] NET: Registered protocol family 10
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.091057] NET: Registered protocol family 17
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.093315] Key type dns_resolver registered
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.097023] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.102082] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.107050] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.110947] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.114963] microcsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.360770] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.360800] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.364168] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.365400] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.366287] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.366413] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.370244]  sda: sda1
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.371568] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.692138] tsc: Refined TSC clocksource calibration: 2299.998 MHz
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    4.693202] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212733415c7, max_idle_ns: 440795236380 ns
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    5.117878] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    7.288173] floppy0: no floppy controllers found
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    8.448067] raid6: sse2x1   gen()  9011 MB/s
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    8.516045] raid6: sse2x1   xor()  6709 MB/s
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    8.584078] raid6: sse2x2   gen() 10965 MB/s
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    8.652069] raid6: sse2x2   xor()  7364 MB/s
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    8.720058] raid6: sse2x4   gen() 12766 MB/s
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    8.788092] raid6: sse2x4   xor()  8877 MB/s
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    8.856067] raid6: avx2x1   gen() 16859 MB/s
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    8.924129] raid6: avx2x2   gen() 19347 MB/s
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    8.992116] raid6: avx2x4   gen() 21573 MB/s
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    8.994580] raid6: using algorithm avx2x4 gen() 21573 MB/s
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    8.997456] raid6: using avx2x2 recovery algorithm
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [    9.002568] xor: automatically using best checksumming function:
Aug  9 11:09:16 travis-job-6 (32 bytes read, 33 bits of entropy available)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [   10.651645] random: cloud-init: uninitialized urandom read (32 bytes read, 40 bits of entropy available)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [   10.818274] systemd-udevd[701]: starting version 204
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [   10.953141] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [   11.005644] intel_rapl: no valid rapl domains found in package 0
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [   11.069231] ppdev: user-space parallel port driver
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [   11.173247] random: mktemp: uninitialized urandom read (6 bytes read, 50 bits of entropy available)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [   11.235887] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [   11.309760] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [   11.478235] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [   11.770848] random: mktemp: uninitialized urandom read (12 bytes read, 54 bits of entropy available)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [   11.856812] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [   11.948830] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [   12.005995] EXT4-fs (sda1): resized filesystem to 7864064
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [   12.344232] init: failsafe main process (1092) killed by TERM signal
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO Running set_multiqueue.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO Set channels for eth0 to 4.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  9 11:09:16 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  9 11:09:17 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  9 11:09:17 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  9 11:09:17 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  9 11:09:17 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  9 11:09:17 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  9 11:09:17 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 11:09:17 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 11:09:17 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf google-accounts: INFO Starting Google Accounts daemon.
Aug  9 11:09:17 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf google-accounts: INFO Creating a new user account for me.
Aug  9 11:09:17 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf google-accounts: INFO Created user account me.
Aug  9 11:09:17 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf google-accounts: INFO Creating a new user account for henrik.
Aug  9 11:09:17 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf google-accounts: INFO Created user account henrik.
Aug  9 11:09:17 travis-job-679469d2-b3dc-4242-b45c-f224663f1cc-4242-b45c-f224663f1cdf kernel: [   14.113652] random: nonblocking pool is initialized
Aug  9 11:09:18 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 11:09:18 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 11:09:18 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf google-accounts: INFO Created user account carmen.
Aug  9 11:09:18 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf google-accounts: INFO Creating a new user account for maria.
Aug  9 11:09:18 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf google-accounts: INFO Created user account maria.
Aug  9 11:09:18 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf google-accounts: INFO Removing user packer.
Aug  9 11:09:21 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 11:09:21 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf cron[1708]: (CRON) INFO (pidfile fd = 3)
Aug  9 11:09:21 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 11:09:21 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf cron[1744]: (CRON) STARTUP (fork ok)
Aug  9 11:09:21 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf cron[1744]: (CRON) INFO (Running @reboot jobs)
Aug  9 11:09:21 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf acpid: starting up with netlink and the input layer
Aug  9 11:09:21 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf acpid: 1 rule loaded
Aug  9 11:09:21 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf acpid: waiting for events: event logging is off
Aug  9 11:09:22 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf haveged: haveged starting up
Aug  9 11:09:22 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [   18.177948] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 11:09:27 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpd[1853]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 11:09:27 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpd[1854]: proto: precision = 0.103 usec
Aug  9 11:09:27 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpd[1854]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 11:09:27 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpd[1854]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 11:09:27 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpd[1854]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 11:09:27 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpd[1854]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 11:09:27 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpd[1854]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 11:09:27 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpd[1854]: Listen normally on 3 eth0 10.20.1.162 UDP 123
Aug  9 11:09:27 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpd[1854]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 11:09:27 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpd[1854]: peers refreshed
Aug  9 11:09:27 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpd[1854]: Listening on routing socket d2-b3dc-4242-b45c-f224663f1cdf ec2: 256 c8:6c:c0:ed:83:6d:ae:8e:f4:f8:32:7e:e0:e4:33:73  root@travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf (ED25519)
Aug  9 11:09:27 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ec2: 2048 f8:10:a5:e6:43:19:a4:0b:6c:69:50:26:ad:17:68:86  root@travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf (RSA)
Aug  9 11:09:27 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 11:09:27 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ec2: #############################################################
Aug  9 11:09:33 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpdate[2252]: the NTP socket is in use, exiting
Aug  9 11:11:10 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [  127.045288] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 11:12:05 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [  181.418571] device vethbb13de6 entered promiscuous mode
Aug  9 11:12:05 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [  181.523288] cgroup: docker-runc (4860) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 11:12:05 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [  181.523291] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  9 11:12:05 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [  181.615094] eth0: renamed from veth061a1c4
Aug  9 11:12:05 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [  181.661917] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  9 11:12:05 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [  181.665594] docker0: port 1(vethbb13de6) entered forwarding state
Aug  9 11:12:05 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [  181.665613] docker0: port 1(vethbb13de6) entered forwarding state
Aug  9 11:12:05 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [  181.665639] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  9 11:12:08 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpd[1854]: Listen normally on 5 docker0 fe80::42:60ff:fec2:6f8b UDP 123
Aug  9 11:12:08 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpd[1854]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  9 11:12:08 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpd[1854]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  9 11:12:08 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpd[1854]: peers refreshed
Aug  9 11:12:08 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf ntpd[1854]: new interface(s) found: waking up resolver
Aug  9 11:12:20 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [  196.713561] docker0: port 1(vethbb13de6) entered forwarding state
Aug  9 11:15:19 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [  375.757177] docker0: port 1(vethbb13de6) entered disabled state
Aug  9 11:15:19 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [  375.757235] veth061a1c4: renamed from eth0
Aug  9 11:15:19 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [  375.818291] docker0: port 1(vethbb13de6) entered disabled state
Aug  9 11:15:19 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [  375.820083] device vethbb13de6 left promiscuous mode
Aug  9 11:15:19 travis-job-679469d2-b3dc-4242-b45c-f224663f1cdf kernel: [  375.820087] docker0: port 1(vethbb13de6) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:04670388
---
travis_time:end:0b8919e0:start=1533813320060938475,finish=1533813320068970049,duration=8031574
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12b96f01
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01964a54
travis_time:start:01964a54
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:2927b4d8
$ dmesg | grep -i kill
