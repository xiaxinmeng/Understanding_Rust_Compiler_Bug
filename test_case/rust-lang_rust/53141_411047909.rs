plain
[01:02:22] 
[01:02:22] error: infinite iteration detected
[01:02:22]   --> $DIR/infinite_iter.rs:14:5
[01:02:22]    |
[01:02:22] 14 |     (0..8_u32).rev().cycle().map(|x| x + 1_u32).for_each(|x| println!("{}", x)); // infinite iter
[01:02:22] 
[01:02:22] error: infinite iteration detected
[01:02:22]   --> $DIR/infinite_iter.rs:16:5
[01:02:22]    |
---
[01:02:22] 
[01:02:22] error: possible infinite iteration detected
[01:02:22]   --> $DIR/infinite_iter.rs:26:5
[01:02:22]    |
[01:02:22] 26 |     (1..).scan(0, |state, x| { *state += x; Some(*state) }).min(); // maybe infinite iter
[01:02:22] 
[01:02:22] error: possible infinite iteration detected
[01:02:22]   --> $DIR/infinite_iter.rs:27:5
[01:02:22]    |
---
[01:02:22] 
[01:02:22] error: infinite iteration detected
[01:02:22]   --> $DIR/infinite_iter.rs:14:5
[01:02:22]    |
[01:02:22] 14 |     (0..8_u32).rev().cycle().map(|x| x + 1_u32).for_each(|x| println!("{}", x)); // infinite iter
[01:02:22] 
[01:02:22] error: infinite iteration detected
[01:02:22]   --> $DIR/infinite_iter.rs:16:5
[01:02:22]    |
---
[01:02:22] 
[01:02:22] error: possible infinite iteration detected
[01:02:22]   --> $DIR/infinite_iter.rs:26:5
[01:02:22]    |
[01:02:22] 26 |     (1..).scan(0, |state, x| { *state += x; Some(*state) }).min(); // maybe infinite iter
[01:02:22] 
[01:02:22] error: possible infinite iteration detected
[01:02:22]   --> $DIR/infinite_iter.rs:27:5
[01:02:22]    |
---
[01:02:22]  
[01:02:22]  error: infinite iteration detected
[01:02:22]    --> $DIR/infinite_iter.rs:14:5
[01:02:22]     |
[01:02:22]  14 |     (0..8_u32).rev().cycle().map(|x| x + 1_u32).for_each(|x| println!("{}", x)); // infinite iter
[01:02:22]  
[01:02:22]  error: infinite iteration detected
[01:02:22]    --> $DIR/infinite_iter.rs:16:5
[01:02:22]     |
---
[01:02:22]  
[01:02:22]  error: possible infinite iteration detected
[01:02:22]    --> $DIR/infinite_iter.rs:26:5
[01:02:22]     |
[01:02:22]  26 |     (1..).scan(0, |state, x| { *state += x; Some(*state) }).min(); // maybe infinite iter
[01:02:22]  
[01:02:22]  error: possible infinite iteration detected
[01:02:22]    --> $DIR/infinite_iter.rs:27:5
[01:02:22]     |
---
[01:02:22] 
[01:02:22] error: infinite iteration detected
[01:02:22]   --> tests/ui/infinite_iter.rs:14:5
[01:02:22]    |
[01:02:22] 14 |     (0..8_u32).rev().cycle().map(|x| x + 1_u32).for_each(|x| println!("{}", x)); // infinite iter
[01:02:22] 
[01:02:22] error: infinite iteration detected
[01:02:22]   --> tests/ui/infinite_iter.rs:16:5
[01:02:22]    |
---
[01:02:22] 
[01:02:22] error: possible infinite iteration detected
[01:02:22]   --> tests/ui/infinite_iter.rs:26:5
[01:02:22]    |
[01:02:22] 26 |     (1..).scan(0, |state, x| { *state += x; Some(*state) }).min(); // maybe infinite iter
[01:02:22] 
[01:02:22] error: possible infinite iteration detected
[01:02:22]   --> tests/ui/infinite_iter.rs:27:5
[01:02:22]    |
---
[01:26:30] This PR updated 'src/tools/rustfmt', verifying if status is 'test-pass'...
[01:26:30] Verifying status of clippy-driver...
[01:26:30] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:26:30] 
[01:26:30] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:26:30] 
[01:26:30] If you do intend to update 'clippy-driver', please check the error messages above and
[01:26:30] commit another update.
[01:26:30] 
[01:26:30] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:26:30] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:26:30] proper steps.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:239fed00
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0036f26d
$ sudo tail -n 500 /var/log/syslog
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] kvm-clock: using sched offset of 1530272415 cycles
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] Zone ranges:
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000]   Device   empty
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] Movable zone start for each node
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] Early memory node ranges
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] Policy zone: Normal
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] console [ttyS0] enabled
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.347488] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.348857] pid_max: default: 32768 minimum: 301
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.349672] ACPI: Core revision 20150930
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.356306] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.357844] Security Framework initialized
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.358510] Yama: becoming mindful.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.359436] AppArmor: AppArmor disabled by boot time parameter
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.362110] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.372267] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.377376] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.378712] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.380529] Initializing cgroup subsys io
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.381188] Initializing cgroup subsys memory
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.382002] Initializing cgroup subsys devices
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.383023] Initializing cgroup subsys freezer
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.384569] Initializing cgroup subsys net_cls
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.385190] Initializing cgroup subsys perf_event
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.385900] Initializing cgroup subsys net_prio
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.386861] Initializing cgroup subsys hugetlb
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.387620] Initializing cgroup subsys pids
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.388502] CPU: Physical Processor ID: 0
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.389371] CPU: Processor Core ID: 0
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.391354] mce: CPU supports 32 MCE banks
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.392231] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.393072] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.396224] Freeing SMP alternatives memory: 32K
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.405133] ftrace: allocating 32185 entries in 126 pages
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.454430] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.455556] smpboot: Max logical packages: 2
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.456758] x2apic enabled
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.459266] Switched APIC routing to physical x2apic.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.463069] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.570755] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.572276] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.575894] x86: Booting SMP configuration:
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.576505] .... node  #0, CPUs:      #1
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.577420] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.581988]  #2
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.582495] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.587085]  #3
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.587545] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.592094] x86: Booted up 1 node, 4 CPUs
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.592806] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.596088] devtmpfs: initialized
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.600654] evm: security.selinux
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.601264] evm: security.SMACK64
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.601744] evm: security.SMACK64EXEC
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.602254] evm: security.SMACK64TRANSMUTE
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.602841] evm: security.SMACK64MMAP
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.603419] evm: security.ima
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.603839] evm: security.capability
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.604870] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.606376] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.607842] pinctrl core: initialized pinctrl subsystem
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.608818] RTC time: 11:28:44, date: 08/07/18
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.610319] NET: Registered protocol family 16
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.622800] cpuidle: using governor ladder
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.634791] cpuidle: using governor menu
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.635678] PCCT header not found.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.636490] ACPI: bus type PCI registered
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.637299] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.638546] PCI: Using configuration type 1 for base access
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.651978] ACPI: Added _OSI(Module Device)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.652947] ACPI: Added _OSI(Processor Device)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.653611] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.654368] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.657769] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.681379] ACPI: Interpreter enabled
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.682131] ACPI: (supports S0 S3 S4 S5)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.682825] ACPI: Using IOAPIC for interrupt routing
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.683552] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.713468] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.714636] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.715706] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.716830] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.719393] PCI host bridge to bus 0000:00
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.720067] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.721003] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.721977] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.723369] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.724429] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.725325] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.725768] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.740327] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.756392] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.757897] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.764196] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.770164] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.785817] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.792052] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.796916] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.811787] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.814287] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.817020] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.819700] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.822056] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.843059] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.844198] vgaarb: loaded
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.844854] SCSI subsystem initialized
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.845548] libata version 3.00 loaded.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.845610] ACPI: bus type USB registered
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.846436] usbcore: registered new interface driver usbfs
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.847501] usbcore: registered new interface driver hub
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.848397] usbcore: registered new device driver usb
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.849416] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.850678] dmi: Firmware registration failed.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.851528] PCI: Using ACPI for IRQ routing
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.852209] PCI: pci_cache_line_size set to 64 bytes
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.852315] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.852316] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.852482] NetLabel: Initializing
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.853102] NetLabel:  domain hash size = 128
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.854019] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.854875] NetLabel:  unlabeled traffic allowed by default
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.855905] amd_nb: Cannot enumerate AMD northbridges
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.856733] clocksource: Switched to clocksource kvm-clock
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.865485] pnp: PnP ACPI init
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.866272] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.866338] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.866381] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.866429] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.866468] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.866506] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.866544] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.866720] pnp: PnP ACPI: found 7 devices
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.874072] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.875593] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.875596] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.875598] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.875599] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.875646] NET: Registered protocol family 2
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.876508] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.877772] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.879135] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.880192] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.881203] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.882642] NET: Registered protocol family 1
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.883542] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.884494] PCI: CLS 0 bytes, default 64
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    0.885331] Unpacking initramfs...
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.005122] Freeing initrd memory: 21432K
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.006144] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.007232] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.010289] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.012427] hw unit of domain pp0-core 2^-0 Joules
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.013669] hw unit of domain package 2^-0 Joules
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.014656] hw unit of domain dram 2^-0 Joules
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.015511] Scanning for low memory corruption every 60 seconds
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.017714] audit: initializing netlink subsys (disabled)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.019356] audit: type=2000 audit(1533641326.275:1): initialized
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.020984] Initialise system trusted keyring
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.022722] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.024009] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.026522] zbud: loaded
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.027465] VFS: Disk quotas dquot_6.6.0
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.028258] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.029591] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.030929] fuse init (API version 7.23)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.032000] Key type big_key registered
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.032704] Allocating IMA MOK and blacklist keyrings.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.035196] Key type asymmetric registered
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.036111] Asymmetric key parser 'x509' registered
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.036936] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.038169] io scheduler noop registered
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.038836] io scheduler deadline registered (default)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.039952] io scheduler cfq registered
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.041372] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.042234] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.043492] intel_idle: does not run on family 6 model 62
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.043615] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.044869] ACPI: Power Button [PWRF]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.045896] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.047083] ACPI: Sleep Button [SLPF]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.048258] GHES: HEST is not enabled!
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.050900] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.052195] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.057267] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.058826] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.063892] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.087274] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.111309] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.134940] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.158392] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.162318] Linux agpgart interface v0.103
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.165619] loop: module loaded
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.166741] libphy: Fixed MDIO Bus: probed
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.167918] tun: Universal TUN/TAP device driver, 1.6
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.169651] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.209515] PPP generic driver version 2.4.2
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.210865] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.212861] ehci-pci: EHCI PCI platform driver
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.214074] ehci-platform: EHCI generic platform driver
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.215137] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.216901] ohci-pci: OHCI PCI platform driver
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.218418] ohci-platform: OHCI generic platform driver
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.219699] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.221565] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.224312] i8042: Warning: Keylock active
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.226023] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.227432] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.228564] mousedev: PS/2 mouse device common for all mice
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.230445] rtc_cmos 00:00: RTC can wake from S4
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.231896] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.233467] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.234966] i2c /dev entries driver
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.235716] device-mapper: uevent: version 1.0.3
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.237011] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.239120] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.241323] NET: Registered protocol family 10
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.242660] NET: Registered protocol family 17
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.243543] Key type dns_resolver registered
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.245087] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.246635] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.247816] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.249354] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.250993] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.253044] registered taskstats version 1
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.253820] Loading compiled-in X.509 certificates
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.255476] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.257859] zswap: loaded using pool lzo/zbud
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.260878] Key type trusted registered
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.265447] Key type encrypted registered
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.266307] ima: No TPM chip found, activating TPM-bypass!
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.267730] evm: HMAC attrs: 0x1
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.269016]   Magic number: 14:628:478
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.269901] rtc rtc0: hash matches
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.271135] rtc_cmos 00:00: setting system clock to 2018-08-07 11:28:46 UTC (1533641326)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.273343] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.274847] EDD information not available.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.276186] PM: Hibernation image not present or could not be loaded.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.277905] Freeing unused kernel memory: 1496K
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.278703] Write protecting the kernel read-only data: 14336k
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.280860] Freeing unused kernel memory: 1956K
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.282204] Freeing unused kernel memory: 92K
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.297793] systemd-udevd[119]: starting version 204
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.360342] scsi host0: Virtio SCSI HBA
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.364648] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.368867] AVX version of gcm_enc/dec engaged.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.369684] AES CTR mode by8 optimization enabled
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.402000] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.402002] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.404945] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.406536] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.407578] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.407710] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.411671]  sda: sda1
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.413112] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    3.429121] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    4.012880] tsc: Refined TSC clocksource calibration: 2499.784 MHz
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    4.014009] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24086dc9c08, max_idle_ns: 440795236697 ns
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    4.261996] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    6.356953] floppy0: no floppy controllers found
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    7.528751] raid6: sse2x1   gen()  9149 MB/s
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    7.596747] raid6: sse2x1   xor()  6861 MB/s
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    7.664748] raid6: sse2x2   gen() 11470 MB/s
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    7.732749] raid6: sse2x2   xor()  7889 MB/s
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    7.800755] raid6: sse2x4   gen() 12691 MB/s
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    7.868751] raid6: sse2x4   xor()  8886 MB/s
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    7.870052] raid6: using algorithm sse2x4 gen() 12691 MB/s
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    7.871195] raid6: .... xor() 8886 MB/s, rmw enabled
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    7.872901] raid6: using ssse3x2 recovery algorithm
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    7.875426] xor: automatically using best checksumming function:
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    7.912744]    avx       : 22157.000 MB/sec
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    7.926545] Btrfs loaded
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    7.965520] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    7.967503] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    8.041892] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    8.054651] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    8.055649] EXT4-fs (sda1): recovery complete
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    8.060604] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    8.260656] random: init: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    8.378850] random: mountall: uninitialized urandom read (12 bytes read, 33 bits of entropy available)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    8.427690] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    8.622374] random: cloud-init: uninitialized urandom read (32 bytes read, 41 bits of entropy available)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    9.193790] random: cloud-init: uninitialized urandom read (32 bytes read, 49 bits of entropy available)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    9.334720] systemd-udevd[705]: starting version 204
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    9.445875] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    9.552787] ppdev: user-space parallel port driver
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    9.663583] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    9.716638] random: mktemp: uninitialized urandom read (6 bytes read, 62 bits of entropy available)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    9.790025] random: cloud-init: uninitialized urandom read (32 bytes read, 62 bits of entropy available)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [    9.951626] random: cloud-init: uninitialized urandom read (32 bytes read, 62 bits of entropy available)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [   10.218424] random: mktemp: uninitialized urandom read (12 bytes read, 65 bits of entropy available)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [   10.290511] random: mktemp: uninitialized urandom read (6 bytes read, 66 bits of entropy available)
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [   10.365355] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [   10.400359] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [   10.993984] init: failsafe main process (1096) killed by TERM signal
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO Running set_multiqueue.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO Set channels for eth0 to 4.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Starting Google Accounts daemon.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [   11.662039] random: nonblocking pool is initialized
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Creating a new user account for me.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Created user account me.
Aug  7 11:28:54 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Creating a new user account for henrik.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Created user account henrik.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Creating a new user account for emma.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Created user account emma.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Creating a new user account for igor.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Created user account igor.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Created user account konstantinhaase.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Creating a new user account for aj.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Created user account aj.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a cron[1449]: (CRON) INFO (pidfile fd = 3)
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Creating a new user account for solarce.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a cron[1485]: (CRON) STARTUP (fork ok)
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a cron[1485]: (CRON) INFO (Running @reboot jobs)
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a acpid: starting up with netlink and the input layer
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a acpid: 1 rule loaded
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a acpid: waiting for events: event logging is off
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Created user account solarce.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Creating a new user account for asari.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Created user account asari.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a haveged: haveged starting up
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Creating a new user account for bogdana.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Created user account bogdana.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [   12.254470] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [   12.264377] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Creating a new user account for konstantin.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Created user account konstantin.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Creating a new user account for carmen.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Created user account carmen.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [   12.365424] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [   12.368848] Bridge firewalling registered
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Creating a new user account for maria.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [   12.383129] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Created user account maria.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-accounts: INFO Removing user packer.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [   12.452558] Initializing XFRM netlink socket
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [   12.459014] Netfilter messages via NETLINK v0.30.
Aug  7 11:28:55 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [   12.462512] ctnetlink v0.93: registering with nfnetlink.
Aug  7 11:28:56 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 11:28:56 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [   12.516801] floppy0: no floppy controllers found
Aug  7 11:29:18 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpdate[1858]: adjust time server 169.254.169.254 offset 0.004857 sec
Aug  7 11:29:25 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpd[1891]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 11:29:25 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpd[1894]: proto: precision = 0.102 usec
Aug  7 11:29:25 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpd[1894]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 11:29:25 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpd[1894]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 11:29:25 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpd[1894]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 11:29:25 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpd[1894]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  7 11:29:25 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpd[1894]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 11:29:25 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpd[1894]: Listen normally on 3 eth0 10.20.1.110 UDP 123
Aug  7 11:29:25 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpd[1894]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 11:29:25 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpd[1894]: peers refreshed
Aug  7 11:29:25 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpd[1894]: Listening on routing socket on fd #21 for interface updates
Aug  7 11:29:25 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [   42.432559] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 11:29:26 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a startup-script: INFO Found startup-script in metadata.
Aug  7 11:29:26 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 11:29:26 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a startup-script: INFO startup-script: job 1 at Tue Aug  7 14:39:00 2018
Aug  7 11:29:26 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a startup-script: INFO startup-script: Return code 0.
Aug  7 11:29:26 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a startup-script: INFO startup-script: Return code 0.
Aug  7 11:29:26 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a startup-script: INFO Finished running startup scripts.
Aug  7 11:29:26 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ec2: 
Aug  7 11:29:26 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ec2: #############################################################
Aug  7 11:29:26 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 11:29:26 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ec2: 1024 86:9a:9e:ff:8c:3d:8f:8c:aa:ca:30:b8:c8:b7:6b:ee  root@travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a (DSA)
Aug  7 11:29:26 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ec2: 256 89:54:04:74:b4:dc:55:22:39:1a:85:3b:09:55:97:b3  root@travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a (ECDSA)
Aug  7 11:29:26 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ec2: 256 7e:a9:56:28:a2:46:6a:e6:8f:c4:f6:13:a4:ca:a7:ae  root@travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a (ED25519)
Aug  7 11:29:26 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ec2: 2048 40:6a:e2:7e:b0:de:e7:53:59:3a:3f:27:18:62:94:23  root@travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a (RSA)
Aug  7 11:29:26 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 11:29:26 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ec2: #############################################################
Aug  7 11:30:46 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [  122.814359] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 11:31:33 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [  170.338261] device vetha439a70 entered promiscuous mode
Aug  7 11:31:33 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [  170.338319] docker0: port 1(vetha439a70) entered forwarding state
Aug  7 11:31:33 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [  170.338331] docker0: port 1(vetha439a70) entered forwarding state
Aug  7 11:31:33 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [  170.338692] docker0: port 1(vetha439a70) entered disabled state
Aug  7 11:31:33 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [  170.423306] cgroup: docker-runc (4908) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 11:31:33 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [  170.423309] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 11:31:34 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [  170.503134] eth0: renamed from vethae6a3ee
Aug  7 11:31:34 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [  170.547372] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 11:31:34 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [  170.548664] docker0: port 1(vetha439a70) entered forwarding state
Aug  7 11:31:34 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [  170.548679] docker0: port 1(vetha439a70) entered forwarding state
Aug  7 11:31:34 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [  170.548696] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 11:31:37 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpd[1894]: Listen normally on 5 docker0 fe80::42:29ff:fe95:a99b UDP 123
Aug  7 11:31:37 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpd[1894]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  7 11:31:37 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpd[1894]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 11:31:37 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpd[1894]: peers refreshed
Aug  7 11:31:37 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a ntpd[1894]: new interface(s) found: waking up resolver
Aug  7 11:31:49 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a kernel: [  185.590406] docker0: port 1(vetha439a70) entered forwarding state
Aug  7 12:17:01 travis-job-f3f55c1f-3c32-4f74-a98f-a652d895165a CRON[2873]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
---
travis_time:end:3ada1dd8:start=1533646639939529141,finish=1533646639948642913,duration=9113772
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c7edb68
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14d43a28
travis_time:start:14d43a28
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0268f454
$ dmesg | grep -i kill
