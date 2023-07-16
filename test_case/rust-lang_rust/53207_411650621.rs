plain
[00:44:38] ....................................................................................................
[00:44:41] ....................................................................................................
[00:44:43] ....................................................................................................
[00:44:46] ....................................................................................................
[00:44:49] ..............iiiiiiiii.............................................................................
[00:44:55] ....................................................................................................
[00:44:58] ....................i...............................................................................
[00:45:01] ..............................i.....................................................................
[00:45:04] ....................................................................................................
---
[01:01:02] ....................................................................................................
[01:01:10] ....................................................................................................
[01:01:19] ...................................i................................................................
[01:01:25] ....................................................................................................
[01:01:32] ......................................................................................FF............
[01:01:39] ....................................................................................................
[01:01:46] ............................................................................................F.F.....
[01:01:52] ....................................................................................................
[01:01:59] .........................F.F...........................................................F.F..........
[01:02:05] ..................................................FF................................................
[01:02:12] ............FF..........................................................................FF..........
[01:02:18] ....................................................F.F.............................................
[01:02:32] ....................................................................................................
[01:02:38] ....................................................................................................
[01:02:45] ....................................................................................................
[01:02:53] ....................................................................................................
---
[01:03:20] ---- num/mod.rs - num::i16::rotate_right (line 191) stdout ----
[01:03:20] thread 'num/mod.rs - num::i16::rotate_right (line 191)' panicked at 'test executable failed:
[01:03:20] 
[01:03:20] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:03:20]   left: `-24573`,
[01:03:20]  right: `-24572`', num/mod.rs:7:1
[01:03:20] 
[01:03:20] ', librustdoc/test.rs:368:17
[01:03:20] 
[01:03:20] ---- num/mod.rs - num::i8::rotate_left (line 190) stdout ----
---
[01:03:20] ---- num/mod.rs - num::i8::rotate_right (line 191) stdout ----
[01:03:20] thread 'num/mod.rs - num::i8::rotate_right (line 191)' panicked at 'test executable failed:
[01:03:20] 
[01:03:20] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:03:20]   left: `-126`,
[01:03:20]  right: `-125`', num/mod.rs:7:1
[01:03:20] 
[01:03:20] ', librustdoc/test.rs:368:17
[01:03:20] 
[01:03:20] ---- num/mod.rs - num::u128::rotate_left (line 190) stdout ----
[01:03:20] ---- num/mod.rs - num::u128::rotate_left (line 190) stdout ----
[01:03:20] error[E0599]: no method named `rotate_left` found for type `{integer}` in the current scope
[01:03:20]  --> num/mod.rs:194:14
[01:03:20]   |
[01:03:20] 7 | assert_eq!(n.rotate_left(16), m);
[01:03:20] 
[01:03:20] thread 'num/mod.rs - num::u128::rotate_left (line 190)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:03:20] 
[01:03:20] ---- num/mod.rs - num::u128::rotate_right (line 191) stdout ----
[01:03:20] ---- num/mod.rs - num::u128::rotate_right (line 191) stdout ----
[01:03:20] error[E0599]: no method named `rotate_right` found for type `{integer}` in the current scope
[01:03:20]  --> num/mod.rs:195:14
[01:03:20]   |
[01:03:20] 7 | assert_eq!(n.rotate_right(16), m);
[01:03:20] 
[01:03:20] thread 'num/mod.rs - num::u128::rotate_right (line 191)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:03:20] 
[01:03:20] ---- num/mod.rs - num::u16::rotate_left (line 190) stdout ----
[01:03:20] ---- num/mod.rs - num::u16::rotate_left (line 190) stdout ----
[01:03:20] error[E0599]: no method named `rotate_left` found for type `{integer}` in the current scope
[01:03:20]  --> num/mod.rs:194:14
[01:03:20]   |
[01:03:20] 7 | assert_eq!(n.rotate_left(4), m);
[01:03:20] 
[01:03:20] thread 'num/mod.rs - num::u16::rotate_left (line 190)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:03:20] 
[01:03:20] ---- num/mod.rs - num::u16::rotate_right (line 191) stdout ----
[01:03:20] ---- num/mod.rs - num::u16::rotate_right (line 191) stdout ----
[01:03:20] error[E0599]: no method named `rotate_right` found for type `{integer}` in the current scope
[01:03:20]  --> num/mod.rs:195:14
[01:03:20]   |
[01:03:20] 7 | assert_eq!(n.rotate_right(4), m);
[01:03:20] 
[01:03:20] thread 'num/mod.rs - num::u16::rotate_right (line 191)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:03:20] 
[01:03:20] ---- num/mod.rs - num::u32::rotate_left (line 190) stdout ----
[01:03:20] ---- num/mod.rs - num::u32::rotate_left (line 190) stdout ----
[01:03:20] error[E0599]: no method named `rotate_left` found for type `{integer}` in the current scope
[01:03:20]  --> num/mod.rs:194:14
[01:03:20]   |
[01:03:20] 7 | assert_eq!(n.rotate_left(8), m);
[01:03:20] 
[01:03:20] thread 'num/mod.rs - num::u32::rotate_left (line 190)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:03:20] 
[01:03:20] ---- num/mod.rs - num::u32::rotate_right (line 191) stdout ----
[01:03:20] ---- num/mod.rs - num::u32::rotate_right (line 191) stdout ----
[01:03:20] error[E0599]: no method named `rotate_right` found for type `{integer}` in the current scope
[01:03:20]  --> num/mod.rs:195:14
[01:03:20]   |
[01:03:20] 7 | assert_eq!(n.rotate_right(8), m);
[01:03:20] 
[01:03:20] thread 'num/mod.rs - num::u32::rotate_right (line 191)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:03:20] 
[01:03:20] ---- num/mod.rs - num::u64::rotate_left (lindate && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0d943670
$ sudo tail -n 500 /var/log/syslog
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] kvm-clock: using sched offset of 1627660631 cycles
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] Zone ranges:
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000]   Device   empty
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] Movable zone start for each node
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] Early memory node ranges
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] Policy zone: Normal
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 coMB 8
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.371731] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.374912] Freeing SMP alternatives memory: 32K
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.384016] ftrace: allocating 32185 entries in 126 pages
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.434131] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.435544] smpboot: Max logical packages: 2
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.436804] x2apic enabled
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.438541] Switched APIC routing to physical x2apic.
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.442357] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.550110] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.551743] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.555136] x86: Booting SMP configuration:
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.555851] .... node  #0, CPUs:      #1
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.557010] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.561616]  #2
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.562061] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.567411]  #3
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.568050] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.572543] x86: Booted up 1 node, 4 CPUs
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.573397] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.575967] devtmpfs: initialized
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.580570] evm: security.selinux
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.581175] evm: security.SMACK64
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.581794] evm: security.SMACK64EXEC
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.582528] evm: security.SMACK64TRANSMUTE
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.583222] evm: security.SMACK64MMAP
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.583817] evm: security.ima
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.584372] evm: security.capability
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.585399] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.587352] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.588693] pinctrl core: initialized pinctrl subsystem
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.590100] RTC time:  5:20:05, date: 08/09/18
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.591717] NET: Registered protocol family 16
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.602142] cpuidle: using governor ladder
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.614146] cpuidle: using governor menu
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.615185] PCCT header not found.
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.615843] ACPI: bus type PCI registered
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.616511] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.617629] PCI: Using configuration type 1 for base access
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.631190] ACPI: Addedc653b87-fa23-4950-803d-a760ac53318d kernel: [    0.834052] SCSI subsystem initialized
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.835098] libata version 3.00 loaded.
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.835126] ACPI: bus type USB registered
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.835765] usbcore: registered new interface driver usbfs
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.836805] usbcore: registered new interface driver hub
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.837757] usbcore: registered new device driver usb
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.839040] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.840155] dmi: Firmware registration failed.
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.840928] PCI: Using ACPI for IRQ routing
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.841668] PCI: pci_cache_line_size set to 64 bytes
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.841772] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.841774] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.841885] NetLabel: Initializing
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.842694] NetLabel:  domain hash size = 128
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.843409] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.844255] NetLabel:  unlabeled traffic allowed by default
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.845200] amd_nb: Cannot enumerate AMD northbridges
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.846261] clocksource: Switched to clocksource kvm-clock
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.854437] pnp: PnP ACPI init
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.855000] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.855060] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.855101] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.855152] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.855194] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.855233] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.855270] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.855428] pnp: PnP ACPI: found 7 devices
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.863010] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.864674] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.864676] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.864678] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.864679] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.864716] NET: Registered protocol family 2
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.865695] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.867191] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.868637] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.869765] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.870898] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.872918] NET: Registered protocol family 1
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.873608] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.874701] PCI: CLS 0 bytes, default 64
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    0.874753] Unpacking initramfs...
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.005211] Freeing initrd memory: 21432K
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.006333] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.007304] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.009570] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.011270] hw unit of domain pp0-core 2^-0 Joules
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.012374] hw unit of domain package 2^-0 Joules
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.013338] hw unit of domain dram 2^-0 Joules
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.014308] Scanning for low memory corruption every 60 seconds
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.015879] audit: initializing netlink subsys (disabled)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.017130] audit: type=2000 audit(1533792007.543:1): initialized
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.018481] Initialise system trusted keyring
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.019626] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.020740] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.022965] zbud: loaded
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.023742] VFS: Disk quotas dquot_6.6.0
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.024538] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.025884] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.027426] fuse init (API version 7.23)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.028364] Key type big_key registered
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.028987] Allocating IMA MOK and blacklist keyrings.
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.030919] Key type asymmetric registered
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.032043] Asymmetric key parser 'x509' registered
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.032899] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.034147] io scheduler noop registered
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.035059] io scheduler deadline registered (default)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.036031] io scheduler cfq registered
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.036719] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.038289] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.039813] intel_idle: does not run on family 6 model 62
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.039927] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.041501] ACPI: Power Button [PWRF]
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.042287] input: Sleep 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.245547] NET: Registered protocol family 10
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.246891] NET: Registered protocol family 17
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.248074] Key type dns_resolver registered
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.249333] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.251118] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.254625] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.256018] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.257501] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.260011] registered taskstats version 1
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.261055] Loading compiled-in X.509 certificates
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.262607] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.264495] zswap: loaded using pool lzo/zbud
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.267258] Key type trusted registered
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.271856] Key type encrypted registered
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.273260] ima: No TPM chip found, activating TPM-bypass!
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.274722] evm: HMAC attrs: 0x1
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.275854]   Magic number: 14:486:314
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.276827] rtc_cmos 00:00: setting system clock to 2018-08-09 05:20:08 UTC (1533792008)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.278852] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.280039] EDD information not available.
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.280905] PM: Hibernation image not present or could not be loaded.
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.282295] Freeing unused kernel memory: 1496K
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.283209] Write protecting the kernel read-only data: 14336k
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.285343] Freeing unused kernel memory: 1956K
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.286432] Freeing unused kernel memory: 92K
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.302162] systemd-udevd[118]: starting version 204
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.362290] scsi host0: Virtio SCSI HBA
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.368825] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.370691] AVX version of gcm_enc/dec engaged.
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.371421] AES CTR mode by8 optimization enabled
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.409969] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.410000] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.412283] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.413892] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.414587] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.414808] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.417966]  sda: sda1
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.419453] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    3.431039] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    4.010401] tsc: Refined TSC clocksource calibration: 2499.757 MHz
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    4.011774] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2408541cd11, max_idle_ns: 440795240685 ns
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    4.268425] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    6.350477] floppy0: no floppy controllers found
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    7.534311] raid6: sse2x1   gen()  8993 MB/s
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    7.602290] raid6: sse2x1   xor()  6913 MB/s
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    7.670302] raid6: sse2x2   gen() 11344 MB/s
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    7.738308] raid6: sse2x2   xor()  7800 MB/s
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    7.806299] raid6: sse2x4   gen() 12687 MB/s
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    7.874316] raid6: sse2x4   xor()  8478 MB/s
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    7.875084] raid6: using algorithm sse2x4 gen() 12687 MB/s
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    7.876276] raid6: .... xor() 8478 MB/s, rmw enabled
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    7.877013] raid6: using ssse3x2 recovery algorithm
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    7.879589] xor: automatically using best checksumming function:
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    7.918277]    avx       : 22289.000 MB/sec
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    7.932075] Btrfs loaded
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    7.973786] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    7.975099] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    8.036694] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    8.048688] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    8.049621] EXT4-fs (sda1): recovery complete
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    8.054132] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    8.245059] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    8.355991] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    8.403893] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    8.602557] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    9.142832] random: cloud-init: uninitialized urandom read (32 bytes read, 44 bits of entropy available)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    9.274803] systemd-udevd[701]: starting version 204
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    9.372097] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    9.476686] ppdev: user-space parallel port driver
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    9.581884] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    9.632466] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug  9 05:20:15 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [    9.698437] random: cloud-03d-a760ac53318d google-accounts: INFO Creating a new user account for henrik.
Aug  9 05:20:16 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [   11.424661] random: nonblocking pool is initialized
Aug  9 05:20:16 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Created user account henrik.
Aug  9 05:20:16 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  9 05:20:16 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Creating a new user account for emma.
Aug  9 05:20:16 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Created user account emma.
Aug  9 05:20:16 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Creating a new user account for igor.
Aug  9 05:20:16 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Created user account igor.
Aug  9 05:20:16 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  9 05:20:16 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Created user account konstantinhaase.
Aug  9 05:20:16 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Creating a new user account for aj.
Aug  9 05:20:16 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Created user account aj.
Aug  9 05:20:16 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Creating a new user account for solarce.
Aug  9 05:20:16 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Created user account solarce.
Aug  9 05:20:16 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Creating a new user account for asari.
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Created user account asari.
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Creating a new user account for bogdana.
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d cron[1465]: (CRON) INFO (pidfile fd = 3)
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d cron[1516]: (CRON) STARTUP (fork ok)
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d cron[1516]: (CRON) INFO (Running @reboot jobs)
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d acpid: starting up with netlink and the input layer
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d acpid: 1 rule loaded
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d acpid: waiting for events: event logging is off
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Created user account bogdana.
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Creating a new user account for konstantin.
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d haveged: haveged starting up
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Created user account konstantin.
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Creating a new user account for carmen.
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [   12.011816] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [   12.023267] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [   12.034105] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [   12.039570] Bridge firewalling registered
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Created user account carmen.
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [   12.056596] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Creating a new user account for maria.
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Created user account maria.
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [   12.117747] Initializing XFRM netlink socket
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [   12.124245] Netfilter messages via NETLINK v0.30.
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-accounts: INFO Removing user packer.
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [   12.127175] ctnetlink v0.93: registering with nfnetlink.
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 05:20:17 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [   12.454437] floppy0: no floppy controllers found
Aug  9 05:20:40 travis-job-1c653b87-fa23-4950-803d-a760ac53318d ntpdate[1857]: adjust time server 169.254.169.254 offset 0.016757 sec
Aug  9 05:20:46 travis-job-1c653b87-fa23-4950-803d-a760ac53318d ntpd[1892]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 05:20:46 travis-job-1c653b87-fa23-4950-803d-a760ac53318d ntpd[1893]: proto: precision = 0.105 usec
Aug  9 05:20:46 travis-job-1c653b87-fa23-4950-803d-a760ac53318d ntpd[1893]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 05:20:46 travis-job-1c653b87-fa23-4950-803d-a760ac53318d ntpd[1893]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 05:20:46 travis-job-1c653b87-fa23-4950-803d-a760ac53318d ntpd[1893]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 05:20:46 travis-job-1c653b87-fa23-4950-803d-a760ac53318d ntpd[1893]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 05:20:46 travis-job-1c653b87-fa23-4vis-job-1c653b87-fa23-4950-803d-a760ac53318d ntpd[1893]: peers refreshed
Aug  9 05:23:11 travis-job-1c653b87-fa23-4950-803d-a760ac53318d ntpd[1893]: new interface(s) found: waking up resolver
Aug  9 05:23:23 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [  198.428110] docker0: port 1(veth26fe263) entered forwarding state
Aug  9 06:07:19 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [ 2834.848438] traps: a[5398] trap invalid opcode ip:563212863b1b sp:7ffde0c173a0 error:0 in a[563212860000+6000]
Aug  9 06:07:34 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [ 2849.098970] traps: a[8231] trap invalid opcode ip:7f56dfa8dea1 sp:7ffd3d281a20 error:0 in libstd-2339b911e3c09de8.so[7f56dfa33000+172000]
Aug  9 06:07:34 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [ 2849.134050] traps: a[8233] trap invalid opcode ip:7fd91c1c7ea1 sp:7ffffc089f90 error:0 in libstd-2339b911e3c09de8.so[7fd91c16d000+172000]
Aug  9 06:08:54 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [ 2929.274914] traps: a[23140] trap invalid opcode ip:55c5678b5d68 sp:7fffd081abe0 error:0 in a[55c5678b3000+4000]
Aug  9 06:11:33 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [ 3088.881696] a[19344]: segfault at 0 ip 000055dfb73b1548 sp 00007ffe464657a0 error 6 in a[55dfb73ae000+5000]
Aug  9 06:11:42 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [ 3097.359392] a[20086]: segfault at 1 ip 0000556ec906ab5c sp 00007ffd7f182340 error 6 in a[556ec9068000+4000]
Aug  9 06:11:46 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [ 3101.062558] traps: a[20461] trap invalid opcode ip:558b134f742c sp:7ffc038c9010 error:0 in a[558b134f4000+7000]
Aug  9 06:17:01 travis-job-1c653b87-fa23-4950-803d-a760ac53318d CRON[18223]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  9 06:25:01 travis-job-1c653b87-fa23-4950-803d-a760ac53318d CRON[20568]: (root) CMD (test -x /usr/sbin/anacron || ( cd / && run-parts --report /etc/cron.daily ))
Aug  9 06:25:26 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [ 3921.437464] veth66af92d: renamed from eth0
Aug  9 06:25:26 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [ 3921.455297] docker0: port 1(veth26fe263) entered disabled state
Aug  9 06:25:26 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [ 3921.506416] docker0: port 1(veth26fe263) entered disabled state
Aug  9 06:25:26 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [ 3921.508198] device veth26fe263 left promiscuous mode
Aug  9 06:25:26 travis-job-1c653b87-fa23-4950-803d-a760ac53318d kernel: [ 3921.508200] docker0: port 1(veth26fe263) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:22ad9699
