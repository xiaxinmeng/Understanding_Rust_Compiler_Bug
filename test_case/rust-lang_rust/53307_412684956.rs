plain
[00:52:27] ....................................................................................................
[00:52:30] ....................................................................................................
[00:52:33] ....................................................................................................
[00:52:36] ....................................................................................................
[00:52:39] ...................iiiiiiiii........................................................................
[00:52:45] ....................................................................................................
[00:52:49] .........................i..........................................................................
[00:52:52] ....................................i...............................................................
[00:52:55] ....................................................................................................
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:022f19dc
$ sudo tail -n 500 /var/log/syslog
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] kvm-clock: using sched offset of 1939960002 cycles
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] Zone ranges:
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000]   Device   empty
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] Movable zone start for each node
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] Early memory node ranges
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batc-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] Policy zone: Normal
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] Hierarchical RCU implementation.
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 13 21:04:45a7-a416-c5336079b76a kernel: [    0.652926] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.657336] Initializing cgroup subsys io
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.659597] Initializing cgroup subsys memory
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.662092] Initializing cgroup subsys devices
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.665097] Initializing cgroup subsys freezer
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.667549] Initializing cgroup subsys net_cls
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.670419] Initializing cgroup subsys perf_event
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.673732] Initializing cgroup subsys net_prio
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.676868] Initializing cgroup subsys hugetlb
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.679236] Initializing cgroup subsys pids
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.681806] CPU: Physical Processor ID: 0
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.684097] CPU: Processor Core ID: 0
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.686357] mce: CPU supports 32 MCE banks
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.689346] [    0.963765] evm: security.ima
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.965757] evm: security.capability
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.968930] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.974930] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.978195] pinctrl core: initialized pinctrl subsystem
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.981686] RTC time: 21:04:34, date: 08/13/18
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.985139] NET: Registered protocol family 16
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    0.999454] cpuidle: using governor ladder
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.011437] cpuidle: using governor menu
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.014483] PCCT header not found.
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.016654] ACPI: bus type PCI registered
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.018792] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.023726] PCI: Using configuration type 1 for base access
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.041153] ACPI: Added _OSI(Module Device)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.043793] ACPI: Added _OSI(Processor Device)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.046117] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.048699] ACPI: Added _OSI(Processor Aggregator Device)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.054544] ACPI: Executed 2 blocks of module-level executable AML code
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.082405] ACPI: Interpreter enabled
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.084692] ACPI: (supports S0 S3 S4 S5)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.086563] ACPI: Using IOAPIC for interrupt routing
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.090646] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.125297] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.129155] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.132724] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    .230313] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.241685] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.249889] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.270922] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.279860] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.288289] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.310391] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.315530] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.321828] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.327392] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.332317] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.355170] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.358715] 4-b7c1-45a7-a416-c5336079b76a kernel: [    1.443240] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.446255] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.450220] NET: Registered protocol family 1
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.453828] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.457340] PCI: CLS 0 bytes, default 64
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    1.458206] Unpacking initramfs...
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.531580] Freeing initrd memory: 21432K
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.534026] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.537204] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.542643] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.548423] hw unit of domain pp0-core 2^-0 Joules
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.551349] hw unit of domain package 2^-0 Joules
Aug 13 21:04:45 travis-job-0be079b76a kernel: [    3.640231] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.644481] ACPI: Sleep Button [SLPF]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.647744] GHES: HEST is not enabled!
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.652362] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.656033] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.666274] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.670167] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.680829] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.706788] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.733386] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.760575] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.787828] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baudstered to indicate activity on CPUs
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.942840] NET: Registered protocol family 10
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.945404] NET: Registered protocol family 17
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.947334] Key type dns_resolver registered
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.949596] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.952566] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.955015] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.957903] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.960879] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.965013] registered taskstats version 1
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.966707] Loading compiled-in X.509 certificates
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.969783] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.973537] zswap: loaded using pool lzo/zbud
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.977524] Key type trusted registered
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.982979] Key type encrypted registered
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.984309] ima: No TPM chip found, activating TPM-bypass!
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.986157] evm: HMAC attrs: 0x1
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.987340]   Magic number: 14:729:95
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.988849] rtc_cmos 00:00: setting system clock to 2018-08-13 21:04:37 UTC (1534194277)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.991402] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.993121] EDD information not available.
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.994353] PM: Hibernation image not present or could not be loaded.
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.995839] Freeing unused kernel memory: 1496K
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    3.997501] Write protecting the kernel read-only data: 14336k
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    4.000583] Freeing unused kernel memory: 1956K
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [848 MB/s
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    8.568411] raid6: using algorithm sse2x4 gen() 12650 MB/s
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    8.570314] raid6: .... xor() 8848 MB/s, rmw enabled
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    8.572190] raid6: using ssse3x2 recovery algorithm
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    8.576106] xor: automatically using best checksumming function:
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    8.614387]    avx       : 27097.000 MB/sec
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    8.630361] Btrfs loaded
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    8.684141] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    8.687068] EXT4-fs (sda1): write access will be enabled during recovery
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    8.790533] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    8.800160] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    8.803601] EXT4-fs (sda1): recovery complete
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [    8.811789] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   10.741859] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   10.922158] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   11.189972] random: mktemp: uninitialized urandom read (12 bytes read, 56 bits of entropy available)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   11.271541] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   11.360774] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   11.401279] EXT4-fs (sda1): resized filesystem to 7864064
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   11.739526] init: failsafe main process (1093) killed by TERM signal
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a instance-setup: INFO Running set_multiqueue.
Aug 13 21:04:45 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a instance-setup: INFO Set channels for eth0 to 4.
Aug 13 21:0-45a7-a416-c5336079b76a google-accounts: INFO Creating a new user account for solarce.
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a google-accounts: INFO Created user account solarce.
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a google-accounts: INFO Creating a new user account for asari.
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a google-accounts: INFO Created user account asari.
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a google-accounts: INFO Creating a new user account for bogdana.
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a google-accounts: INFO Created user account bogdana.
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   13.219524] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   13.222802] Bridge firewalling registered
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a google-accounts: INFO Creating a new user account for konstantin.
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   13.235204] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   13.268145] random: nonblocking pool is initialized
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   13.276434] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a google-accounts: INFO Created user account konstantin.
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a google-accounts: INFO Creating a new user account for carmen.
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   13.347990] Initializing XFRM netlink socket
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a google-accounts: INFO Created user account carmen.
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   13.355830] Netfilter messages via NETLINK v0.30.
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   13.359115] ctnetlink v0.93: registering with nfnetlink.
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a google-accounts: INFO Creating a new user account for maria.
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a google-accounts: INFO Created user account maria.
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a google-accounts: INFO Removing user packer.
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   13.466669] floppy0: no floppy controllers found
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 21:04:46 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 21:04:49 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 21:04:49 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a pollinate: To re-seed this system agai 123
Aug 13 21:04:54 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ntpd[1845]: Listen normally on 3 eth0 10.20.0.102 UDP 123
Aug 13 21:04:54 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ntpd[1845]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 13 21:04:54 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ntpd[1845]: peers refreshed
Aug 13 21:04:54 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ntpd[1845]: Listening on routing socket on fd #21 for interface updates
Aug 13 21:04:54 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ntpd[1845]: Listening on routing socket on fd #21 for interface updates
Aug 13 21:04:55 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [   21.912093] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 21:04:55 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a startup-script: INFO Found startup-script in metadata.
Aug 13 21:04:55 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 13 21:04:55 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a startup-script: INFO startup-script: job 1 at Tue Aug 14 00:14:00 2018
Aug 13 21:04:55 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a startup-script: INFO startup-script: Return code 0.
Aug 13 21:04:55 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a startup-script: INFO startup-script: Return code 0.
Aug 13 21:04:55 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a startup-script: INFO Finished running startup scripts.
Aug 13 21:04:55 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ec2: 
Aug 13 21:04:55 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ec2: #############################################################
Aug 13 21:04:55 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 13 21:04:55 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ec2: 1024 27:2f:20:98:c1:c7:23:84:59:5e:d2:75:ba:c1:6e:65  root@travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a (DSA)
Aug 13 21:04:55 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ec2: 256 58:37:b1:81:4f:f6:ea:7f:ab:e6:4c:2d:99:ea:e6:2f  root@travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a (ECDSA)
Aug 13 21:04:55 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ec2: 256 db:9b:8a:a3:7b:aa:ab:ac:7c:2d:6b:ad:de:6e:6f:67  root@travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a (ED25519)
Aug 13 21:04:55 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ec2: 2048 a8:e9:d3:74:8a:0e:d4:d1:f1:7b:b6:7b:00:7a:3a:56  root@travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a (RSA)
Aug 13 21:04:55 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 13 21:04:55 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ec2: #############################################################
Aug 13 21:05:01 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ntpdate[1924]: the NTP socket is in use, exiting
Aug 13 21:06:42 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [  129.720385] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 13 21:08:24 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [  231.452556] device vethc2659db entered promiscuous mode
Aug 13 21:08:24 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [  231.452644] docker0: port 1(vethc2659db) eP 123
Aug 13 21:08:27 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ntpd[1845]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 13 21:08:27 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ntpd[1845]: peers refreshed
Aug 13 21:08:27 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a ntpd[1845]: new interface(s) found: waking up resolver
Aug 13 21:08:39 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [  246.706107] docker0: port 1(vethc2659db) entered forwarding state
Aug 13 21:17:01 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a CRON[12380]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 13 21:59:50 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [ 3317.528243] traps: a[5201] trap invalid opcode ip:55d226ec9a9b sp:7fffda683480 error:0 in a[55d226ec6000+6000]
Aug 13 22:00:07 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [ 3334.199305] traps: a[8030] trap invalid opcode ip:7faaa1edb491 sp:7ffc4428f760 error:0 in libstd-2339b911e3c09de8.so[7faaa1e7b000+16f000]
Aug 13 22:00:07 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [ 3334.230348] traps: a[8031] trap invalid opcode ip:7f37ee275491 sp:7ffe87b3eb20 error:0 in libstd-2339b911e3c09de8.so[7f37ee215000+16f000]
Aug 13 22:01:38 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [ 3425.338849] traps: a[22899] trap invalid opcode ip:55bda32aad98 sp:7fff71b44c20 error:0 in a[55bda32a8000+4000]
Aug 13 22:04:34 travis-job-0be8a274-b7c1-45a7-a416-c5336079b76a kernel: [ 3601.027319] a[18901]: segfault at 0 ip 0000564f0f308463 sp 00007ffe1d209f80 error 6 in a[564f0f305000+5000]
Aug 13 22:04:44 travis-job-0bdev/sda1        30G   11G   18G  37% /
none            5.0M     0  5.0M   0% /run/lock
none            7.4G     0  7.4G   0% /run/shm
none            100M     0  100M   0% /run/user
none            768M     0  768M   0% /var/ramfs
