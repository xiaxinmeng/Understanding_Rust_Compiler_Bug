plain
[00:43:38] ....................................................................................................
[00:43:41] ....................................................................................................
[00:43:43] ....................................................................................................
[00:43:46] ....................................................................................................
[00:43:48] .......iiiiiiiii....................................................................................
[00:43:54] ....................................................................................................
[00:43:57] ............i.......................................................................................
[00:44:00] .....................i..............................................................................
[00:44:03] ....................................................................................................
---
[00:51:28] ....ii...
[00:51:28] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:51:28] failures:
[00:51:28] 
[00:51:28] ---- [debuginfo-gdb] debuginfo/pretty-std.rs stdout ----
[00:51:28] NOTE: compiletest thinks it is using GDB without native rust support
[00:51:28] NOTE: compiletest thinks it is using GDB version 7011001
[00:51:28] 
[00:51:28] error: line not found in debugger output: $7 = "IAMA OS string 😃"
[00:51:28] status: exit code: 0
[00:51:28] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std/pretty-std.debugger.script"
[00:51:28] ------------------------------------------
[00:51:28] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[00:51:28] Copyright (C) 2016 Free Software Foundation, Inc.
[00:51:28] Copyright (C) 2016 Free Software Foundation, Inc.
[00:51:28] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[00:51:28] This is free software: you are free to change and redistribute it.
[00:51:28] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[00:51:28] and "show warranty" for details.
[00:51:28] This GDB was configured as "x86_64-linux-gnu".
[00:51:28] Type "show configuration" for configuration details.
[00:51:28] For bug reporting instructions, please see:
[00:51:28] <http://www.gnu.org/software/gdb/bugs/>.
[00:51:28] Find the GDB manual and other documentation resources online at:
[00:51:28] <http://www.gnu.org/software/gdb/documentation/>.
[00:51:28] For help, type "help".
[00:51:28] Type "apropos word" to search for commands related to "word".
[00:51:28] Breakpoint 1 at 0x1f88: file /checkout/src/test/debuginfo/pretty-std.rs, line 103.
[00:51:28] [Thread debugging using libthread_db enabled]
[00:51:28] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[00:51:28] 
[00:51:28] Breakpoint 1, pretty_std::main::h4d3f8e5c949f641a () at /checkout/src/test/debuginfo/pretty-std.rs:103
[00:51:28] 103     zzz(); // #break
[00:51:28] $1 = &[i32](len: 4) = {0, 1, 2, 3}
[00:51:28] $2 = Vec<u64>(len: 4, cap: 4) = {4, 5, 6, 7}
[00:51:28] $3 = "IAMA string slice!"
[00:51:28] $4 = "IAMA string!"
[00:51:28] $5 = Some = {8}
[00:51:28] $6 = None
[00:51:28] $7 = "IAMA OS string \360\237\230\203"
[00:51:28] A debugging session is active.
[00:51:28] 
[00:51:28]  Inferior 1 [process 12720] will be killed.
[00:51:28] 
[00:51:28] Quit anyway? (y or n) [answered Y; input not from terminal]
[00:51:28] ------------------------------------------
[00:51:28] stderr:
[00:51:28] ------------------------------------------
[00:51:28] 
---
[00:51:28] test result: FAILED. 87 passed; 1 failed; 21 ignored; 0 measured; 0 filtered out
[00:51:28] 
[00:51:28] 
[00:51:28] 
[00:51:2x03 -> Node 0
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] kvm-clock: using sched offset of 1628896526 cycles
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] Zone ranges:
Au169f kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irg  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] Policy zone: Normal
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kern travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.372997] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.374456] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.376086] Initializing cgroup subsys io
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.376811] Initializing cgroup subsys memory
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.377549] Initializing cgroup subsys devices
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.378582] Initializing cgroup subsys freezer
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.379377] Initializing cgroup subsys net_cls
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.380360] Initializing cgroup subsys perf_event
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.381555] Initializing cgroup subsys net_prio
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.382330] Initializing cgroup subsys hugetlb
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.383095] Initializing cgroup subsys pids
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.383974] CPU: Physical Processor ID: 0
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.384972] CPU: Processor Core ID: 0
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.387018] mce: CPU supports 32 MCE banks
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.387796] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.388954] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.392064] Freeing SMP alternatives memory: 32K
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.401462] ftrace: allocating 32185 entries in 126 pages
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.452171] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.454366] smpboot: Max logical packages: 2
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.456136] x2apic enabled
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.458944] Switched APIC routing to physical x2apic.
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.463064] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.572547] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.574986] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug  7 10:37:02 travis-job-87e06d:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.608155] evm: security.SMACK64MMAP
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.608935] evm: security.ima
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.609642] evm: security.capability
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.610723] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.612708] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.614544] pinctrl core: initialized pinctrl subsystem
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.615704] RTC time: 10:36:51, date: 08/07/18
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.617650] NET: Registered protocol family 16
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.628590] cpuidle: using governor ladder
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.640580] cpuidle: using governor menu
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.641682] PCCT header not found.
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.642331] ACPI: bus type PCI registered
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.643097] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.644739] PCI: Using configuration type 1 for base access
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.658209] ACPI: Added _OSI(Module Device)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.659820] ACPI: Added _OSI(Processor Device)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.660907] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.662522] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.666715] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.690618] ACPI: Interpreter enabled
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.691493] ACPI: (supports S0 S3 S4 S5)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.692148] ACPI: Using IOAPIC for interrupt routing
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.693614] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.724674] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.725980] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.727387] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.728633] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.731083] PCI host bridge to bus 0000:00
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.731829] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.733387] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.734531] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.735965] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.737352] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.738464] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.738912] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.756672] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.774073] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.775633] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.781690] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.787983] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.804381] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.811657] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.817031] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.832627] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.835391] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.838010] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.841422] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.844485] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 10:37:02 travis-job-87e06d39-5f21-91800890169f kernel: [    0.876693] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.876845] NetLabel: Initializing
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.877529] NetLabel:  domain hash size = 128
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.878717] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.879815] NetLabel:  unlabeled traffic allowed by default
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.881235] amd_nb: Cannot enumerate AMD northbridges
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.882176] clocksource: Switched to clocksource kvm-clock
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.890205] pnp: PnP ACPI init
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.891003] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.891079] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.891127] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.891175] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.891215] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.891256] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.891296] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.891465] pnp: PnP ACPI: found 7 devices
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.899560] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.901077] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.901080] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.901081] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.901083] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.901128] NET: Registered protocol family 2
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.902215] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    0.903544] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  p0-core 2^-0 Joules
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.025695] hw unit of domain package 2^-0 Joules
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.026509] hw unit of domain dram 2^-0 Joules
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.027440] Scanning for low memory corruption every 60 seconds
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.029274] audit: initializing netlink subsys (disabled)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.030638] audit: type=2000 audit(1533638213.843:1): initialized
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.032093] Initialise system trusted keyring
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.033485] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.034451] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.036774] zbud: loaded
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.037904] VFS: Disk quotas dquot_6.6.0
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.038798] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.040268] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.041595] fuse init (API version 7.23)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.042429] Key type big_key registered
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.043039] Allocating IMA MOK and blacklist keyrings.
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.044838] Key type asymmetric registered
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.045744] Asymmetric key parser 'x509' registered
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.046704] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.048097] io scheduler noop registered
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.048676] io scheduler deadline registered (default)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.049547] io scheduler cfq registered
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.050338] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.051133] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.052102] intel_idle: does not run on family 6 model 62
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.052211] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/ic-a521-91800890169f kernel: [    3.239811] ohci-platform: OHCI generic platform driver
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.241108] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.242613] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.245254] i8042: Warning: Keylock active
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.247394] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.248723] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.249987] mousedev: PS/2 mouse device common for all mice
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.251633] rtc_cmos 00:00: RTC can wake from S4
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.252771] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.254412] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.255675] i2c /dev entries driver
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.256477] device-mapper: uevent: version 1.0.3
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.257778] device-mavis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.304500] Freeing unused kernel memory: 1956K
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.305764] Freeing unused kernel memory: 92K
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.322337] systemd-udevd[119]: starting version 204
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.393430] scsi host0: Virtio SCSI HBA
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.398722] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.409880] AVX version of gcm_enc/dec engaged.
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.411056] AES CTR mode by8 optimization enabled
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.448452] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.448547] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.448549] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.448776] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.448778] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.448820] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.449928]  sda: sda1
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.450555] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    3.456498] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    4.026294] tsc: Refined TSC clocksource calibration: 2499.798 MHz
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    4.028136] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24087ac8d90, max_idle_ns: 440795227389 ns
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    4.292006] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    6.370445] floppy0: no floppy controllers found
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    7.542203] raid6: sse2x1   gen()  9180 MB/s
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    7.610193] raid6: sse2x1   xor()  6848 MB/s
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    7.678187] raid6: sse2x2   gen() 11144 MB/s
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    7.746189] raid6: sse2x2   xor()  7835 MB/s
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91887e06d39-5fbe-454c-a521-91800890169f kernel: [    8.086410] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    8.293726] random: init: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    8.409962] random: mountall: uninitialized urandom read (12 bytes read, 32 bits of entropy available)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    8.461918] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    8.664257] random: cloud-init: uninitialized urandom read (32 bytes read, 40 bits of entropy available)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    9.220270] random: cloud-init: uninitialized urandom read (32 bytes read, 49 bits of entropy available)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    9.365462] systemd-udevd[704]: starting version 204
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    9.473064] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    9.584401] ppdev: user-space parallel port driver
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    9.674085] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    9.730039] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    9.798561] random: cloud-init: uninitialized urandom read (32 bytes read, 62 bits of entropy available)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [    9.960255] random: cloud-init: uninitialized urandom read (32 bytes read, 62 bits of entropy available)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [   10.198955] random: mktemp: uninitialized urandom read (12 bytes read, 65 bits of entropy available)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [   10.283060] random: mktemp: uninitialized urandom read (6 bytes read, 66 bits of entropy available)
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [   10.365856] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [   10.407468] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [   10.734843] init: failsafe main process (1095) killed by TERM signal
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f instance-setup: INFO Running set_multiqueue.
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f instance-setup: INFO Set90169f instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [   11.504775] random: nonblocking pool is initialized
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Starting Google Accounts daemon.
Aug  7 10:37:02 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Creating a new user account for me.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Created user account me.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Creating a new user account for henrik.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Created user account henrik.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Creating a new user account for emma.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Created user account emma.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Creating a new user account for igor.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f cron[1416]: (CRON) INFO (pidfile fd = 3)
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f cron[1461]: (CRON) STARTUP (fork ok)
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f cron[1461]: (CRON) INFO (Running @reboot jobs)
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Created user account igor.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f acpid: starting up with netlink and the input layer
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f acpid: 1 rule loaded
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f acpid: waiting for events: event logging is off
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f haveged: haveged starting up
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Created user account konstantinhaase.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Creating a new user account for aj.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Created user account aj.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [   12.151506] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [   12.160349] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Creating a new user account for solarce.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [   12.164906] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [   12.165650] Bridge firewalling registered
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [   12.175190] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Created user account solarce.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Creating a new user account for asari.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [   12.257859] Initializing XFRM netlink socket
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [   12.266488] Netfilter messages via NETLINK v0.30.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [   12.269508] ctnetlink v0.93: registering with nfnetlink.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Created user account asari.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Creating a new user account for bogdana.
Aug  7 10:37:03 travis-job-87e06d39-5fbe-454c-a521-91800890169f google-accounts: INFO Created user account bogdana.
Aug  7 10:37:03 travis-job-87e06d39-5rnel: [  189.267855] docker0: port 1(vetheca2bfa) entered forwarding state
Aug  7 10:40:00 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [  189.268189] docker0: port 1(vetheca2bfa) entered disabled state
Aug  7 10:40:00 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [  189.364648] cgroup: docker-runc (4853) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 10:40:00 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [  189.364652] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 10:40:00 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [  189.430804] eth0: renamed from veth1c4f395
Aug  7 10:40:00 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [  189.465127] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 10:40:00 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [  189.466272] docker0: port 1(vetheca2bfa) entered forwarding state
Aug  7 10:40:00 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [  189.466285] docker0: port 1(vetheca2bfa) entered forwarding state
Aug  7 10:40:00 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [  189.466305] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 10:40:04 travis-job-87e06d39-5fbe-454c-a521-91800890169f ntpd[1863]: Listen normally on 5 docker0 fe80::42:6ff:fe71:d350 UDP 123
Aug  7 10:40:04 travis-job-87e06d39-5fbe-454c-a521-91800890169f ntpd[1863]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  7 10:40:04 travis-job-87e06d39-5fbe-454c-a521-91800890169f ntpd[1863]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 10:40:04 travis-job-87e06d39-5fbe-454c-a521-91800890169f ntpd[1863]: peers refreshed
Aug  7 10:40:04 travis-job-87e06d39-5fbe-454c-a521-91800890169f ntpd[1863]: new interface(s) found: waking up resolver
Aug  7 10:40:15 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [  204.493979] docker0: port 1(vetheca2bfa) entered forwarding state
Aug  7 11:17:01 travis-job-87e06d39-5fbe-454c-a521-91800890169f CRON[20008]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  7 11:23:20 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [ 2788.856603] traps: a[5617] trap invalid opcode ip:555ab88f8c1b sp:7fff60e15e10 error:0 in a[555ab88f5000+6000]
Aug  7 11:23:33 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [ 2802.551626] traps: a[8614] trap invalid opcode ip:7f6c2af61ef1 sp:7ffd48a41780 error:0 in libstd-e054c7a28f8831a7.so[7f6c2af06000+172000]
Aug  7 11:23:33 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [ 2802.579544] traps: a[8617] trap invalid opcode ip:7f30f256def1 sp:7ffc1ed0c330 error:0 in libstd-e054c7a28f8831a7.so[7f30f2512000+172000]
Aug  7 11:24:50 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [ 2878.924119] traps: a[24371] trap invalid opcode ip:55dc0dc6ee68 sp:7ffe993d63b0 error:0 in a[55dc0dc6c000+4000]
Aug  7 11:27:24 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [ 3033.048310] a[22208]: segfault at 0 ip 00005625ef339658 sp 00007ffd5b167d00 error 6 in a[5625ef336000+5000]
Aug  7 11:27:32 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [ 3041.294840] a[22985]: segfault at 1 ip 0000560a0d364c6c sp 00007ffdcc9305c0 error 6 in a[560a0d362000+4000]
Aug  7 11:27:36 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [ 3044.945548] traps: a[23378] trap invalid opcode ip:561cb24ab5bc sp:7fffb13335d0 error:0 in a[561cb24a8000+7000]
Aug  7 11:30:36 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [ 3225.045823] veth1c4f395: renamed from eth0
Aug  7 11:30:36 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [ 3225.059806] docker0: port 1(vetheca2bfa) entered disabled state
Aug  7 11:30:36 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [ 3225.098521] docker0: port 1(vetheca2bfa) entered disabled state
Aug  7 11:30:36 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [ 3225.100328] device vetheca2bfa left promiscuous mode
Aug  7 11:30:36 travis-job-87e06d39-5fbe-454c-a521-91800890169f kernel: [ 3225.100332] docker0: port 1(vetheca2bfa) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:16d1351f
