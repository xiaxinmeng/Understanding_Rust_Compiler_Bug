plain
[00:47:58] ....................................................................................................
[00:48:00] ....................................................................................................
[00:48:03] ....................................................................................................
[00:48:06] ....................................................................................................
[00:48:09] ..............iiiiiiiii.............................................................................
[00:48:14] ....................................................................................................
[00:48:18] ....................i...............................................................................
[00:48:20] ..............................i.....................................................................
[00:48:23] ....................................................................................................
---
[01:05:13] ...................................i................................................................
[01:05:19] ....................................................................................................
[01:05:26] ....................................................................................................
[01:05:32] ....................................................................................................
[01:05:39] .............................................................................................F......
[01:05:52] ....................................................................................................
[01:05:58] ....................................................................................................
[01:06:05] ....................................................................................................
[01:06:11] ....................................................................................................
---
[01:07:10] ................................................i...............................................i...
[01:07:11] ....................
[01:07:11] failures:
[01:07:11] 
[01:07:11] ---- num/mod.rs - num::i8::rotate_right (line 191) stdout ----
[01:07:11] error: invalid suffix `i8i8` for numeric literal
[01:07:11]  --> num/mod.rs:192:9
[01:07:11]   |
[01:07:11] 4 | let n = 0xai8i8;
[01:07:11]   |
[01:07:11]   |
[01:07:11]   = help: the suffix must be one of the integral types (`u32`, `isize`, etc)
[01:07:11] 
[01:07:11] error[E0599]: no method named `rotate_right` found for type `{integer}` in the current scope
[01:07:11]  --> num/mod.rs:195:14
[01:07:11]   |
[01:07:11] 7 | assert_eq!(n.rotate_right(2), m);
[01:07:11] 
[01:07:11] thread 'num/mod.rs - num::i8::rotate_right (line 191)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:07:11] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:11] 
---
[01:07:11] 
[01:07:11] error: test failed, to rerun pass '--doc'
[01:07:11] 
[01:07:11] 
[01:07:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:07:11] 
[01:07:11] 
[01:07:11] 
[01:07:11] failoogle GOOGFACP 00000001 GOOG 00000001)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  9 11:32:12 trav travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] Zone ranges:
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000]   Device   empty
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] Movable zone start for each node
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] Early memory node ranges
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 rsyslogd-2307: warning: ~ action is deprecated, consider using the 'stop' statement instead [try http://www.rsyslog.com/e/2307 ]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 rsyslogd: rsyslogd's groupid changed to 104
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 rsyslogd: rsyslogd's userid changed to 101
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [on.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] console [ttyS0] enabled
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.305807] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.306958] pid_max: default: 32768 minimum: 301
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.307676] ACPI: Core revision 20150930
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.313913] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.315040] Security Framework initialized
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.315606] Yama: becoming mindful.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.316098] AppArmor: AppArmor disabled by boot time parameter
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.318503] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.327319] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.331321] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.332334] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.333912] Initializing cgroup subsys io
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.334575] Initializing cgroup subsys memory
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.335443] Initializing cgroup subsys devices
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.336114] Initializing cgroup subsys freezer
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.336746] Initializing cgroup subsys net_cls
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.337557] Initializing cgroup subsys perf_event
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.338202] Initializing cgroup subsys net_prio
Aug-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.406870] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.513069] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.514755] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.516915] x86: Booting SMP configuration:
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.517523] .... node  #0, CPUs:      #1
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.518276] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.521506]  #2
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.521920] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.525751]  #3
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.526195] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.529332] x86: Booted up 1 node, 4 CPUs
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.529992] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: 9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.569117] cpuidle: using governor menu
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.569777] PCCT header not found.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.570369] ACPI: bus type PCI registered
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.570963] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.572085] PCI: Using configuration type 1 for base access
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.585893] ACPI: Added _OSI(Module Device)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.586682] ACPI: Added _OSI(Processor Device)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.587293] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.587961] ACPI: Added _OSI(Processor Aggregator Device)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.591323] ACPI: Executed 2 blocks of module-level executable AML code
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.614314] ACPI: Interpreter enabled
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.615243] ACPI: (supports S0 S3 S4 S5)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.615800] ACPI: Using IOAPIC for interrupt routing
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.616571] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.645826] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.646862] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.647792] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.648689] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.651082] PCI host bridge to bus 0000:00
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.651725] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.652651] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.653689] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.654749] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.655784] pci_bus 0000:00: root bus   0.729406] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.731407] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.733430] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.735329] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.755362] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.756610] vgaarb: loaded
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.757220] SCSI subsystem initialized
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.757872] libata version 3.00 loaded.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.757894] ACPI: bus type USB registered
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.758621] usbcore: registered new interface driver usbfs
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.759393] usbcore: registered new interface driver hub
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.760158] usbcore: registered new device driver usb
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    0.761152] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb5046590istered rtc_cmos as rtc0
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.941932] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.943483] i2c /dev entries driver
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.944264] device-mapper: uevent: version 1.0.3
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.945339] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.946992] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.948723] NET: Registered protocol family 10
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.949797] NET: Registered protocol family 17
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.950773] Key type dns_resolver registered
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.951673] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.953063] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.953969] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.955421] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.956578] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.958378] registered taskstats version 1
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.959057] Loading compiled-in X.509 certificates
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.960559] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.962589] zswap: loaded using pool lzo/zbud
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.965130] Key type trusted registered
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.969146] Key type encrypted registered
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.969984] ima: No TPM chip found, activating TPM-bypass!
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.971103] evm: HMAC attrs: 0x1
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.971893]   Magic number: 14:331:529
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.972953] rtc_cmos 00:00: setting system clock to 2018-08-09 11:32:05 UTC (1533814325)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.974810] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.976074] EDD information not available.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.977029] PM: Hibernation image not present or could not be loaded.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.978329] Freeing unused kernel memory: 1496K
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.978974] Write protecting the kernel read-only data: 14336k
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.980533] Freeing unused kernel memory: 1956K
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.981447] Freeing unused kernel memory: 92K
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    2.993643] systemd-udevd[119]: starting version 204
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    3.048180] scsi host0: Virtio SCSI HBA
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    3.052775] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    3.059230] AVX version of gcm_enc/dec engaged.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    3.060096] AES CTR mode by8 optimization enabled
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    3.095824] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    3.095825] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    3.097672] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    3.098865] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    3.099658] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    3.099739] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    3.102404]  sda: sda1
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    3.103524] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    3.139862] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    3.755418] tsc: Refined TSC clocksource calibration: 2600.000 MHz
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    3.756327] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3c3232d, max_idle_ns: 440795236700 ns
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    3.972877] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    6.043435] floppy0: no floppy controllers found
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    7.195343] raid6: sse2x1   gen()  9047 MB/s
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    7.263331] raid6: sse2x1   xor()  6732 MB/s
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    7.331329] raid6: sse2x2   gen() 10928 MB/s
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    7.399330] raid6: sse2x2   xor()  7328 MB/s
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    7.467329] raid6: sse2x4   gen() 12865 MB/s
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    7.535345] raid6: sse2x4   xor()  9166 MB/s
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    7.536045] raid6: using algorithm sse2x4 gen() 12865 MB/s
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    7.536900] raid6: .... xor() 9166 MB/s, rmw enabled
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    7.537610] raid6: using ssse3x2 recovery algorithm
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    7.539618] xor: automatically using best checksumming function:
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    7.579346]    avx       : 28034.000 MB/sec
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    7.593046] Btrfs loaded
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    7.628433] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb504659-a2ba-fb50465900c8 kernel: [    8.877853] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    8.920480] intel_rapl: no valid rapl domains found in package 0
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    8.960146] intel_rapl: no valid rapl domains found in package 0
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    8.964897] ppdev: user-space parallel port driver
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    9.061706] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    9.103999] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    9.158219] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    9.321104] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    9.564131] random: mktemp: uninitialized urandom read (12 bytes read, 60 bits of entropy available)
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [    9.630740] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug  9 11:32:12 travis-job-d5202ca6nity 1
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  9 11:32:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 google-accounts: INFO Starting Google Accounts daemon.
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 google-accounts: INFO Creating a new user account for me.
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 google-accounts: INFO Created user account me.
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 google-accounts: INFO Creating a new user account for bogdana.
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 google-accounts: INFO Created user account bogdana.
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [   10.755968] random: nonblocking pool is initialized
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 google-accounts: INFO Creating a new user account for aj.
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 google-accounts: INFO Created user account aj.
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 google-accounts: INFO Creating a new user account for asari.
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 google-accounts: INFO Created user account asari.
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 google-accounts: INFO Removing user packer.
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 cron[1434]: (CRON) INFO (pidfile fd = 3)
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 cron[1474]: (CRON) STARTUP (fork ok)
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 cron[1474]: (CRON) INFO (Running @reboot jobs)
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 acpid: starting up with netlink and the input layer
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 acpid: 1 rule loaded
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 acpid: waiting for events: event logging is off
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 haveged: haveged starting up
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [   11.220308] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 11:32:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [   11.232382] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 11:32:14 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 11:32:14 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [   11.417686] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  9 11:32:14 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [   11.421813] Bridge firewalling registered
Aug  9 11:32:14 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [   11.430760] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 11:32:14 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [   11.485751] Initializing XFRM netlink socket
Aug  9 11:32:14 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [   11.491998] Netfilter messages via NETLINK v0.30.
Aug  9 11:32:14 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [   11.494295] ctnetlink v0.93: registering with nfnetlink.
Aug  9 11:32:14 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [   11.923441] floppy0: no floppy controllers found
Aug  9 11:32:37 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 ntpdate[1790]: adjust time server 169.254.169.254job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  9 11:32:44 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 startup-script: INFO startup-script: job 1 at Thu Aug  9 14:42:00 2018
Aug  9 11:32:44 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 startup-script: INFO startup-script: Return code 0.
Aug  9 11:32:44 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 startup-script: INFO Finished running startup scripts.
Aug  9 11:32:44 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 startup-script: INFO Finished running startup scripts.
Aug  9 11:32:44 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 ec2: 
Aug  9 11:32:44 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 ec2: #############################################################
Aug  9 11:32:44 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 11:32:44 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 ec2: 1024 08:e8:5f:27:a3:1e:29:1f:90:f8:10:02:34:77:47:77  root@travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 (DSA)
Aug  9 11:32:44 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 ec2: 256 4b:ce:10:d1:b8:58:91:d8:cc:24:d8:65:a1:10:31:b8  root@travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 (ECDSA)
Aug  9 11:32:44 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 ec2: 256 4d:86:57:70:06:3e:b6:87:1b:13:e8:e8:4e:a6:c9:66  root@travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 (ED25519)
Aug  9 11:32:44 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 ec2: 2048 03:68:02:b3:43:8c:6a:51:2a:2a:8c:24:1c:67:59:b0  root@travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 (RSA)
Aug  9 11:32:44 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 ec2: -----END SSH HOST KEY FINGERPRIN53] traps: a[8191] trap invalid opcode ip:7f0adfa0dea1 sp:7ffdb1e37f30 error:0 in libstd-2339b911e3c09de8.so[7f0adf9b3000+172000]
Aug  9 12:24:12 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [ 3129.626532] traps: a[23089] trap invalid opcode ip:55a33d20fd68 sp:7ffd1f08fc50 error:0 in a[55a33d20d000+4000]
Aug  9 12:26:55 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [ 3292.507220] a[19282]: segfault at 0 ip 000055aca6f9e548 sp 00007ffd1dc56c60 error 6 in a[55aca6f9b000+5000]
Aug  9 12:27:04 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [ 3301.848815] a[20036]: segfault at 1 ip 000055f6e8ee6b5c sp 00007fffd1d67470 error 6 in a[55f6e8ee4000+4000]
Aug  9 12:27:08 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [ 3305.837463] traps: a[20408] trap invalid opcode ip:5594f83e442c sp:7ffc3702d250 error:0 in a[5594f83e1000+7000]
Aug  9 12:41:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [ 4150.831937] vethe7e172c: renamed from eth0
Aug  9 12:41:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [ 4150.844664] docker0: port 1(veth54e7cdc) entered disabled state
Aug  9 12:41:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [ 4150.902449] docker0: port 1(veth54e7cdc) entered disabled state
Aug  9 12:41:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [ 4150.904001] device veth54e7cdc left promiscuous mode
Aug  9 12:41:13 travis-job-d5202ca6-2a4b-4600-a2ba-fb50465900c8 kernel: [ 4150.904004] docker0: port 1(veth54e7cdc) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:1dfb9858
