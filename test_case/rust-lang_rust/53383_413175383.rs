plain
[00:50:36] ....................................................................................................
[00:50:39] ...............................................................................................i....
[00:50:42] ....................................................................................................
[00:50:45] ....................................................................................................
[00:50:47] ............................................iiiiiiiii...............................................
[00:50:53] ....................................................................................................
[00:50:57] ....................................................................................................
[00:51:00] .......................i............................................................................
[00:51:03] ..........................i...............................................i.i..ii...................
---
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:43] 
[00:51:43] running 3068 tests
[00:51:52] ....................................................................................................
[00:52:03] .................F.............................i....................................................
[00:52:24] ....................................................................................................
[00:52:33] ....................................................................................................
[00:52:47] ....................................................................................................
[00:52:56] ....................................................................................................
[00:52:56] ....................................................................................................
[00:53:06] .................................................................................F..................
[00:53:28] ....................................................................................................
[00:53:39] ....................................................................................................
[00:53:46] ....................................................................................................
[00:53:53] ....................................................................................................
---
[00:57:12] ....................................................................................................
[00:57:23] ....................................................................................................
7:31] stderr:
[00:57:31] ------------------------------------------
[00:57:31] warning: function is never used: `unsafe_async_fn`
[00:57:31]     |
[00:57:31]     |
[00:57:31] 103 | unsafe async fn unsafe_async_fn(x: u8) -> u8 {
[00:57:31]     |
[00:57:31]     = note: #[warn(dead_code)] on by default
[00:57:31] 
[00:57:31] warning: struct is never constructed: `Foo`
---
[00:57:31] 
[00:57:31] warning: method is never used: `async_method`
[00:57:31]    --> /checkout/src/test/run-pass/async-await.rs:115:5
[00:57:31]     |
[00:57:31] 115 |     async fn async_method(x: u8) -> u8 {
[00:57:31] 
[00:57:31] 
[00:57:31] error: internal compiler error: librustc_mir/transform/generator.rs:509: Broken MIR: generator contains type std::task::Poll<()> in MIR, but typeck only knows about {WakeOnceThenComplete, ()}
[00:57:31]    |
[00:57:31] 64 |       async move {
[00:57:31]    |  ________________^
[00:57:31]    |  ________________^
[00:57:31] 65 | |         await!(wake_and_yield_once());
[00:57:31] 67 | |     }
[00:57:31]    | |_____^
[00:57:31] 
[00:57:31] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:517:9
[00:57:31] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:517:9
[00:57:31] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:31] error: aborting due to previous error
[00:57:31] 
[00:57:31] 
[0c6aa8 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] kvm-clock: using sched offset of 1842837489 cycles
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] Zone ranges:
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000]   Device   empty
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] Movable zone start for each node
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] Early memory node ranges
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] Policy zone: Normal
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.000000] PID hash table entries1d9470c6aa8 kernel: [    0.545989] Initializing cgroup subsys io
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.547549] Initializing cgroup subsys memory
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.548949] Initializing cgroup subsys devices
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.550858] Initializing cgroup subsys freezer
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.552337] Initializing cgroup subsys net_cls
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.554552] Initializing cgroup subsys perf_event
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.556613] Initializing cgroup subsys net_prio
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.558500] Initializing cgroup subsys hugetlb
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.560929] Initializing cgroup subsys pids
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.563003] CPU: Physical Processor ID: 0
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.564632] CPU: Processor Core ID: 0
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.567318] mce: CPU supports 32 MCE banks
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.569504] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.573328] Last level dTLB entries 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.826063] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.829294] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.831564] pinctrl core: initialized pinctrl subsystem
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.833623] RTC time: 10:52:50, date: 08/15/18
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.836547] NET: Registered protocol family 16
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.849371] cpuidle: using governor ladder
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.861396] cpuidle: using governor menu
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.863172] PCCT header not found.
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.864502] ACPI: bus type PCI registered
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.866482] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.870180] PCI: Using configuration type 1 for base access
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.887190] ACPI: Added _OSI(Module Device)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.888689] ACPI: Added _OSI(Processor Device)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.892095] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.894319] ACPI: Added _OSI(Processor Aggregator Device)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.899826] ACPI: Executed 2 blocks of module-level executable AML code
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.927223] ACPI: Interpreter enabled
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.928920] ACPI: (supports S0 S3 S4 S5)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.930696] ACPI: Using IOAPIC for interrupt routing
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.932661] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.965960] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.969103] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.971547] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.973899] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.980089] PCI host bridge to bus 0000:00
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.981327] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.984337] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.987485] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.990321] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.993481] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.995825] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    0.996344] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.024295] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.055628] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.058395] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.070175] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.079695] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.104464] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.115674] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.125125] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.151370] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.156557] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.161758] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.167375] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.172607] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.195611] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.198522] vgaarb: loaded
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.199842] SCSI subsystem initialized
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.201163] libata version 3.00 loaded.
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.201187] ACPI: bus type USB registered
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.202445] usbcore: registered new interface driver usbfs
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.204577] usbcore: registered new interface driver hub
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.207019] usbcore: registered new device driver usb
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.209274] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.211580] dmi: Firmware registration failed.
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.213624] PCI: Using ACPI for IRQ routing
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.215141] PCI: pci_cache_line_size set to 64 bytes
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.215251] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.215253] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.215404] NetLabel: Initializing
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.217279] NetLabel:  domain hash size = 128
Aug -98bb-b1d9470c6aa8 kernel: [    1.265821] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.268924] NET: Registered protocol family 1
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.270710] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.272580] PCI: CLS 0 bytes, default 64
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    1.272652] Unpacking initramfs...
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.517402] Freeing initrd memory: 21432K
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.520234] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.522826] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.527891] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.531594] hw unit of domain pp0-core 2^-0 Joules
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.533126] hw unit of domain package 2^-0 Joules
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.534919] hw unit of domain dram 2^-16 Joules
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.578688] Key type asymmetric registered
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.580468] Asymmetric key parser 'x509' registered
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.582404] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.586129] io scheduler noop registered
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.587928] io scheduler deadline registered (default)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.590506] io scheduler cfq registered
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.592484] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.594732] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.598003] intel_idle: does not run on family 6 model 63
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.598124] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.600516] ACPI: Power Button [PWRF]
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.601776] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb- 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.928164] Key type encrypted registered
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.930472] ima: No TPM chip found, activating TPM-bypass!
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.932792] evm: HMAC attrs: 0x1
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.935440]   Magic number: 14:537:881
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.937918] rtc_cmos 00:00: setting system clock to 2018-08-15 10:52:53 UTC (1534330373)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.941819] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.944414] EDD information not available.
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.946575] PM: Hibernation image not present or could not be loaded.
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.948467] Freeing unused kernel memory: 1496K
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.950265] Write protecting the kernel read-only data: 14336k
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.953973] Freeing unused kernel memory: 1956K
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.956836] Freeing unused kernel memory: 92K
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    3.976087] systemd-udevd[119]: starting version 204
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    4.052846] scsi host0: Virtio SCSI HBA
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    4.060520] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    4.061564] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    4.071075] AVX2 version of gcm_enc/dec engaged.
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    4.072904] AES CTR mode by8 optimization enabled
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    4.139689] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    4.142848] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    4.146112] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    4.148813] sd 0:0:1:0: [sda] Write Protect is off
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    4.150296] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    4.150544] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    4.156777]  sda: sda1
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    4.159953] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    4.533472] tsc: Refined TSC clocksource calibration: 2299.821 MHz
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    4.536135] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x21268c5bf96, max_idle_ns: 440795323231 ns
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    4.894623] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    7.037465] floppy0: no floppy controllers found
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.205355] raid6: sse2x1   gen()  8761 MB/s
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.273333] raid6: sse2x1   xor()  6564 MB/s
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.341343] raid6: sse2x2   gen() 10990 MB/s
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.409339] raid6: sse2x2   xor()  7674 MB/s
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.477335] raid6: sse2x4   gen() 12262 MB/s
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.545332] raid6: sse2x4   xor()  8628 MB/s
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.613339] raid6: avx2x1   gen() 16671 MB/s
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.681326] raid6: avx2x2   gen() 20421 MB/s
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.749330] raid6: avx2x4   gen() 20646 MB/s
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.751091] raid6: using algorithm avx2x4 gen() 20646 MB/s
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.752819] raid6: using avx2x2 recovery algorithm
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.756333] xor: automatically using best checksumming function:
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.797334]    avx       : 21715.000 MB/sec
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.815513] Btrfs loaded
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.880627] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.883085] EXT4-fs (sda1): write access will be enabled during recovery
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.976904] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.986574] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.989174] EXT4-fs (sda1): recovery complete
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    8.996944] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    9.253370] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    9.432421] random: mountall: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    9.493702] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [    9.748315] random: cloud-init: uninitialized urandom read (32 bytes read, 34 bits of entropy available)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [   10.476762] random: cloud-init: uninitialized urandom read (32 bytes read, 42 bits of entropy available)
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [   10.639995] systemd-udevd[702]: starting version 204
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [   10.781626] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [   10.834557] intel_rapl: no valid rapl domains found in package 0
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [   10.886367] intel_rapl: no valid rapl domains found in package 0
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [   10.902360] ppdev: user-space parallel port driver
Aug 15 10:53:01 travis-job-b54ea93f-b201-4b10is-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 15 10:53:02 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 15 10:53:02 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 15 10:53:02 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 15 10:53:02 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 15 10:53:02 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 15 10:53:02 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 15 10:53:02 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 15 10:53:02 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 15 10:53:02 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 15 10:53:02 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 15 10:53:02 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 google-accounts: INFO Starting Google Accounts aded
Aug 15 10:53:03 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 acpid: waiting for events: event logging is off
Aug 15 10:53:03 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 haveged: haveged starting up
Aug 15 10:53:03 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 10:53:03 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 10:53:03 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [   13.667933] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 10:53:03 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [   13.680283] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 10:53:03 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [   13.728143] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 15 10:53:03 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [   13.733957] Bridge firewalling registered
Aug 15 10:53:03 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [   13.747639] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 15 10:53:03 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [   13.821583] Initializing XFRM netlink socket
Aug 15 10:53:03 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [   13.828667] Netfilter messages via NETLINK v0.30.
Aug 15 10:53:03 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [   13.832978] ctnetlink v0.93: registering with nfnetlink.
Aug 15 10:53:03 travis-job-b54ea93f-d-2339b911e3c09de8.so[7f147b30d000+16f000]
Aug 15 11:48:12 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [ 3322.679615] traps: a[30639] trap invalid opcode ip:55b833a738b0 sp:7ffd332bcb78 error:0 in a[55b833a73000+1000]
Aug 15 11:48:30 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [ 3340.977461] traps: a[1021] trap invalid opcode ip:5581f66ced98 sp:7ffca05f87b0 error:0 in a[5581f66cc000+4000]
Aug 15 11:51:14 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [ 3504.723946] a[29587]: segfault at 0 ip 0000561ff4dd2463 sp 00007ffd535fd160 error 6 in a[561ff4dcf000+5000]
Aug 15 11:51:23 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [ 3513.917206] a[30339]: segfault at 1 ip 00005580fbc73b8c sp 00007ffdde27cd80 error 6 in a[5580fbc71000+4000]
Aug 15 11:51:27 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [ 3517.697377] traps: a[30701] trap invalid opcode ip:55cfc781942c sp:7fff9e439750 error:0 in a[55cfc7816000+7000]
Aug 15 11:52:39 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [ 3589.552869] vethd0ff1ef: renamed from eth0
Aug 15 11:52:39 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [ 3589.592905] docker0: port 1(veth6d98f4a) entered disabled state
Aug 15 11:52:39 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [ 3589.628127] docker0: port 1(veth6d98f4a) entered disabled state
Aug 15 11:52:39 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [ 3589.630234] device veth6d98f4a left promiscuous mode
Aug 15 11:52:39 travis-job-b54ea93f-b201-4b10-98bb-b1d9470c6aa8 kernel: [ 3589.630237] docker0: port 1(veth6d98f4a) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:036b4900
