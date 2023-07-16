plain
[00:53:57] ....................................................................................................
[00:54:00] ....................................................................................................
[00:54:02] ....................................................................................................
[00:54:06] ....................................................................................................
[00:54:08] ...................iiiiiiiii........................................................................
[00:54:14] ....................................................................................................
[00:54:18] .........................i..........................................................................
[00:54:21] ....................................i...............................................................
[00:54:24] ....................................................................................................
---
[01:24:46] travis_fold:end:stage0-linkchecker

[01:24:46] travis_time:end:stage0-linkchecker:start=1534209108786099929,finish=1534209111390916506,duration=2604816577

[01:24:59] reference/behavior-considered-undefined.html:154: directory link - nomicon
[01:24:59] reference/print.html:7281: directory link - nomicon
[01:25:01] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:25:01] 
[01:25:01] 
[01:25:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:25:01] expected success, got: exit code: 101
[01:25:01] expected success, got: exit code: 101
[01:25:01] 
[01:25:01] 
[01:25:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:25:01] Build completed unsuccessfully in 0:33:59
[01:25:01] make: *** [check] Error 1
[01:25:01] Makefile:58: recipe for target 'check' failed
6eae2096a8a2 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] kvm-clock: using sched offset of 1581354951 cycles
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] Zone ranges:
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-96a8a2 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 13 23:45:20 travis-job-060223eatecting Calgary via BIOS EBDA area
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] console [ttyS0] enabled
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: kernel: [    0.354416] Initializing cgroup subsys memory
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.355269] Initializing cgroup subsys devices
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.355946] Initializing cgroup subsys freezer
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.356694] Initializing cgroup subsys net_cls
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.357502] Initializing cgroup subsys perf_event
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.358246] Initializing cgroup subsys net_prio
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.359245] Initializing cgroup subsys hugetlb
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.360160] Initializing cgroup subsys pids
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.360967] CPU: Physical Processor ID: 0
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.361643] CPU: Processor Core ID: 0
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.363310] mce: CPU supports 32 MCE banks
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.364095] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.364937] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.368397] Free-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.562275] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.567783]  #3
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.568379] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.572800] x86: Booted up 1 node, 4 CPUs
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.573593] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.576051] devtmpfs: initialized
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.580611] evm: security.selinux
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.581198] evm: security.SMACK64
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.581700] evm: security.SMACK64EXEC
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.582228] evm: security.SMACK64TRANSMUTE
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.582845] evm: security.SMACK64MMAP
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.583403] evm: security.ima
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.583845] evm: security.capability
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.584752] clocksource: jiffies: mask: 0xffffffff max_cyclesdded _OSI(3.0 _SCP Extensions)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.631367] ACPI: Added _OSI(Processor Aggregator Device)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.634914] ACPI: Executed 2 blocks of module-level executable AML code
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.658598] ACPI: Interpreter enabled
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.659203] ACPI: (supports S0 S3 S4 S5)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.659772] ACPI: Using IOAPIC for interrupt routing
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.660611] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.691218] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.692321] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.693504] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.694609] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.697436] PCI host bridge to bus 0000:00
Aug 13 23:45:20 travis-job-060223ea-41] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.758757] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.763674] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.767418] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.780275] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.783525] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.785824] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.787797] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.790027] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.810872] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.812093] vgaarb: loaded
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.812762] SCSI subsystem initialized
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.813608] libata version 3.00 loaded.
Aug 13 23:45:20 travis-job-060223ea-3 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.821866] NetLabel:  unlabeled traffic allowed by default
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.822856] amd_nb: Cannot enumerate AMD northbridges
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.823666] clocksource: Switched to clocksource kvm-clock
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.831164] pnp: PnP ACPI init
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.831821] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.831897] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.831943] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.831995] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.832038] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.832080] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.832122] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    0.832284] pnp: PnP ACPI: found 7 devices
Aujob-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    2.990196] Asymmetric key parser 'x509' registered
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    2.992371] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    2.996029] io scheduler noop registered
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    2.998015] io scheduler deadline registered (default)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.000342] io scheduler cfq registered
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.002135] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.004987] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.008021] intel_idle: does not run on family 6 model 63
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.008123] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.011094] ACPI: Power Button [PWRF]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.013001] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.016144] ACPI: Sleep Button [SLPF]
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 ker
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.308182] Key type dns_resolver registered
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.311736] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.315820] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.319065] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.322695] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.326543] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.332913] registered taskstats version 1
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.335489] Loading compiled-in X.509 certificates
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.339025] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.345063] zswap: loaded using pool lzo/zbud
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.350676] Key type trusted registered
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.358269] Key type encrypted registered
Aug 13 23:45:20 trversion 204
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.476659] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.496731] scsi host0: Virtio SCSI HBA
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.507560] AVX2 version of gcm_enc/dec engaged.
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.507880] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.515186] AES CTR mode by8 optimization enabled
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.583339] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.583400] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.583402] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.583781] sd 0:0:1:0: [sda] Write Protect is off
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.583783] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.583864] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    3.5ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    8.143739] raid6: avx2x2   gen() 19085 MB/s
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    8.211791] raid6: avx2x4   gen() 21657 MB/s
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    8.214312] raid6: using algorithm avx2x4 gen() 21657 MB/s
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    8.217012] raid6: using avx2x2 recovery algorithm
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    8.221385] xor: automatically using best checksumming function:
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    8.263737]    avx       : 26476.000 MB/sec
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    8.281131] Btrfs loaded
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    8.366328] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    8.369716] EXT4-fs (sda1): write access will be enabled during recovery
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    8.463276] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    8.473545] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    8.476715] EXT4-fs (sda1): recovery complete
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [    8.485787] EXT4-fs (sda1): mounted filesystem with ordered dob-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [   10.530340] random: mktemp: uninitialized urandom read (6 bytes read, 50 bits of entropy available)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [   10.605799] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [   10.784032] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [   11.078765] random: mktemp: uninitialized urandom read (12 bytes read, 53 bits of entropy available)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [   11.161268] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [   11.255179] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [   11.323001] EXT4-fs (sda1): resized filesystem to 7864064
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [   11.750679] init: failsafe main process (1092) killed by TERM signal
Aug 13 23:45:20 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 13 23:45:21 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 instance-setup: INFO Running set_multiqueue.
Aug 13 23:45:21 travis-job-060223ea-4cbc-4371-bc50-6eae2b-060223ea-4cbc-4371-bc50-6eae2096a8a2 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 13 23:45:21 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 13 23:45:21 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 13 23:45:21 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 13 23:45:21 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 13 23:45:21 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 13 23:45:21 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 13 23:45:21 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 13 23:45:21 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 13 23:45:21 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug 13 23:45:21 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 13 23:45:21 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 23:45:21 tra user account for solarce.
Aug 13 23:45:22 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 google-accounts: INFO Created user account solarce.
Aug 13 23:45:22 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 google-accounts: INFO Creating a new user account for asari.
Aug 13 23:45:22 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 google-clock-skew: INFO Synced system time with hardware clock.
Aug 13 23:45:22 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [   13.247590] random: nonblocking pool is initialized
Aug 13 23:45:22 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [   13.254225] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 13 23:45:22 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 google-accounts: INFO Created user account asari.
Aug 13 23:45:22 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [   13.260913] Bridge firewalling registered
Aug 13 23:45:22 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 google-accounts: INFO Creating a new user account for bogdana.
Aug 13 23:45:22 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [   13.276513] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 13 23:45:22 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 google-accounts: INFO Created user account bogdana.
Aug 13 23:45:22 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [   13.311778] floppy0: no floppy controllers found
Aug 13 23:45:22 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [   13.323184] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 23:45:28 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 ntpd[1843]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 13 23:45:28 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 ntpd[1843]: Listen normally on 3 eth0 10.20.1.196 UDP 123
Aug 13 23:45:28 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 ntpd[1843]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 13 23:45:28 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 ntpd[1843]: peers refreshed
Aug 13 23:45:28 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 ntpd[1843]: peers refreshed
Aug 13 23:45:28 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 ntpd[1843]: Listening on routing socket on fd #21 for interface updates
Aug 13 23:45:28 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [   20.041041] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 23:45:28 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 startup-script: INFO Found startup-script in metadata.
Aug 13 23:45:28 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 13 23:45:28 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 startup-script: INFO startup-script: job 1 at Tue Aug 14 02:55:00 2018
Aug 13 23:45:28 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 startup-script: INFO startup-script: Return code 0.
Aug 13 23:45:28 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 startup-script: INFO startup-script: Return code 0.
Aug 13 23:45:28 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 startup-script: INFO Finished running startup scripts.
Aug 13 23:45:29 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 ec2: 
Aug 3b642f0 error 6 in a[55f877505000+5000]
Aug 14 00:46:28 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [ 3679.786956] a[19783]: segfault at 1 ip 0000560ed7325b8c sp 00007ffe592fa640 error 6 in a[560ed7323000+4000]
Aug 14 00:46:32 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [ 3684.099615] traps: a[20163] trap invalid opcode ip:5649538f242c sp:7fff99de1490 error:0 in a[5649538ef000+7000]
Aug 14 01:12:05 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [ 5217.205801] veth547df29: renamed from eth0
Aug 14 01:12:06 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [ 5217.221336] docker0: port 1(veth214c51a) entered disabled state
Aug 14 01:12:06 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [ 5217.274029] docker0: port 1(veth214c51a) entered disabled state
Aug 14 01:12:06 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [ 5217.275697] device veth214c51a left promiscuous mode
Aug 14 01:12:06 travis-job-060223ea-4cbc-4371-bc50-6eae2096a8a2 kernel: [ 5217.275699] docker0: port 1(veth214c51a) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:00a65165
