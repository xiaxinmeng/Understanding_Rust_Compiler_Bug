plain

[00:04:03] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:03] tidy error: /checkout/src/librustc_resolve/lib.rs:4274: line longer than 100 chars
[00:04:04] some tidy checks failed
[00:04:04] 
[00:04:04] 
[00:04:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:04] 
[00:04:04] 
[00:04:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:04] Build completed unsuccessfully in 0:00:55
[00:04:04] Build completed unsuccessfully in 0:00:55
[00:04:04] make: *** [tidy] Error 1
[00:04:04] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ac1c360
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:123f8040
$ sudo tail -n 500 /var/log/syslog
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] kvm-clock: using sched offset of 1493167968 cycles
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] Zone ranges:
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000]   Device   empty
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] Movable zone start for each node
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] Early memory node ranges
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] Policy zone: Normal
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] Hierarchical RCU implementation.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] console [ttyS0] enabled
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.315102] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.316604] pid_max: default: 32768 minimum: 301
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.317469] ACPI: Core revision 20150930
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.323818] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.325130] Security Framework initialized
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.325880] Yama: becoming mindful.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.326614] AppArmor: AppArmor disabled by boot time parameter
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.329101] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.337984] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.342428] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.343642] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.345325] Initializing cgroup subsys io
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.346422] Initializing cgroup subsys memory
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.347422] Initializing cgroup subsys devices
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.348302] Initializing cgroup subsys freezer
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.350743] Initializing cgroup subsys net_cls
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.351721] Initializing cgroup subsys perf_event
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.352789] Initializing cgroup subsys net_prio
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.353687] Initializing cgroup subsys hugetlb
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.354710] Initializing cgroup subsys pids
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.355786] CPU: Physical Processor ID: 0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.357078] CPU: Processor Core ID: 0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.358429] mce: CPU supports 32 MCE banks
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.359486] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.360731] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.364336] Freeing SMP alternatives memory: 32K
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.374273] ftrace: allocating 32185 entries in 126 pages
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.428206] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.429781] smpboot: Max logical packages: 2
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.431282] x2apic enabled
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.432964] Switched APIC routing to physical x2apic.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.437219] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.546224] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.548100] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.550326] x86: Booting SMP configuration:
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.551312] .... node  #0, CPUs:      #1
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.552182] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.556599]  #2
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.557182] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.562254]  #3
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.562718] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.566802] x86: Booted up 1 node, 4 CPUs
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.567567] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.569854] devtmpfs: initialized
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.573929] evm: security.selinux
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.574916] evm: security.SMACK64
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.575386] evm: security.SMACK64EXEC
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.575952] evm: security.SMACK64TRANSMUTE
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.576516] evm: security.SMACK64MMAP
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.577060] evm: security.ima
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.577480] evm: security.capability
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.578257] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.579647] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.580741] pinctrl core: initialized pinctrl subsystem
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.581828] RTC time:  2:15:30, date: 08/10/18
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.583727] NET: Registered protocol family 16
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.594253] cpuidle: using governor ladder
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.606256] cpuidle: using governor menu
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.607429] PCCT header not found.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.608120] ACPI: bus type PCI registered
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.608699] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.609823] PCI: Using configuration type 1 for base access
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.623375] ACPI: Added _OSI(Module Device)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.624151] ACPI: Added _OSI(Processor Device)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.624869] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.625613] ACPI: Added _OSI(Processor Aggregator Device)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.629170] ACPI: Executed 2 blocks of module-level executable AML code
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.652195] ACPI: Interpreter enabled
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.652942] ACPI: (supports S0 S3 S4 S5)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.653528] ACPI: Using IOAPIC for interrupt routing
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.654258] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.683446] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.684509] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.685479] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.686707] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.689295] PCI host bridge to bus 0000:00
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.690104] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.691136] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.692169] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.693529] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.695171] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.696464] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.696949] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.712340] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.726813] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.728295] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.733447] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.737204] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.749225] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.754138] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.758834] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.771251] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.773923] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.776111] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.778193] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.780118] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.800602] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.801789] vgaarb: loaded
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.802748] SCSI subsystem initialized
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.803402] libata version 3.00 loaded.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.803423] ACPI: bus type USB registered
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.805100] usbcore: registered new interface driver usbfs
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.806366] usbcore: registered new interface driver hub
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.807187] usbcore: registered new device driver usb
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.808116] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.810756] dmi: Firmware registration failed.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.812716] PCI: Using ACPI for IRQ routing
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.813481] PCI: pci_cache_line_size set to 64 bytes
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.813590] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.813592] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.813719] NetLabel: Initializing
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.814266] NetLabel:  domain hash size = 128
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.814992] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.815684] NetLabel:  unlabeled traffic allowed by default
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.816615] amd_nb: Cannot enumerate AMD northbridges
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.818011] clocksource: Switched to clocksource kvm-clock
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.825655] pnp: PnP ACPI init
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.826810] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.826880] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.826928] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.826979] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.827022] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.827071] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.827117] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.827273] pnp: PnP ACPI: found 7 devices
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.834766] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.836487] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.836490] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.836491] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.836493] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.836526] NET: Registered protocol family 2
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.837448] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.839570] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.842164] TCP: Hash tables configured (established 131072 bind 65536)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.843232] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.844138] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.845251] NET: Registered protocol family 1
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.846004] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.847242] PCI: CLS 0 bytes, default 64
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    0.847975] Unpacking initramfs...
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.877004] Freeing initrd memory: 21432K
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.878162] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.880063] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.882590] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.884818] hw unit of domain pp0-core 2^-0 Joules
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.886156] hw unit of domain package 2^-0 Joules
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.887070] hw unit of domain dram 2^-16 Joules
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.887947] Scanning for low memory corruption every 60 seconds
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.890302] audit: initializing netlink subsys (disabled)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.891279] audit: type=2000 audit(1533867332.917:1): initialized
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.892821] Initialise system trusted keyring
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.894276] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.896032] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.898097] zbud: loaded
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.899158] VFS: Disk quotas dquot_6.6.0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.900202] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.902329] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.904127] fuse init (API version 7.23)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.905051] Key type big_key registered
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.905938] Allocating IMA MOK and blacklist keyrings.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.908588] Key type asymmetric registered
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.909482] Asymmetric key parser 'x509' registered
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.910499] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.912017] io scheduler noop registered
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.912658] io scheduler deadline registered (default)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.913782] io scheduler cfq registered
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.914763] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.915614] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.917100] intel_idle: does not run on family 6 model 63
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.917185] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.918503] ACPI: Power Button [PWRF]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.919136] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.920558] ACPI: Sleep Button [SLPF]
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.921445] GHES: HEST is not enabled!
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.923806] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.924918] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.929788] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.931049] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.935790] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.958898] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    2.982294] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.006363] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.030077] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.033863] Linux agpgart interface v0.103
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.037148] loop: module loaded
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.038218] libphy: Fixed MDIO Bus: probed
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.039471] tun: Universal TUN/TAP device driver, 1.6
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.040923] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.069594] PPP generic driver version 2.4.2
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.071103] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.073026] ehci-pci: EHCI PCI platform driver
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.074818] ehci-platform: EHCI generic platform driver
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.076269] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.077940] ohci-pci: OHCI PCI platform driver
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.079151] ohci-platform: OHCI generic platform driver
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.080481] uhci_hcd: USB Universal Host Controller Interface driver
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.082686] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.085326] i8042: Warning: Keylock active
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.087484] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.088877] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.090895] mousedev: PS/2 mouse device common for all mice
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.092723] rtc_cmos 00:00: RTC can wake from S4
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.094279] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.096269] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.098076] i2c /dev entries driver
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.099152] device-mapper: uevent: version 1.0.3
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.100510] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.102999] ledtrig-cpu: registered to indicate activity on CPUs
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.106610] NET: Registered protocol family 10
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.108036] NET: Registered protocol family 17
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.109300] Key type dns_resolver registered
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.110843] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.112507] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.114342] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.116412] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.118204] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.120906] registered taskstats version 1
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.122031] Loading compiled-in X.509 certificates
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.124284] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.127136] zswap: loaded using pool lzo/zbud
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.130052] Key type trusted registered
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.133914] Key type encrypted registered
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.135220] ima: No TPM chip found, activating TPM-bypass!
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.136747] evm: HMAC attrs: 0x1
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.138647]   Magic number: 14:690:257
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.139914] rtc_cmos 00:00: setting system clock to 2018-08-10 02:15:33 UTC (1533867333)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.143155] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.146703] EDD information not available.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.148832] PM: Hibernation image not present or could not be loaded.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.150314] Freeing unused kernel memory: 1496K
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.151146] Write protecting the kernel read-only data: 14336k
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.152904] Freeing unused kernel memory: 1956K
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.153985] Freeing unused kernel memory: 92K
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.166877] systemd-udevd[119]: starting version 204
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.223696] scsi host0: Virtio SCSI HBA
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.232095] AVX2 version of gcm_enc/dec engaged.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.236005] AES CTR mode by8 optimization enabled
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.237810] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.279847] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.279903] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.279904] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.279995] sd 0:0:1:0: [sda] Write Protect is off
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.279997] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.280021] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.281750]  sda: sda1
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.282330] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.294341] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.886130] tsc: Refined TSC clocksource calibration: 2300.001 MHz
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    3.887631] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735f0517, max_idle_ns: 440795237604 ns
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    4.131122] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    6.226157] floppy0: no floppy controllers found
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    7.382036] raid6: sse2x1   gen()  8782 MB/s
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    7.450062] raid6: sse2x1   xor()  6849 MB/s
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    7.518047] raid6: sse2x2   gen() 10846 MB/s
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    7.586034] raid6: sse2x2   xor()  7201 MB/s
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    7.654052] raid6: sse2x4   gen() 12262 MB/s
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    7.722042] raid6: sse2x4   xor()  9088 MB/s
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    7.790033] raid6: avx2x1   gen() 17163 MB/s
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    7.858038] raid6: avx2x2   gen() 19833 MB/s
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    7.926043] raid6: avx2x4   gen() 23012 MB/s
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    7.926955] raid6: using algorithm avx2x4 gen() 23012 MB/s
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    7.927844] raid6: using avx2x2 recovery algorithm
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    7.930112] xor: automatically using best checksumming function:
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    7.970040]    avx       : 27484.000 MB/sec
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    7.983854] Btrfs loaded
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    8.045799] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    8.046939] EXT4-fs (sda1): write access will be enabled during recovery
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    8.135733] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    8.142716] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    8.143430] EXT4-fs (sda1): recovery complete
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    8.148361] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    8.362988] random: init: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    8.489768] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    8.535554] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    8.749614] random: cloud-init: uninitialized urandom read (32 bytes read, 39 bits of entropy available)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    9.303275] random: cloud-init: uninitialized urandom read (32 bytes read, 47 bits of entropy available)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    9.442157] systemd-udevd[701]: starting version 204
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    9.559123] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    9.625574] intel_rapl: no valid rapl domains found in package 0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    9.693873] ppdev: user-space parallel port driver
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    9.780928] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    9.825981] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [    9.888337] random: cloud-init: uninitialized urandom read (32 bytes read, 60 bits of entropy available)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   10.044635] random: cloud-init: uninitialized urandom read (32 bytes read, 60 bits of entropy available)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   10.300346] random: mktemp: uninitialized urandom read (12 bytes read, 62 bits of entropy available)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   10.375818] random: mktemp: uninitialized urandom read (6 bytes read, 63 bits of entropy available)
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   10.446996] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   10.491490] EXT4-fs (sda1): resized filesystem to 7864064
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   10.857411] init: failsafe main process (1092) killed by TERM signal
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO Running set_multiqueue.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO Set channels for eth0 to 4.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 10 02:15:41 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   11.600044] random: nonblocking pool is initialized
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-clock-skew: INFO Synced system time with hardware clock.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Starting Google Accounts daemon.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Creating a new user account for me.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Created user account me.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Creating a new user account for henrik.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Created user account henrik.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Creating a new user account for emma.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de cron[1403]: (CRON) INFO (pidfile fd = 3)
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de cron[1457]: (CRON) STARTUP (fork ok)
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de cron[1457]: (CRON) INFO (Running @reboot jobs)
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Created user account emma.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Creating a new user account for igor.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de acpid: starting up with netlink and the input layer
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de acpid: 1 rule loaded
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de acpid: waiting for events: event logging is off
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Created user account igor.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de haveged: haveged starting up
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Created user account konstantinhaase.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Creating a new user account for aj.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   12.155842] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   12.166319] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Created user account aj.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Creating a new user account for solarce.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Created user account solarce.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Creating a new user account for asari.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   12.267478] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   12.271421] Bridge firewalling registered
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   12.284157] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Created user account asari.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Creating a new user account for bogdana.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   12.364332] Initializing XFRM netlink socket
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Created user account bogdana.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   12.373781] Netfilter messages via NETLINK v0.30.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   12.376920] ctnetlink v0.93: registering with nfnetlink.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Creating a new user account for konstantin.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Created user account konstantin.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Creating a new user account for carmen.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Created user account carmen.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Creating a new user account for maria.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Created user account maria.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de google-accounts: INFO Removing user packer.
Aug 10 02:15:42 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   12.630111] floppy0: no floppy controllers found
Aug 10 02:16:05 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpdate[1860]: adjust time server 169.254.169.254 offset 0.002156 sec
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpd[1896]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpd[1897]: proto: precision = 0.171 usec
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpd[1897]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpd[1897]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpd[1897]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpd[1897]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpd[1897]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpd[1897]: Listen normally on 3 eth0 10.20.0.42 UDP 123
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpd[1897]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpd[1897]: peers refreshed
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpd[1897]: Listening on routing socket on fd #21 for interface updates
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [   42.339685] init: plymouth-upstart-bridge main process ended, respawning
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de startup-script: INFO Found startup-script in metadata.
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de startup-script: INFO startup-script: job 1 at Fri Aug 10 05:26:00 2018
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de startup-script: INFO startup-script: Return code 0.
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de startup-script: INFO startup-script: Return code 0.
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de startup-script: INFO Finished running startup scripts.
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ec2: 
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ec2: #############################################################
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ec2: 1024 ac:a7:c2:52:eb:54:32:90:27:34:7b:8e:84:f5:6f:56  root@travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de (DSA)
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ec2: 256 3d:bd:30:27:5d:0e:90:cc:37:f5:44:8e:64:48:87:d2  root@travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de (ECDSA)
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ec2: 256 ef:48:d0:88:6a:4a:df:a7:df:4a:87:5f:0a:85:b4:59  root@travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de (ED25519)
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ec2: 2048 33:23:92:32:6b:51:0c:90:b6:82:19:8f:d2:b8:d6:32  root@travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de (RSA)
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 10 02:16:12 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ec2: #############################################################
Aug 10 02:17:01 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de CRON[3444]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 10 02:17:33 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [  123.184593] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 10 02:18:34 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [  183.761473] device vethce45cb7 entered promiscuous mode
Aug 10 02:18:34 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [  183.761585] docker0: port 1(vethce45cb7) entered forwarding state
Aug 10 02:18:34 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [  183.761599] docker0: port 1(vethce45cb7) entered forwarding state
Aug 10 02:18:34 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [  183.761936] docker0: port 1(vethce45cb7) entered disabled state
Aug 10 02:18:34 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [  183.845152] cgroup: docker-runc (4894) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 10 02:18:34 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [  183.845155] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 10 02:18:34 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [  183.908746] eth0: renamed from veth38de7bf
Aug 10 02:18:34 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [  183.946560] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 10 02:18:34 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [  183.947546] docker0: port 1(vethce45cb7) entered forwarding state
Aug 10 02:18:34 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [  183.947561] docker0: port 1(vethce45cb7) entered forwarding state
Aug 10 02:18:34 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [  183.947585] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 10 02:18:37 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpd[1897]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 10 02:18:37 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpd[1897]: Listen normally on 6 docker0 fe80::42:e1ff:fe89:4a94 UDP 123
Aug 10 02:18:37 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpd[1897]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 10 02:18:37 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpd[1897]: peers refreshed
Aug 10 02:18:37 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de ntpd[1897]: new interface(s) found: waking up resolver
Aug 10 02:18:49 travis-job-533c5f9f-65ff-4cd0-9110-3927a84aa1de kernel: [  198.952682] docker0: port 1(vethce45cb7) entered forwarding state
---
travis_time:end:1554a4b8:start=1533867699827382921,finish=1533867699832750935,duration=5368014
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f026d31
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12f5877c
travis_time:start:12f5877c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0225f8ab
$ dmesg | grep -i kill
