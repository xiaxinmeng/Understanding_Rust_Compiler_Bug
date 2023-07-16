plain
[00:47:40] ....................................................................................................
[00:47:43] ....................................................................................................
[00:47:45] ....................................................................................................
[00:47:49] ....................................................................................................
[00:47:51] ..............iiiiiiiii.............................................................................
[00:47:57] ....................................................................................................
[00:48:01] ....................i...............................................................................
[00:48:04] ..............................i.....................................................................
[00:48:07] ....................................................................................................
---
[01:08:51] iii.................................................................................................
[01:09:05] ................................................................................................iii.
[01:09:13] .....i......i...i......i............................................................................
[01:09:19] ....................................................................................................
[01:09:31] ......F.............iiii........ii..................................................................
[01:09:48] ....................................................................iiii............................
[01:10:04] ....................................................................................................
[01:10:09] ..................................................................................iiii..............
[01:10:13] .................................
[01:10:13] .................................
[01:10:13] failures:
[01:10:13] 
[01:10:13] ---- keyword_docs.rs - let_keyword (line 47) stdout ----
[01:10:13] error: value assigned to `x` is never read
[01:10:13]  --> keyword_docs.rs:50:1
[01:10:13]   |
[01:10:13] 6 | x += 4; // `x` is now equal to `7`.
[01:10:13]   |
[01:10:13] note: lint level defined here
[01:10:13]  --> keyword_docs.rs:45:9
[01:10:13]   |
[01:10:13]   |
[01:10:13] 1 | #![deny(warnings)]
[01:10:13]   |         ^^^^^^^^
[01:10:13]   = note: #[deny(unused_assignments)] implied by #[deny(warnings)]
[01:10:13] 
[01:10:13] thread 'keyword_docs.rs - let_keyword (line 47)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:10:13] 
[01:10:13] 
[01:10:13] failures:
[01:10:13]     keyword_docs.rs - let_keyword (line 47)
[01:10:13]     keyword_docs.rs - let_keyword (line 47)
[01:10:13] 
[01:10:13] test result: FAILED. 908 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
[01:10:13] 
[01:10:13] error: test failed, to rerun pass '--doc'
[01:10:13] 
[01:10:13] 
[01:10:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:10:13] 
[01:10:13] 
[01:10:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:13] Build completed unsuccessfully in 0:25:17
[01:10:13] Build completed unsuccessfully in 0:25:17
[01:10:13] Makefile:58: recipe for target 'check' failed
[01:10:13] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b6927c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0bdbcb1b
$ sudo tail -n 500 /var/log/syslog
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] kvm-clock: using sched offset of 1770204092 cycles
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] Zone ranges:
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000]   Device   empty
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] Movable zone start for each node
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] Early memory node ranges
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] Policy zone: Normal
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] Hierarchical RCU implementation.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] console [ttyS0] enabled
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.473298] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.476229] pid_max: default: 32768 minimum: 301
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.477951] ACPI: Core revision 20150930
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.486417] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.488708] Security Framework initialized
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.490057] Yama: becoming mindful.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.491490] AppArmor: AppArmor disabled by boot time parameter
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.495599] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.507046] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.514032] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.516756] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.519532] Initializing cgroup subsys io
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.520867] Initializing cgroup subsys memory
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.522280] Initializing cgroup subsys devices
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.523648] Initializing cgroup subsys freezer
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.525269] Initializing cgroup subsys net_cls
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.527195] Initializing cgroup subsys perf_event
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.528670] Initializing cgroup subsys net_prio
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.530405] Initializing cgroup subsys hugetlb
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.532565] Initializing cgroup subsys pids
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.533937] CPU: Physical Processor ID: 0
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.535165] CPU: Processor Core ID: 0
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.537756] mce: CPU supports 32 MCE banks
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.539325] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.541340] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.545816] Freeing SMP alternatives memory: 32K
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.556007] ftrace: allocating 32185 entries in 126 pages
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.607248] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.609841] smpboot: Max logical packages: 2
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.612589] x2apic enabled
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.614769] Switched APIC routing to physical x2apic.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.620249] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.727619] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.730954] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.735472] x86: Booting SMP configuration:
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.737019] .... node  #0, CPUs:      #1
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.738620] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.744747]  #2
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.745780] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.751478]  #3
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.752452] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.758104] x86: Booted up 1 node, 4 CPUs
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.759428] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.763386] devtmpfs: initialized
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.768797] evm: security.selinux
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.770497] evm: security.SMACK64
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.771662] evm: security.SMACK64EXEC
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.773089] evm: security.SMACK64TRANSMUTE
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.774332] evm: security.SMACK64MMAP
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.775618] evm: security.ima
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.776609] evm: security.capability
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.778255] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.781527] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.783719] pinctrl core: initialized pinctrl subsystem
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.785549] RTC time: 20:27:04, date: 08/09/18
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.788156] NET: Registered protocol family 16
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.799695] cpuidle: using governor ladder
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.811699] cpuidle: using governor menu
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.813204] PCCT header not found.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.814569] ACPI: bus type PCI registered
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.816141] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.818721] PCI: Using configuration type 1 for base access
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.833340] ACPI: Added _OSI(Module Device)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.835143] ACPI: Added _OSI(Processor Device)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.836690] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.838214] ACPI: Added _OSI(Processor Aggregator Device)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.843124] ACPI: Executed 2 blocks of module-level executable AML code
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.869124] ACPI: Interpreter enabled
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.870920] ACPI: (supports S0 S3 S4 S5)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.873045] ACPI: Using IOAPIC for interrupt routing
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.875435] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.910341] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  9 20:27:15 travis-job-ed874879-9-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.968797] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.996704] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    0.999530] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.009825] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.018515] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.043500] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.054600] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.064077] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.087711] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.092333] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.097144] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.101200] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.105676] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.129230] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.132057] vgaarb: loaded
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.133292] SCSI subsystem initialized
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.134585] libata version 3.00 loaded.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.134617] ACPI: bus type USB registered
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.136031] usbcore: registered new interface driver usbfs
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.138062] usbcore: registered new interface driver hub
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.139932] usbcore: registered new device driver usb
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.141944] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.144671] dmi: Firmware registration failed.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.146486] PCI: Using ACPI for IRQ routing
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.148215] PCI: pci_cache_line_size set to 64 bytes
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.148326] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.148328] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.148455] NetLabel: Initializing
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.150066] NetLabel:  domain hash size = 128
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.151661] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.153824] NetLabel:  unlabeled traffic allowed by default
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.157581] amd_nb: Cannot enumerate AMD northbridges
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.160699] clocksource: Switched to clocksource kvm-clock
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.172236] pnp: PnP ACPI init
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.174669] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.174742] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.174792] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.174880] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.174924] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.174967] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.175005] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.175171] pnp: PnP ACPI: found 7 devices
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.185718] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.189901] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.189904] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.189906] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.189907] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.189943] NET: Registered protocol family 2
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.192215] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.195691] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.198702] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.200981] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.203298] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.206306] NET: Registered protocol family 1
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.208817] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.212404] PCI: CLS 0 bytes, default 64
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    1.213372] Unpacking initramfs...
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.496903] Freeing initrd memory: 21432K
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.499188] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.501253] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.506039] RAPL PMU detected, API unit is 2^ob-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.537078] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.539821] fuse init (API version 7.23)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.541373] Key type big_key registered
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.542785] Allocating IMA MOK and blacklist keyrings.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.547966] Key type asymmetric registered
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.549489] Asymmetric key parser 'x509' registered
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.551353] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.553878] io scheduler noop registered
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.555280] io scheduler deadline registered (default)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.557174] io scheduler cfq registered
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.558438] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.560362] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.562716] intel_idle: does not run on family 6 model 62
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.562832] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.565733] ACPI: Power Button [PWRF]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.567135] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.569816] ACPI: Sleep Button [SLPF]
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.571707] GHES: HEST is not enabled!
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.576743] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.579341] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.589790] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.592017] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.602271] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.627481] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.654143] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.680414] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.706703] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.715109] Linux agpgart interface v0.103
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.720421] loop: module loaded
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.722595] libphy: Fixed MDIO Bus: probed
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.724947] tun: Universal TUN/TAP device driver, 1.6
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.727836] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.789739] PPP generic driver version 2.4.2
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.792563] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.795895] ehci-pci: EHCI PCI platform driver
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.799209] ehci-platform: EHCI generic platform driver
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.802321] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.805587] ohci-pci: OHCI PCI platform driver
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.807540] ohci-platform: OHCI generic platform driver
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.810453] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.814025] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.819199] i8042: Warning: Keylock active
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.822533] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.825517] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.829518] mousedev: PS/2 mouse device common for all mice
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.832719] rtc_cmos 00:00: RTC can wake from S4
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.835972] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.839036] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.843681] i2c /dev entries driver
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.845934] device-mapper: uevent: version 1.0.3
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.848723] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.853331] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.858414] NET: Registered protocol family 10
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.861575] NET: Registered protocol family 17
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.863832] Key type dns_resolver registered
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.866663] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.869702] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.872548] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.874902] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.877215] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.882388] registered taskstats version 1
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.884639] Loading compiled-in X.509 certificates
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.887274] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.893298] zswap: loaded using pool lzo/zbud
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.898295] Key type trusted registered
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.906619] Key type encrypted registered
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.908985] ima: No TPM chip found, activating TPM-bypass!
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.911863] evm: HMAC attrs: 0x1
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.914308]   Magic number: 14:735:497
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.915897] rtc_cmos 00:00: setting system clock to 2018-08-09 20:27:07 UTC (1533846427)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.919964] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.923420] EDD information not available.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.925148] PM: Hibernation image not present or could not be loaded.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.926730] Freeing unused kernel memory: 1496K
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.928362] Write protecting the kernel read-only data: 14336k
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.931566] Freeing unused kernel memory: 1956K
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.933982] Freeing unused kernel memory: 92K
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    3.954010] systemd-udevd[118]: starting version 204
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    4.030119] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    4.034069] scsi host0: Virtio SCSI HBA
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    4.047681] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    4.053856] AVX version of gcm_enc/dec engaged.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    4.055898] AES CTR mode by8 optimization enabled
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    4.130450] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    4.133008] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    4.135959] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    4.138814] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    4.140656] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    4.141187] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    4.147314]  sda: sda1
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    4.150189] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    4.512871] tsc: Refined TSC clocksource calibration: 2499.762 MHz
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    4.515120] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2408592f29b, max_idle_ns: 440795301848 ns
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    4.877955] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    7.009041] floppy0: no floppy controllers found
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.184754] raid6: sse2x1   gen()  9174 MB/s
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.252739] raid6: sse2x1   xor()  7094 MB/s
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.320731] raid6: sse2x2   gen() 11388 MB/s
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.388734] raid6: sse2x2   xor()  7964 MB/s
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.456776] raid6: sse2x4   gen() 12498 MB/s
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.524741] raid6: sse2x4   xor()  8377 MB/s
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.525683] raid6: using algorithm sse2x4 gen() 12498 MB/s
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.526687] raid6: .... xor() 8377 MB/s, rmw enabled
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.528135] raid6: using ssse3x2 recovery algorithm
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.530091] xor: automatically using best checksumming function:
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.568733]    avx       : 22389.000 MB/sec
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.583060] Btrfs loaded
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.628739] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.630301] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.697170] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.710856] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.712930] EXT4-fs (sda1): recovery complete
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.719211] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    8.963097] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    9.102989] random: mountall: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    9.166370] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [    9.408898] random: cloud-init: uninitialized urandom read (32 bytes read, 33 bits of entropy available)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [   10.119451] random: cloud-init: uninitialized urandom read (32 bytes read, 41 bits of entropy available)
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [   10.286777] systemd-udevd[705]: starting version 204
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [   10.420949] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [   10.526059] ppdev: user-space parallel port driver
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO Running set_multiqueue.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO Set channels for eth0 to 4.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  9 20:27:15 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  9 20:27:16 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  9 20:27:16 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  9 20:27:16 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 20:27:16 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Starting Google Accounts daemon.
Aug  9 20:27:16 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Creating a new user account for me.
Aug  9 20:27:16 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Created user account me.
Aug  9 20:27:16 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Creating a new user account for henrik.
Aug  9 20:27:16 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Created user account henrik.
Aug  9 20:27:16 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Creating a new user account for emma.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Created user account emma.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Creating a new user account for igor.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Created user account igor.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [   13.283938] random: nonblocking pool is initialized
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Created user account konstantinhaase.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Creating a new user account for aj.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Created user account aj.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [   13.417608] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [   13.422299] Bridge firewalling registered
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Creating a new user account for solarce.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [   13.436140] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [   13.483269] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Created user account solarce.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [   13.489087] floppy0: no floppy controllers found
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Creating a new user account for asari.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Created user account asari.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [   13.583152] Initializing XFRM netlink socket
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Creating a new user account for bogdana.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [   13.606740] Netfilter messages via NETLINK v0.30.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [   13.610446] ctnetlink v0.93: registering with nfnetlink.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Created user account bogdana.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Creating a new user account for konstantin.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Created user account konstantin.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Creating a new user account for carmen.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Created user account carmen.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Creating a new user account for maria.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Created user account maria.
Aug  9 20:27:17 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f google-accounts: INFO Removing user packer.
Aug  9 20:27:18 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f cron[1716]: (CRON) INFO (pidfile fd = 3)
Aug  9 20:27:18 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f cron[1749]: (CRON) STARTUP (fork ok)
Aug  9 20:27:18 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 20:27:18 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 20:27:18 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f cron[1749]: (CRON) INFO (Running @reboot jobs)
Aug  9 20:27:18 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f acpid: starting up with netlink and the input layer
Aug  9 20:27:18 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f acpid: 1 rule loaded
Aug  9 20:27:18 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f acpid: waiting for events: event logging is off
Aug  9 20:27:18 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f haveged: haveged starting up
Aug  9 20:27:18 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [   14.895420] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 20:27:23 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpd[1850]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 20:27:23 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpd[1851]: proto: precision = 0.107 usec
Aug  9 20:27:23 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpd[1851]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 20:27:23 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpd[1851]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 20:27:23 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpd[1851]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 20:27:23 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpd[1851]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 20:27:23 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpd[1851]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 20:27:23 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpd[1851]: Listen normally on 3 eth0 10.20.0.207 UDP 123
Aug  9 20:27:23 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpd[1851]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 20:27:23 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpd[1851]: peers refreshed
Aug  9 20:27:23 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpd[1851]: Listening on routing socket on fd #21 for interface updates
Aug  9 20:27:24 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [   20.102166] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 20:27:24 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f startup-script: INFO Found startup-script in metadata.
Aug  9 20:27:24 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  9 20:27:24 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f startup-script: INFO startup-script: job 1 at Thu Aug  9 23:37:00 2018
Aug  9 20:27:24 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f startup-script: INFO startup-script: Return code 0.
Aug  9 20:27:24 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f startup-script: INFO startup-script: Return code 0.
Aug  9 20:27:24 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f startup-script: INFO Finished running startup scripts.
Aug  9 20:27:24 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ec2: 
Aug  9 20:27:24 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ec2: #############################################################
Aug  9 20:27:24 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 20:27:24 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ec2: 1024 0e:9e:69:be:fc:18:99:ad:1e:49:da:71:f0:bf:cf:39  root@travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f (DSA)
Aug  9 20:27:24 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ec2: 256 59:51:31:34:26:ce:35:b9:69:3e:2c:cf:89:d5:93:ea  root@travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f (ECDSA)
Aug  9 20:27:24 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ec2: 256 88:a9:9f:52:0d:14:4a:6c:a5:92:7a:37:68:aa:bb:85  root@travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f (ED25519)
Aug  9 20:27:24 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ec2: 2048 b6:36:7c:28:bf:d9:49:39:c2:1f:a2:dd:3d:1c:0e:4b  root@travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f (RSA)
Aug  9 20:27:24 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 20:27:24 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ec2: #############################################################
Aug  9 20:27:32 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpdate[1949]: the NTP socket is in use, exiting
Aug  9 20:29:42 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [  159.004176] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 20:30:54 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [  230.437726] device veth036724e entered promiscuous mode
Aug  9 20:30:54 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [  230.437792] docker0: port 1(veth036724e) entered forwarding state
Aug  9 20:30:54 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [  230.437800] docker0: port 1(veth036724e) entered forwarding state
Aug  9 20:30:54 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [  230.439245] docker0: port 1(veth036724e) entered disabled state
Aug  9 20:30:54 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [  230.525934] cgroup: docker-runc (4877) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 20:30:54 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [  230.525936] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  9 20:30:54 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [  230.593576] eth0: renamed from vethf942624
Aug  9 20:30:54 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [  230.631906] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  9 20:30:54 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [  230.633298] docker0: port 1(veth036724e) entered forwarding state
Aug  9 20:30:54 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [  230.633313] docker0: port 1(veth036724e) entered forwarding state
Aug  9 20:30:54 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [  230.633333] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  9 20:30:57 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpd[1851]: Listen normally on 5 docker0 fe80::42:4eff:fef0:c0bc UDP 123
Aug  9 20:30:57 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpd[1851]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  9 20:30:57 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpd[1851]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  9 20:30:57 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpd[1851]: peers refreshed
Aug  9 20:30:57 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f ntpd[1851]: new interface(s) found: waking up resolver
Aug  9 20:31:09 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [  245.681060] docker0: port 1(veth036724e) entered forwarding state
Aug  9 21:17:01 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f CRON[31028]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  9 21:18:01 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [ 3057.156521] traps: a[5342] trap invalid opcode ip:55b93f596b1b sp:7ffc04cf81e0 error:0 in a[55b93f593000+6000]
Aug  9 21:18:16 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [ 3072.270766] traps: a[8193] trap invalid opcode ip:7fb89bc27ea1 sp:7fffd14e4c60 error:0 in libstd-2339b911e3c09de8.so[7fb89bbcd000+172000]
Aug  9 21:18:16 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [ 3072.299564] traps: a[8198] trap invalid opcode ip:7f976eafdea1 sp:7ffe9ce2d4a0 error:0 in libstd-2339b911e3c09de8.so[7f976eaa3000+172000]
Aug  9 21:19:37 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [ 3153.350134] traps: a[23110] trap invalid opcode ip:55a202818d68 sp:7ffd6a5a5dc0 error:0 in a[55a202816000+4000]
Aug  9 21:22:21 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [ 3317.559868] a[19302]: segfault at 0 ip 000055de1e79d548 sp 00007ffe77af95d0 error 6 in a[55de1e79a000+5000]
Aug  9 21:22:30 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [ 3326.549114] a[20062]: segfault at 1 ip 000055963cab9b5c sp 00007fffa3e52a70 error 6 in a[55963cab7000+4000]
Aug  9 21:22:34 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [ 3330.516034] traps: a[20434] trap invalid opcode ip:56418284b42c sp:7ffda637a090 error:0 in a[564182848000+7000]
Aug  9 21:39:57 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [ 4372.970217] docker0: port 1(veth036724e) entered disabled state
Aug  9 21:39:57 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [ 4372.970316] vethf942624: renamed from eth0
Aug  9 21:39:57 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [ 4373.043619] docker0: port 1(veth036724e) entered disabled state
Aug  9 21:39:57 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [ 4373.045541] device veth036724e left promiscuous mode
Aug  9 21:39:57 travis-job-ed874879-9ee1-48cf-820c-f1e452f17f6f kernel: [ 4373.045544] docker0: port 1(veth036724e) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:1515d099
