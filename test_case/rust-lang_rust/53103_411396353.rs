plain
[00:47:34] ....................................................................................................
[00:47:37] ....................................................................................................
[00:47:39] ....................................................................................................
[00:47:43] ....................................................................................................
[00:47:45] .............iiiiiiiii..............................................................................
[00:47:51] ....................................................................................................
[00:47:55] ...................i................................................................................
[00:47:58] ............................i.......................................................................
[00:48:01] ....................................................................................................
---
[01:03:31] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:31]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:03:33] error[E0530]: let bindings cannot shadow constants
[01:03:33]    --> libcore/../libcore/tests/nonzero.rs:131:13
[01:03:33]     |
[01:03:33] 129 |     const n: u32 = nonzero_n.get();
[01:03:33]     |     ------------------------------- a constant `n` is defined here
[01:03:33] 130 | 
[01:03:33] 131 |     let mut n: u32 = n;
[01:03:33]     |             ^ cannot be named the same as a constant
[01:03:33] 
[01:03:33] error[E0434]: can't capture dynamic environment in a fn item
[01:03:33]    --> libcore/../libcore/tests/nonzero.rs:132:61
[01:03:33]     |
[01:03:33] 132 |     const nonnull_p: NonNull<u32> = NonNull::new_unchecked(&n);
[01:03:33]     |
[01:03:33]     |
[01:03:33]     = help: use the `|| { ... }` closure form instead
[01:03:33] 
[01:03:35] error[E0599]: no method named `get` found for type `std::ptr::NonNull<u32>` in the current scope
[01:03:35]    --> libcore/../libcore/tests/nonzero.rs:133:35
[01:03:35]     |
[01:03:35] 133 |     const p: *mut u32 = nonnull_p.get();
[01:03:35] 
[01:03:36] error: aborting due to 3 previous errors
[01:03:36] 
[01:03:36] Some errors occurred: E0434, E0530, E0599.
[01:03:36] Some errors occurred: E0434, E0530, E0599.
[01:03:36] For more information about an error, try `rustc --explain E0434`.
[01:03:36] error: Could not compile `core`.
[01:03:36] 
[01:03:36] To learn more, run the command again with --verbose.
[01:03:36] 
[01:03:36] 
[01:03:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:03:36] 
[01:03:36] 
[01:03:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:36] Build completed unsuccessfully in 0:18:48
[01:03:36] Build completed unsuccessfully in 0:18:48
[01:03:36] make: *** [check] Error 1
[01:03:36] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09a73d08
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:1f2627fc
$ sudo tail -n 500 /var/log/syslog
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Using GB pages for direct mapping
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] kvm-clock: using sched offset of 1723994975 cycles
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Zone ranges:
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000]   Device   empty
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Movable zone start for each node
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Early memory node ranges
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Policy zone: Normal
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Hierarchical RCU implementation.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] console [ttyS0] enabled
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.512668] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.516164] pid_max: default: 32768 minimum: 301
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.517854] ACPI: Core revision 20150930
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.525819] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.528890] Security Framework initialized
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.530717] Yama: becoming mindful.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.532158] AppArmor: AppArmor disabled by boot time parameter
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.535922] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.548887] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.555878] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.558779] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.562393] Initializing cgroup subsys io
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.563691] Initializing cgroup subsys memory
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.565352] Initializing cgroup subsys devices
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.567056] Initializing cgroup subsys freezer
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.568911] Initializing cgroup subsys net_cls
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.570439] Initializing cgroup subsys perf_event
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.572110] Initializing cgroup subsys net_prio
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.575352] Initializing cgroup subsys hugetlb
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.577318] Initializing cgroup subsys pids
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.579479] CPU: Physical Processor ID: 0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.581217] CPU: Processor Core ID: 0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.584009] mce: CPU supports 32 MCE banks
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.586111] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.588742] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.594108] Freeing SMP alternatives memory: 32K
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.605361] ftrace: allocating 32185 entries in 126 pages
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.658830] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.661762] smpboot: Max logical packages: 2
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.664002] x2apic enabled
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.667366] Switched APIC routing to physical x2apic.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.673201] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.782339] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.786253] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.791538] x86: Booting SMP configuration:
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.793447] .... node  #0, CPUs:      #1
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.796201] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.802387]  #2
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.803092] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.808899]  #3
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.809907] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.815746] x86: Booted up 1 node, 4 CPUs
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.817563] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.822005] devtmpfs: initialized
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.827734] evm: security.selinux
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.828900] evm: security.SMACK64
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.830696] evm: security.SMACK64EXEC
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.832048] evm: security.SMACK64TRANSMUTE
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.833799] evm: security.SMACK64MMAP
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.835269] evm: security.ima
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.836337] evm: security.capability
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.838209] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.843091] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.845912] pinctrl core: initialized pinctrl subsystem
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.848745] RTC time: 11:54:15, date: 08/08/18
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.851523] NET: Registered protocol family 16
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.866395] cpuidle: using governor ladder
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.878376] cpuidle: using governor menu
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.879649] PCCT header not found.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.881029] ACPI: bus type PCI registered
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.882781] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.885566] PCI: Using configuration type 1 for base access
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.900010] ACPI: Added _OSI(Module Device)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.902082] ACPI: Added _OSI(Processor Device)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.903952] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.905725] ACPI: Added _OSI(Processor Aggregator Device)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.910928] ACPI: Executed 2 blocks of module-level executable AML code
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.936939] ACPI: Interpreter enabled
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.938536] ACPI: (supports S0 S3 S4 S5)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.940002] ACPI: Using IOAPIC for interrupt routing
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.942145] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.976271] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.979302] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.981584] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.983828] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.989941] PCI host bridge to bus 0000:00
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.991461] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.994650] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.997162] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    0.999787] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.002299] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.004146] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.004606] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.040881] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.069812] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.073004] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.084428] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.092688] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.118596] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.128170] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.136438] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.161985] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.166960] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.171421] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.176634] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.181626] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.205140] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.207626] vgaarb: loaded
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.209059] SCSI subsystem initialized
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.211048] libata version 3.00 loaded.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.211078] ACPI: bus type USB registered
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.212877] usbcore: registered new interface driver usbfs
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.214791] usbcore: registered new interface driver hub
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.217431] usbcore: registered new device driver usb
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.219840] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.223011] dmi: Firmware registration failed.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.225423] PCI: Using ACPI for IRQ routing
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.227085] PCI: pci_cache_line_size set to 64 bytes
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.227191] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.227193] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.227337] NetLabel: Initializing
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.228698] NetLabel:  domain hash size = 128
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.230384] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.232337] NetLabel:  unlabeled traffic allowed by default
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.234776] amd_nb: Cannot enumerate AMD northbridges
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.237178] clocksource: Switched to clocksource kvm-clock
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.249756] pnp: PnP ACPI init
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.251328] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.251395] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.251439] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.251489] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.251534] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.251575] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.251614] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.251790] pnp: PnP ACPI: found 7 devices
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.261467] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.264425] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.264427] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.264429] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.264430] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.264472] NET: Registered protocol family 2
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.266702] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.269340] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.272427] TCP: Hash tables configured (established 131072 bind 65536)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.275650] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.277563] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.280842] NET: Registered protocol family 1
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.282593] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.285706] PCI: CLS 0 bytes, default 64
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    1.285779] Unpacking initramfs...
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.541261] Freeing initrd memory: 21432K
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.543023] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.545110] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.549748] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.553465] hw unit of domain pp0-core 2^-0 Joules
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.555096] hw unit of domain package 2^-0 Joules
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.557494] hw unit of domain dram 2^-16 Joules
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.559965] Scanning for low memory corruption every 60 seconds
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.563596] audit: initializing netlink subsys (disabled)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.565936] audit: type=2000 audit(1533729257.309:1): initialized
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.568882] Initialise system trusted keyring
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.571441] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.574120] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.577862] zbud: loaded
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.579191] VFS: Disk quotas dquot_6.6.0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.581218] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.584049] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.587077] fuse init (API version 7.23)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.588942] Key type big_key registered
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.590703] Allocating IMA MOK and blacklist keyrings.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.595335] Key type asymmetric registered
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.597229] Asymmetric key parser 'x509' registered
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.599149] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.602520] io scheduler noop registered
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.603872] io scheduler deadline registered (default)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.605407] io scheduler cfq registered
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.606742] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.609874] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.612462] intel_idle: does not run on family 6 model 63
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.612592] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.615228] ACPI: Power Button [PWRF]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.616678] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.619891] ACPI: Sleep Button [SLPF]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.621563] GHES: HEST is not enabled!
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.627305] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.630377] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.640560] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.643198] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.653635] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.678324] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.704699] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.730710] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.756771] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.762188] Linux agpgart interface v0.103
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.768333] loop: module loaded
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.770463] libphy: Fixed MDIO Bus: probed
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.772703] tun: Universal TUN/TAP device driver, 1.6
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.775251] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.832747] PPP generic driver version 2.4.2
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.836258] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.839771] ehci-pci: EHCI PCI platform driver
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.842128] ehci-platform: EHCI generic platform driver
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.844470] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.848027] ohci-pci: OHCI PCI platform driver
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.850263] ohci-platform: OHCI generic platform driver
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.853512] uhci_hcd: USB Universal Host Controller Interface driver
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.856895] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.862235] i8042: Warning: Keylock active
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.865893] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.868261] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.870930] mousedev: PS/2 mouse device common for all mice
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.874143] rtc_cmos 00:00: RTC can wake from S4
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.877199] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.880272] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.884412] i2c /dev entries driver
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.886748] device-mapper: uevent: version 1.0.3
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.889215] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.894261] ledtrig-cpu: registered to indicate activity on CPUs
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.899460] NET: Registered protocol family 10
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.901971] NET: Registered protocol family 17
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.904338] Key type dns_resolver registered
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.907603] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.909740] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.912500] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.915246] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.917454] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.922524] registered taskstats version 1
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.924261] Loading compiled-in X.509 certificates
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.926838] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.933726] zswap: loaded using pool lzo/zbud
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.939090] Key type trusted registered
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.946305] Key type encrypted registered
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.948512] ima: No TPM chip found, activating TPM-bypass!
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.950902] evm: HMAC attrs: 0x1
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.952658]   Magic number: 14:668:933
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.954406] rtc_cmos 00:00: setting system clock to 2018-08-08 11:54:18 UTC (1533729258)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.959308] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.962435] EDD information not available.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.966441] PM: Hibernation image not present or could not be loaded.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.968527] Freeing unused kernel memory: 1496K
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.970548] Write protecting the kernel read-only data: 14336k
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.975041] Freeing unused kernel memory: 1956K
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.977159] Freeing unused kernel memory: 92K
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    3.997521] systemd-udevd[119]: starting version 204
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    4.074016] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    4.079044] scsi host0: Virtio SCSI HBA
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    4.085733] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    4.090052] AVX2 version of gcm_enc/dec engaged.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    4.092155] AES CTR mode by8 optimization enabled
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    4.166526] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    4.166701] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    4.166702] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    4.167404] sd 0:0:1:0: [sda] Write Protect is off
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    4.167406] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    4.167537] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    4.170064]  sda: sda1
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    4.171919] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    4.557386] tsc: Refined TSC clocksource calibration: 2299.817 MHz
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    4.560179] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x21268855b34, max_idle_ns: 440795253504 ns
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    4.910407] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    7.061341] floppy0: no floppy controllers found
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.229210] raid6: sse2x1   gen()  9246 MB/s
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.297216] raid6: sse2x1   xor()  6965 MB/s
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.365226] raid6: sse2x2   gen() 11541 MB/s
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.433244] raid6: sse2x2   xor()  7979 MB/s
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.501222] raid6: sse2x4   gen() 12503 MB/s
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.569223] raid6: sse2x4   xor()  8592 MB/s
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.637220] raid6: avx2x1   gen() 17540 MB/s
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.705214] raid6: avx2x2   gen() 20673 MB/s
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.773209] raid6: avx2x4   gen() 21459 MB/s
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.774030] raid6: using algorithm avx2x4 gen() 21459 MB/s
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.775078] raid6: using avx2x2 recovery algorithm
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.777350] xor: automatically using best checksumming function:
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.817230]    avx       : 22211.000 MB/sec
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.831699] Btrfs loaded
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.877790] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.880058] EXT4-fs (sda1): write access will be enabled during recovery
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.942713] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.950265] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.951149] EXT4-fs (sda1): recovery complete
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    8.956479] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    9.201439] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    9.336948] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    9.391368] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [    9.607656] random: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   10.213153] random: cloud-init: uninitialized urandom read (32 bytes read, 45 bits of entropy available)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   10.366880] systemd-udevd[701]: starting version 204
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   10.500303] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   10.547314] intel_rapl: no valid rapl domains found in package 0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   10.607436] ppdev: user-space parallel port driver
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   10.731955] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   10.801979] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   10.874456] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   11.049765] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   11.411881] random: mktemp: uninitialized urandom read (12 bytes read, 60 bits of entropy available)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   11.499633] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   11.594053] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   11.654812] EXT4-fs (sda1): resized filesystem to 7864064
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   12.015879] init: failsafe main process (1092) killed by TERM signal
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO Running set_multiqueue.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO Set channels for eth0 to 4.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  8 11:54:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e google-accounts: INFO Starting Google Accounts daemon.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e google-clock-skew: INFO Synced system time with hardware clock.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e google-clock-skew: INFO Synced system time with hardware clock.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e google-accounts: INFO Creating a new user account for me.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e google-accounts: INFO Created user account me.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e google-accounts: INFO Creating a new user account for bogdana.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e google-accounts: INFO Created user account bogdana.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e google-accounts: INFO Creating a new user account for aj.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e google-accounts: INFO Created user account aj.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e google-accounts: INFO Creating a new user account for asari.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e google-accounts: INFO Created user account asari.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   13.264407] random: nonblocking pool is initialized
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e google-accounts: INFO Removing user packer.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   13.506228] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   13.510932] Bridge firewalling registered
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   13.525500] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   13.565278] floppy0: no floppy controllers found
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   13.565481] work still pending
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   13.566590] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   13.681862] Initializing XFRM netlink socket
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   13.690011] Netfilter messages via NETLINK v0.30.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   13.694485] ctnetlink v0.93: registering with nfnetlink.
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 11:54:27 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 11:54:29 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e cron[1630]: (CRON) INFO (pidfile fd = 3)
Aug  8 11:54:29 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e cron[1661]: (CRON) STARTUP (fork ok)
Aug  8 11:54:29 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 11:54:29 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 11:54:29 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e cron[1661]: (CRON) INFO (Running @reboot jobs)
Aug  8 11:54:29 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e acpid: starting up with netlink and the input layer
Aug  8 11:54:29 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e acpid: 1 rule loaded
Aug  8 11:54:29 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e acpid: waiting for events: event logging is off
Aug  8 11:54:29 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e haveged: haveged starting up
Aug  8 11:54:29 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   15.214020] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpd[1767]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpd[1768]: proto: precision = 0.104 usec
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpd[1768]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpd[1768]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpd[1768]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpd[1768]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpd[1768]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpd[1768]: Listen normally on 3 eth0 10.20.255.14 UDP 123
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpd[1768]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpd[1768]: peers refreshed
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpd[1768]: Listening on routing socket on fd #21 for interface updates
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [   20.422887] init: plymouth-upstart-bridge main process ended, respawning
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e startup-script: INFO Found startup-script in metadata.
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e startup-script: INFO startup-script: job 1 at Wed Aug  8 15:04:00 2018
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e startup-script: INFO startup-script: Return code 0.
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e startup-script: INFO startup-script: Return code 0.
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e startup-script: INFO Finished running startup scripts.
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ec2: 
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ec2: #############################################################
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ec2: 1024 0b:f5:ab:71:df:15:ec:41:53:db:5b:0f:fb:8d:bc:27  root@travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e (DSA)
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ec2: 256 8a:e7:26:9b:d7:de:9d:50:ad:b5:a3:21:e4:dd:28:83  root@travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e (ECDSA)
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ec2: 256 34:7f:76:9e:5f:54:29:74:57:27:93:5c:e8:cb:70:b9  root@travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e (ED25519)
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ec2: 2048 d1:d8:0b:dc:49:ee:47:4b:37:7b:9b:bf:b6:f4:b8:52  root@travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e (RSA)
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  8 11:54:34 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ec2: #############################################################
Aug  8 11:54:42 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpdate[2165]: the NTP socket is in use, exiting
Aug  8 11:56:13 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [  119.589740] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  8 11:57:15 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [  181.289055] device vethed69458 entered promiscuous mode
Aug  8 11:57:15 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [  181.388063] cgroup: docker-runc (4773) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  8 11:57:15 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [  181.388065] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  8 11:57:15 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [  181.471722] eth0: renamed from veth1f1e490
Aug  8 11:57:15 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [  181.519763] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  8 11:57:15 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [  181.521525] docker0: port 1(vethed69458) entered forwarding state
Aug  8 11:57:15 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [  181.521540] docker0: port 1(vethed69458) entered forwarding state
Aug  8 11:57:15 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [  181.521556] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  8 11:57:19 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpd[1768]: Listen normally on 5 docker0 fe80::42:39ff:fec4:82cf UDP 123
Aug  8 11:57:19 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpd[1768]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  8 11:57:19 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpd[1768]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  8 11:57:19 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpd[1768]: peers refreshed
Aug  8 11:57:19 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e ntpd[1768]: new interface(s) found: waking up resolver
Aug  8 11:57:30 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [  196.574939] docker0: port 1(vethed69458) entered forwarding state
Aug  8 12:17:01 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e CRON[14278]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  8 12:44:26 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [ 3012.413973] traps: a[5097] trap invalid opcode ip:5558c1313b1b sp:7fffac5359c0 error:0 in a[5558c1310000+6000]
Aug  8 12:44:42 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [ 3028.028392] traps: a[7981] trap invalid opcode ip:7fb0fdd369e1 sp:7ffed5bec860 error:0 in libstd-e054c7a28f8831a7.so[7fb0fdcd7000+171000]
Aug  8 12:44:42 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [ 3028.084030] traps: a[7986] trap invalid opcode ip:7f21ff8429e1 sp:7ffe6e9d5890 error:0 in libstd-e054c7a28f8831a7.so[7f21ff7e3000+171000]
Aug  8 12:46:06 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [ 3111.904918] traps: a[22862] trap invalid opcode ip:560847a10d68 sp:7ffd9eaaf320 error:0 in a[560847a0e000+4000]
Aug  8 12:48:53 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [ 3278.700384] a[19085]: segfault at 0 ip 000055c6f8576548 sp 00007ffd4248a3b0 error 6 in a[55c6f8573000+5000]
Aug  8 12:49:01 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [ 3287.280611] a[19827]: segfault at 1 ip 000055a101bb7b5c sp 00007ffdd80e2100 error 6 in a[55a101bb5000+4000]
Aug  8 12:49:05 travis-job-1a96f1d1-e825-49df-9067-e9adc349e97e kernel: [ 3291.229642] traps: a[20202] trap invalid opcode ip:563b4289342c sp:7fffccfeac90 error:0 in a[563b42890000+7000]
---
travis_time:end:1da1b2ca:start=1533733193394822604,finish=1533733193503848462,duration=109025858
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0023c3eb
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:01ab4403
$ dmesg | grep -i kill
