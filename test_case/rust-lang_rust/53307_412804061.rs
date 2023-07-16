plain
[00:46:06] ....................................................................................................
[00:46:09] ....................................................................................................
[00:46:11] ....................................................................................................
[00:46:14] ....................................................................................................
[00:46:17] ....................iiiiiiiii.......................................................................
[00:46:22] ....................................................................................................
[00:46:26] ..........................i.........................................................................
[00:46:28] .....................................i..............................................................
[00:46:31] ....................................................................................................
---
[01:06:43] .....i......i...i......i............................................................................
[01:06:48] ....................................................................................................
[01:06:59] ....................iiii........ii..................................................................
[01:07:04] ....................................................................................................
[01:07:15] ..................................................F.................iiii............................
[01:07:30] ..............................................................................................F.....
[01:07:35] ....................................................................................iiii.F..........
[01:07:38] failures:
[01:07:38] 
[01:07:38] ---- path.rs - path::PrefixComponent (line 400) stdout ----
[01:07:38] ---- path.rs - path::PrefixComponent (line 400) stdout ----
[01:07:38] error[E0433]: failed to resolve. Maybe a missing `extern crate core;`?
[01:07:38]   --> path.rs:411:21
[01:07:38]    |
[01:07:38] 14 |     _ => unsafe { ::core::hint::unreachable_unchecked() },
[01:07:38]    |                     ^^^^ Maybe a missing `extern crate core;`?
[01:07:38] thread 'path.rs - path::PrefixComponent (line 400)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:07:38] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:38] 
[01:07:38] ---- sync/rwlock.rs - sync::rwlock::RwLock<T>::try_read (line 220) stdout ----
[01:07:38] ---- sync/rwlock.rs - sync::rwlock::RwLock<T>::try_read (line 220) stdout ----
[01:07:38] error[E0433]: failed to resolve. Maybe a missing `extern crate core;`?
[01:07:38]   --> sync/rwlock.rs:227:26
[01:07:38]    |
[01:07:38] 10 |     Err(_) => unsafe { ::core::hint::unreachable_unchecked() },
[01:07:38]    |                          ^^^^ Maybe a missing `extern crate core;`?
[01:07:38] thread 'sync/rwlock.rs - sync::rwlock::RwLock<T>::try_read (line 220)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:07:38] 
[01:07:38] ---- sys_common/poison.rs - sys_common::poison::PoisonError (line 70) stdout ----
[01:07:38] ---- sys_common/poison.rs - sys_common::poison::PoisonError (line 70) stdout ----
[01:07:38] error[E0433]: failed to resolve. Maybe a missing `extern crate core;`?
[01:07:38]   --> sys_common/poison.rs:85:25
[01:07:38]    |
[01:07:38] 18 |     Ok(_) => unsafe { ::core::hint::unreachable_unchecked() },
[01:07:38]    |                         ^^^^ Maybe a missing `extern crate core;`?
[01:07:38] thread 'sys_common/poison.rs - sys_common::poison::PoisonError (line 70)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:07:38] 
[01:07:38] 
[01:07:38] failures:
---
[01:07:38] 
[01:07:38] error: test failed, to rerun pass '--doc'
[01:07:38] 
[01:07:38] 
[01:07:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:07:38] 
[01:07:38] 
[01:07:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:38] Build completed unsuccessfully in 0:24:11
[01:07:38] Build completed unsuccessfully in 0:24:11
[01:07:38] make: *** [check] Error 1
[01:07:38] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05086fff
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Aug 14 09:00:11 UTC 2018
Tue, 14 Aug 2018 09:00:11 GMT
travis_tis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] kvm-clock: using sched offset of 1508218310 cycles
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] Zone ranges:
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000]   Device   empty
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] Movable zone start for each node
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] Early memory node ranges
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] Policy zone: Normal
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] Hierarchical RCU implementation.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] console [ttyS0] enabled
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.330210] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.331747] pid_max: default: 32768 minimum: 301
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.332517] ACPI: Core revision 20150930
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.339694] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.340911] Security Framework initialized
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.341815] Yama: becoming mindful.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.342401] AppArmor: AppArmor disabled by boot time parameter
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.345066] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.354880] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.359192] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.360216] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.361564] Initializing cgroup subsys io
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.362350] Initializing cgroup subsys memory
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.363021] Initializing cgroup subsys devices
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.363827] Initializing cgroup subsys freezer
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.364572] Initializing cgroup subsys net_cls
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.365386] Initializing cgroup subsys perf_event
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.366192] Initializing cgroup subsys net_prio
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.367180] Initializing cgroup subsys hugetlb
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.367832] Initializing cgroup subsys pids
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.368593] CPU: Physical Processor ID: 0
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.369197] CPU: Processor Core ID: 0
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.370967] mce: CPU supports 32 MCE banks
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.371773] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.372990] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.376461] Freeing SMP alternatives memory: 32K
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.387276] ftrace: allocating 32185 entries in 126 pages
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.443761] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.445271] smpboot: Max logical packages: 2
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.446582] x2apic enabled
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.448093] Switched APIC routing to physical x2apic.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.452845] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.561268] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.562899] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.566135] x86: Booting SMP configuration:
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.566797] .... node  #0, CPUs:      #1
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.567706] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.572179]  #2
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.572923] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.578696]  #3
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.579162] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.583811] x86: Booted up 1 node, 4 CPUs
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.584861] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.587450] devtmpfs: initialized
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.591863] evm: security.selinux
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.592404] evm: security.SMACK64
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.592906] evm: security.SMACK64EXEC
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.593438] evm: security.SMACK64TRANSMUTE
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.594205] evm: security.SMACK64MMAP
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.594978] evm: security.ima
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.595454] evm: sed kernel: [    0.643231] ACPI: Added _OSI(Processor Device)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.643972] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.644653] ACPI: Added _OSI(Processor Aggregator Device)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.648103] ACPI: Executed 2 blocks of module-level executable AML code
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.671960] ACPI: Interpreter enabled
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.672833] ACPI: (supports S0 S3 S4 S5)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.673538] ACPI: Using IOAPIC for interrupt routing
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.674439] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.704024] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.705067] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.706228] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.707323] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aernel: [    0.750610] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.755395] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.769019] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.774484] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.779148] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.794828] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.798472] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.801725] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.804980] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.808663] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.830715] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.832534] vgaarb: loaded
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.833481] SCSI subsystem initialized
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.834723] libata version 3.00 loaded.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.834753] ACPI: bus type USB registered
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.836006] usbcore: registered new interface driver usbfs
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.837745] usbcore: registered new interface driver hub
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.839262] usbcore: registered new device driver usb
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.841014] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.843008] dmi: Firmware registration failed.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.844256] PCI: Using ACPI for IRQ routing
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.845401] PCI: pci_cache_line_size set to 64 bytes
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.845514] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.845515] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.845652] NetLabel: Initializing
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.846490] NetLabel:  domain hash size = 128
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.847781] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.849200] NetLabel:  unlabeled traffic allowed by default
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.850849] amd_nb: Cannot enumerate AMD northbridges
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.852330] clocksource: Switched to clocksource kvm-clock
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.861304] pnp: PnP ACPI init
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.862498] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.862578] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.862621] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.862668] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.862707] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.862745] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.862783] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.862985] pnp: PnP ACPI: found 7 devices
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.871682] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.874325] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.874328] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.874329] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.874331] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.874368] NET: Registered protocol family 2
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.875744] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.877862] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.879710] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    0.881617] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 07:51:28 trav-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.089456] Scanning for low memory corruption every 60 seconds
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.091727] audit: initializing netlink subsys (disabled)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.093312] audit: type=2000 audit(1534233080.101:1): initialized
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.095678] Initialise system trusted keyring
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.097331] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.099524] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.102645] zbud: loaded
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.104092] VFS: Disk quotas dquot_6.6.0
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.105417] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.108037] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.110964] fuse init (API version 7.23)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.112441] Key type big_key registered
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.113673] Allocating IMA MOK and blacklist keyrings.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.118827] Key type asymmetric registered
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.120777] Asymmetric key parser 'x509' registered
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.122220] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.124573] io scheduler noop registered
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.125798] io scheduler deadline registered (default)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.127371] io scheduler cfq registered
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.128618] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.130178] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.132045] intel_idle: does not run on family 6 model 62
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.132161] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.134427] ACPI: Power Button [PWRF]
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.135610] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 14 07:51:28 travis-job-erface v0.103
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.268607] loop: module loaded
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.270209] libphy: Fixed MDIO Bus: probed
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.271460] tun: Universal TUN/TAP device driver, 1.6
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.273106] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.313844] PPP generic driver version 2.4.2
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.315593] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.317787] ehci-pci: EHCI PCI platform driver
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.320118] ehci-platform: EHCI generic platform driver
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.322093] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.324494] ohci-pci: OHCI PCI platform driver
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.326056] ohci-platform: OHCI generic platform driver
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.327989] uhci_hcd: USB Universal Host Controller Interface driver
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.330359] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.333618] i8042: Warning: Keylock active
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.336597] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.338952] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.340928] mousedev: PS/2 mouse device common for all mice
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.343310] rtc_cmos 00:00: RTC can wake from S4
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.345262] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.347817] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.349839] i2c /dev entries driver
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.351170] device-mapper: uevent: version 1.0.3
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.352926] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.355922] ledtrig-cpu: registered to indicate activity on CPUs
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.359106] NET: Registered protocol family 10
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.361466] NET: Registered protocol family 17
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.362853] Key type dns_resolver registered
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.364818] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.366753] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.369083] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.371122] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.373231] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.376874] registered taskstats version 1
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.378272] Loading compiled-in X.509 certificates
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.380362] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.383575] zswap: loaded using pool lzo/zbud
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.387702] Key type trusted registered
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.394053] Key type encrypted registered
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.395731] ima: No TPM chip found, activating TPM-bypass!
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.397389] evm: HMAC attrs: 0x1
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.398840]   Magic number: 14:144:875
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.400408] rtc_cmos 00:00: setting system clock to 2018-08-14 07:51:20 UTC (1534233080)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.403223] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.404972] EDD information not available.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.406432] PM: Hibernation image not present or could not be loaded.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.408239] Freeing unused kernel memory: 1496K
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.409575] Write protecting the kernel read-only data: 14336k
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.412619] Freeing unused kernel memory: 1956K
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3.414348] Freeing unused kernel memory: 92K
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    3gen() 12279 MB/s
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    8.027810] raid6: .... xor() 8270 MB/s, rmw enabled
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    8.029514] raid6: using ssse3x2 recovery algorithm
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    8.033109] xor: automatically using best checksumming function:
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    8.072375]    avx       : 21611.000 MB/sec
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    8.088371] Btrfs loaded
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    8.139721] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    8.142392] EXT4-fs (sda1): write access will be enabled during recovery
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    8.233274] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    8.250423] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    8.252574] EXT4-fs (sda1): recovery complete
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    8.259835] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    8.473226] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    8.614262] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    8.668833] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    8.882762] random: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    9.480799] random: cloud-init: uninitialized urandom read (32 bytes read, 46 bits of entropy available)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    9.632912] systemd-udevd[702]: starting version 204
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    9.745020] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    9.859616] ppdev: user-space parallel port driver
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [    9.965682] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [   10.025084] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [   10.093506] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [   10.258806] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [   10.539795] random: mktemp: uninitialized urandom read (12 bytes read, 61 bits of entropy available)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [   10.616423] random: mktemp: uninitialized urandom read (6 bytes read, 62 bits of entropy available)
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [   10.694183] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [   10.726318] EXT4-fs (sda1): resized filesystem to 7864064
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [   11.430222] init: failsafe main process (1093) killed by TERM signal
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO Running set_multiqueue.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO Set channels for eth0 to 4.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 14 07:51:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Starting Google Accounts daemon.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Creating a new user account for me.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Created user account me.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Creating a new user account for henrik.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Created user account henrik.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Creating a new user account for emma.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [   12.340130] random: nonblocking pool is initialized
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Created user account emma.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Creating a new user account for igor.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Created user account igor.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Created user account konstantinhaase.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Creating a new user account for aj.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Created user account aj.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Creating a new user account for solarce.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Created user account solarce.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Creating a new user account for asari.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Created user account asari.
Aug 14 07:51:29 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Creating a new user account for bogdana.
Aug 14 07:51:30 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-clock-skew: INFO Synced system time with hardware clock.
Aug 14 07:51:30 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Created user account bogdana.
Aug 14 07:51:30 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Creating a new user account for konstantin.
Aug 14 07:51:30 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [   12.820555] floppy0: no floppy controllers found
Aug 14 07:51:30 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Created user account konstantin.
Aug 14 07:51:30 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Creating a new user account for carmen.
Aug 14 07:51:30 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [   12.862661] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 07:51:30 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [   12.867940] Bridge firewalling registered
Aug 14 07:51:30 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [   12.887498] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 14 07:51:30 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd google-accounts: INFO Created user account carmen.
Aug 14 07:51:30 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [   12.958141] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 07:51:30 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8 input layer
Aug 14 07:51:30 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd acpid: 1 rule loaded
Aug 14 07:51:30 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd acpid: waiting for events: event logging is off
Aug 14 07:51:30 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd haveged: haveged starting up
Aug 14 07:51:30 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [   13.295081] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 07:51:53 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpdate[1843]: adjust time server 169.254.169.254 offset 0.003382 sec
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpd[1880]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpd[1881]: proto: precision = 0.100 usec
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpd[1881]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpd[1881]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpd[1881]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpd[1881]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpd[1881]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpd[1881]: Listen normally on 3 eth0 10.20.0.98 UDP 123
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpd[1881]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpd[1881]: peers refreshed
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpd[1881]: Listening on routing socket on fd #21 for interface updates
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [   43.476617] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd startup-script: INFO Found startup-script in metadata.
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd startup-script: INFO startup-script: job 1 at Tue Aug 14 11:02:00 2018
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd startup-script: INFO startup-script: Return code 0.
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd startup-script: INFO startup-script: Return code 0.
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd startup-script: INFO Finished running startup scripts.
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ec2: 
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ec2: #############################################################
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 14 07:52:00 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ec2: 1024 7c:2c:6f:b8:4d:dd:99:1e:8b:3e:b7:d5:24:f5:c3:49  root@travis-job-7a3df6bc-93d cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 07:53:36 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [  138.832138] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 07:53:36 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [  138.907441] eth0: renamed from veth361b18d
Aug 14 07:53:36 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [  138.952274] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 07:53:36 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [  138.953509] docker0: port 1(vethccdc1fd) entered forwarding state
Aug 14 07:53:36 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [  138.953522] docker0: port 1(vethccdc1fd) entered forwarding state
Aug 14 07:53:36 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [  138.953541] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 14 07:53:39 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpd[1881]: Listen normally on 5 docker0 fe80::42:46ff:febf:2691 UDP 123
Aug 14 07:53:39 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpd[1881]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 14 07:53:39 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpd[1881]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 14 07:53:39 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpd[1881]: peers refreshed
Aug 14 07:53:39 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd ntpd[1881]: new interface(s) found: waking up resolver
Aug 14 07:53:51 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [  153.975135] docker0: port 1(vethccdc1fd) entered forwarding state
Aug 14 08:17:01 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd CRON[16309]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 14 08:39:14 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [ 2876.735660] traps: a[5225] trap invalid opcode ip:55aca13e5a9b sp:7ffed26d2080 error:0 in a[55aca13e2000+6000]
Aug 14 08:39:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [ 2890.829946] traps: a[8060] trap invalid opcode ip:7f83748e4491 sp:7ffd65fd17d0 error:0 in libstd-2339b911e3c09de8.so[7f8374884000+16f000]
Aug 14 08:39:28 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [ 2890.858500] traps: a[8063] trap invalid opcode ip:7f11c0b2a491 sp:7ffeca6e3a80 error:0 in libstd-2339b911e3c09de8.so[7f11c0aca000+16f000]
Aug 14 08:40:46 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [ 2968.781778] traps: a[22901] trap invalid opcode ip:5571444bad98 sp:7fff9b75e120 error:0 in a[5571444b8000+4000]
Aug 14 08:43:25 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [ 3127.849999] a[18987]: segfault at 0 ip 0000557257fdf463 sp 00007ffeafb7dfd0 error 6 in a[557257fdc000+5000]
Aug 14 08:43:33 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [ 3136.309968] a[19729]: segfault at 1 ip 0000564ee4285b8c sp 00007fffcdcf5050 error 6 in a[564ee4283000+4000]
Aug 14 08:43:37 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [ 3140.190597] traps: a[20104] trap invalid opcode ip:55c4de49a42c sp:7ffd6b9cb590 error:0 in a[55c4de497000+7000]
Aug 14 09:00:11 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [ 4133.403727] docker0: port 1(vethccdc1fd) entered disabled state
Aug 14 09:00:11 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [ 4133.403795] veth361b18d: renamed from eth0
Aug 14 09:00:11 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [ 4133.472350] docker0: port 1(vethccdc1fd) entered disabled state
Aug 14 09:00:11 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [ 4133.474284] device vethccdc1fd left promiscuous mode
Aug 14 09:00:11 travis-job-7a3df6bc-93e1-46aa-be0a-8fb5ee8d7ccd kernel: [ 4133.474288] docker0: port 1(vethccdc1fd) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:03dba553
