plain
[00:55:27] ....................................................................................................
[00:55:29] ....................................................................................................
[00:55:32] ....................................................................................................
[00:55:35] ....................................................................................................
[00:55:38] ..............iiiiiiiii.............................................................................
[00:55:44] ....................................................................................................
[00:55:47] ....................i...............................................................................
[00:55:50] ...............................i....................................................................
[00:55:53] ....................................................................................................
---
[01:17:16] iii.................................................................................................
[01:17:29] ................................................................................................iii.
[01:17:37] .....i......i...i......i............................................................................
[01:17:42] ....................................................................................................
[01:17:54] ......F.............iiii........ii..................................................................
[01:18:11] ....................................................................iiii............................
[01:18:27] ....................................................................................................
[01:18:32] ..................................................................................iiii..............
[01:18:36] .................................
[01:18:36] .................................
[01:18:36] failures:
[01:18:36] 
[01:18:36] ---- keyword_docs.rs - let_keyword (line 48) stdout ----
[01:18:36] error: value assigned to `x` is never read
[01:18:36]  --> keyword_docs.rs:52:1
[01:18:36]   |
[01:18:36] 7 | x += 4; // `x` is now equal to `7`.
[01:18:36]   |
[01:18:36] note: lint level defined here
[01:18:36]  --> keyword_docs.rs:46:9
[01:18:36]   |
[01:18:36]   |
[01:18:36] 1 | #![deny(warnings)]
[01:18:36]   |         ^^^^^^^^
[01:18:36]   = note: #[deny(unused_assignments)] implied by #[deny(warnings)]
[01:18:36] 
[01:18:36] thread 'keyword_docs.rs - let_keyword (line 48)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:18:36] 
[01:18:36] 
[01:18:36] failures:
[01:18:36]     keyword_docs.rs - let_keyword (line 48)
[01:18:36]     keyword_docs.rs - let_keyword (line 48)
[01:18:36] 
[01:18:36] test result: FAILED. 908 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
[01:18:36] 
[01:18:36] error: test failed, to rerun pass '--doc'
[01:18:36] 
[01:18:36] 
[01:18:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:18:36] 
[01:18:36] 
[01:18:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:36] Build completed unsuccessfully in 0:25:59
[01:18:36] Build completed unsuccessfully in 0:25:59
[01:18:36] Makefile:58: recipe for target 'check' failed
[01:18:36] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:327d717b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:035c4409
$ sudo tail -n 500 /var/log/syslog
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 0000PXM 0 [mem 0x00000000-0x0009ffff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] kvm-clock: using sched offset of 1768806078 cycles
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] Zone ranges:
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000]   Device   empty
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] Movable zone start for each node
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] Early memory node ranges
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] Policy zone: Normal
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] Hierarchical RCU implementation.
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] console [ttyS0] enabled
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.653947] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.660671] pid_max: default: 32768 minimum: 301
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.664959] ACPI: Core revision 20150930
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.674556] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.680939] Security Framework initialized
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.687416] Yama: becoming mindful.
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.690568] AppArmor: AppArmor disabled by boot time parameter
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.696246] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.710718] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.718110] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.721753] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.726149] Initializing cgroup subsys io
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.728477] Initializing cgroup subsys memory
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.731228] Initializing cgroup subsys devices
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.733792] Initializing cgroup subsys freezer
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.736957] Initializing cgroup subsys net_cls
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.739500] Initializing cgroup subsys perf_event
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.741912] Initializing cgroup subsys net_prio
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.745709] Initializing cgroup subsys hugetlb
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.748733] Initializing cgroup subsys pids
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.751718] CPU: Physical Processor ID: 0
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.754177] CPU: Processor Core ID: 0
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.758130] mce: CPU supports 32 MCE banks
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.761287] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.766309] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.774468] Freeing SMP alternatives memory: 32K
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.786474] ftrace: allocating 32185 entries in 126 pages
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.844753] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.848753] smpboot: Max logical packages: 2
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.851779] x2apic enabled
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.854475] Switched APIC routing to physical x2apic.
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.860182] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.970385] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.976476] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    0.982241] x86: Booting SMP configuration:
Aug 10 17:08:48g 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.210996] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.220174] PCI host bridge to bus 0000:00
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.222721] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.226517] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.230578] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.234814] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.239685] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.243458] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.243927] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.274130] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.303725] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.308996] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.319557] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.328590] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.351417] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.364010] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.372700] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.398720] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.405357] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.411349] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.418087] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.424144] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.447475] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.451669] vgaarb: loaded
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.455081] SCSI subsystem initialized
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.458151] libata version 3.00 loaded.
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.458179] ACPI: bus type USB registered
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.461266] usbcore: registered new interface driver usbfs
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.465198] usbcore: registered new interface driver hub
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.468941] usbcore: registered new device driver usb
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.472555] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.476946] dmi: Firmware registration failed.
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.480078] PCI: Using ACPI for IRQ routing
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.482514] PCI: pci_cache_line_size set to 64 bytes
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.482615] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.482617] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.482761] NetLabel: Initializing
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.484749] NetLabel:  domain hash size = 128
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.487181] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.490666] NetLabel:  unlabeled traffic allowed by default
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.494961] amd_nb: Cannot enumerate AMD northbridges
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.498044] clocksource: Switched to clocksource kvm-clock
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.508404] pnp: PnP ACPI init
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.510446] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.510525] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.510571] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.510621] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.510665] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.510706] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.510748] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.510950] pnp: PnP ACPI: found 7 devices
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.520221] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.525664] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.525667] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.525669] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.525671] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.525719] NET: Registered protocol family 2
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.528152] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.532495] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.536509] TCP: Hash tables configured (established 131072 bind 65536)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.541586] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.544588] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.548985] NET: Registered protocol family 1
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.551518] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.554705] PCI: CLS 0 bytes, default 64
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    1.555499] Unpacking initramfs...
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.609429] Freeing initrd memory: 21432K
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.611843] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.615321] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.620769] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.626129] hw unit of domain pp0-core 2^-0 Joules
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.62ower Button [PWRF]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.731755] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.736614] ACPI: Sleep Button [SLPF]
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.739581] GHES: HEST is not enabled!
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.744750] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.752382] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.766311] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.769916] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.784924] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.811685] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.837656] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.865180] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.892306] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.899525] Linux agpgart interface v0.103
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.905491] loop: module loaded
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.908975] libphy: Fixed MDIO Bus: probed
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.912125] tun: Universal TUN/TAP device driver, 1.6
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.914952] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.970711] PPP generic driver version 2.4.2
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.973234] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.976345] ehci-pci: EHCI PCI platform driver
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.978549] ehci-platform: EHCI generic platform driver
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.981579] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.986169] ohci-pci: OHCI PCI platform driver
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.988900] ohci-platform: OHCI generic platform driver
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.992672] uhci_hcd: USB Universal Host Controller Interface driver
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    3.996082] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.002001] i8042: Warning: Keylock active
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.006107] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.008410] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.011190] mousedev: PS/2 mouse device common for all mice
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.014908] rtc_cmos 00:00: RTC can wake from S4
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.018043] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.021757] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.025479] i2c /dev entries driver
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.027499] device-mapper: uevent: version 1.0.3
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.030485] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.034736] ledtrig-cpu: registered to indicate activity on CPUs
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.038677] NET: Registered protocol family 10
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.041851] NET: Registered protocol family 17
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.044099] Key type dns_resolver registered
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.046547] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.049265] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.051835] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.054762] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.057237] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.062143] registered taskstats version 1
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.063991] Loading compiled-in X.509 certificates
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.066355] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.070111] zswap: loaded using pool lzo/zbud
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.074618] Key type trusted registered
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.080941] Key type encrypted registered
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.082752] ima: No TPM chip found, activating TPM-bypass!
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.086324] evm: HMAC attrs: 0x1
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.088127]   Magic number: 14:630:137
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.089686] rtc_cmos 00:00: setting system clock to 2018-08-10 17:08:40 UTC (1533920920)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.092612] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.094692] EDD information not available.
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.095879] PM: Hibernation image not present or could not be loaded.
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.097523] Freeing unused kernel memory: 1496K
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.099141] Write protecting the kernel read-only data: 14336k
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.102001] Freeing unused kernel memory: 1956K
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.103550] Freeing unused kernel memory: 92K
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.119447] systemd-udevd[119]: starting version 204
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.179198] scsi host0: Virtio SCSI HBA
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.185362] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.187299] AVX2 version of gcm_enc/dec engaged.
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.188012] AES CTR mode by8 optimization enabled
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.214458] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.228576] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.228580] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.230744] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.232049] sd 0:0:1:0: [sda] Write Protect is off
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [    4.232824] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 10 17:08:4] ppdev: user-space parallel port driver
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   11.100944] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   11.152957] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   11.211864] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   11.373933] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   11.656783] random: mktemp: uninitialized urandom read (12 bytes read, 59 bits of entropy available)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   11.731710] random: mktemp: uninitialized urandom read (6 bytes read, 60 bits of entropy available)
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   11.819574] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   11.874095] EXT4-fs (sda1): resized filesystem to 7864064
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   12.211520] init: failsafe main process (1095) killed by TERM signal
Aug 10 17:08:48 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO Running set_multiqueue.
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO Set channels for eth0 to 4.
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   12.995891] random: nonblocking pool is initialized
Aug 10 17:08:49 travis-job bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   13.698455] Bridge firewalling registered
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   13.709235] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   13.780420] Initializing XFRM netlink socket
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   13.787420] Netfilter messages via NETLINK v0.30.
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   13.790185] ctnetlink v0.93: registering with nfnetlink.
Aug 10 17:08:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   13.962145] floppy0: no floppy controllers found
Aug 10 17:09:12 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpdate[1780]: adjust time server 169.254.169.254 offset 0.009873 sec
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpd[1816]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpd[1817]: proto: precision = 0.106 usec
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpd[1817]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpd[1817]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpd[1817]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpd[1817]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpd[1817]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpd[1817]: Listen normally on 3 eth0 10.20.0.220 UDP 123
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpd[1817]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpd[1817]: peers refreshed
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpd[1817]: Listening on routing socket on fd #21 for interface updates
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [   43.601036] init: plymouth-upstart-bridge main process ended, respawning
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c startup-script: INFO Found startup-script in metadata.
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c startup-script: INFO startup-script: job 1 at Fri Aug 10 20:19:00 2018
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c startup-script: INFO startup-script: Return code 0.
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c startup-script: INFO startup-script: Return code 0.
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c startup-script: INFO Finished running startup scripts.
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ec2: 
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ec2: #############################################################
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ec2: 1024 81:bd:27:d3:57:6f:b9:27:f0:0c:17:cc:57:19:20:b9  root@travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c (DSA)
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ec2: 256 3b:bb:d4:0d:ad:7e:a3:9b:b9:05:0b:64:e0:81:56:50  root@travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c (ECDSA)
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ec2: 256 ae:d2:0a:18:30:6f:54:d1:c6:12:f5:af:84:ec:7e:fb  root@travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c (ED25519)
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ec2: 2048 45:1f:c1:22:d1:19:cf:a9:dc:67:eb:80:f5:48:05:a8  root@travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c (RSA)
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 10 17:09:19 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ec2: #############################################################
Aug 10 17:12:18 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [  222.323807] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 10 17:13:34 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [  298.513774] device veth78b3c2b entered promiscuous mode
Aug 10 17:13:34 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [  298.513827] docker0: port 1(veth78b3c2b) entered forwarding state
Aug 10 17:13:34 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [  298.513833] docker0: port 1(veth78b3c2b) entered forwarding state
Aug 10 17:13:34 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [  298.514217] docker0: port 1(veth78b3c2b) entered disabled state
Aug 10 17:13:34 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [  298.631006] cgroup: docker-runc (4797) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 10 17:13:34 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [  298.631011] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 10 17:13:34 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [  298.718193] eth0: renamed from vethd5116b3
Aug 10 17:13:34 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [  298.753501] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 10 17:13:34 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [  298.754750] docker0: port 1(veth78b3c2b) entered forwarding state
Aug 10 17:13:34 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [  298.754769] docker0: port 1(veth78b3c2b) entered forwarding state
Aug 10 17:13:34 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [  298.754791] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 10 17:13:38 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpd[1817]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 10 17:13:38 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpd[1817]: Listen normally on 6 docker0 fe80::42:ebff:fed7:39c UDP 123
Aug 10 17:13:38 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpd[1817]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 10 17:13:38 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpd[1817]: peers refreshed
Aug 10 17:13:38 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c ntpd[1817]: new interface(s) found: waking up resolver
Aug 10 17:13:49 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [  313.793632] docker0: port 1(veth78b3c2b) entered forwarding state
Aug 10 17:17:01 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c CRON[4938]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 10 18:08:23 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [ 3587.611148] traps: a[5259] trap invalid opcode ip:55e61349cb4b sp:7ffe76280be0 error:0 in a[55e613499000+6000]
Aug 10 18:08:38 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [ 3602.911455] traps: a[8083] trap invalid opcode ip:7fbb50abd0c1 sp:7fff7baacb10 error:0 in libstd-2339b911e3c09de8.so[7fbb50a5e000+16e000]
Aug 10 18:08:38 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [ 3602.951067] traps: a[8084] trap invalid opcode ip:7f7556a960c1 sp:7ffd5811a640 error:0 in libstd-2339b911e3c09de8.so[7f7556a37000+16e000]
Aug 10 18:10:03 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [ 3687.720420] traps: a[22940] trap invalid opcode ip:560e92f22d98 sp:7ffe8b5d71f0 error:0 in a[560e92f20000+4000]
Aug 10 18:12:52 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [ 3856.673628] a[18949]: segfault at 0 ip 0000558eac340463 sp 00007ffd6e508760 error 6 in a[558eac33d000+5000]
Aug 10 18:13:02 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [ 3866.760473] a[19708]: segfault at 1 ip 000055efbf0c0b8c sp 00007ffef682ccd0 error 6 in a[55efbf0be000+4000]
Aug 10 18:13:06 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [ 3870.945928] traps: a[20083] trap invalid opcode ip:561ed6fd542c sp:7ffc8831ac90 error:0 in a[561ed6fd2000+7000]
Aug 10 18:17:01 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c CRON[15246]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 10 18:30:55 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [ 4939.797068] docker0: port 1(veth78b3c2b) entered disabled state
Aug 10 18:30:55 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [ 4939.797120] vethd5116b3: renamed from eth0
Aug 10 18:30:55 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [ 4939.855666] docker0: port 1(veth78b3c2b) entered disabled state
Aug 10 18:30:55 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [ 4939.857589] device veth78b3c2b left promiscuous mode
Aug 10 18:30:55 travis-job-4bbf7d44-7052-4901-9598-36fcd4505b1c kernel: [ 4939.857593] docker0: port 1(veth78b3c2b) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:1aa204c9
