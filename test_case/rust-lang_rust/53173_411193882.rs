plain

[00:04:05] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:05] tidy error: binary checked into source: /checkout/src/librustc_target/spec/aarch64_pc_windows_msvc.rs
[00:04:05] tidy error: /checkout/src/librustc_target/spec/aarch64_pc_windows_msvc.rs:18: TODO is deprecated; use FIXME
[00:04:05] tidy error: /checkout/src/libpanic_unwind/macros.rs: incorrect license
[00:04:07] some tidy checks failed
[00:04:07] 
[00:04:07] 
[00:04:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:07] 
[00:04:07] 
[00:04:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:07] Build completed unsuccessfully in 0:00:52
[00:04:07] Build completed unsuccessfully in 0:00:52
[00:04:07] Makefile:79: recipe for target 'tidy' failed
[00:04:07] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b092e78
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:07f6db50
$ sudo tail -n 500 /var/log/syslog
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] kvm-clock: using sched offset of 1323240238 cycles
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] Zone ranges:
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000]   Device   empty
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] Movable zone start for each node
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] Early memory node ranges
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] Policy zone: Normal
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] console [ttyS0] enabled
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.318001] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.319375] pid_max: default: 32768 minimum: 301
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.320163] ACPI: Core revision 20150930
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.326753] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.327980] Security Framework initialized
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.328665] Yama: becoming mindful.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.329330] AppArmor: AppArmor disabled by boot time parameter
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.331920] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.341012] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.345647] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.346606] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.347961] Initializing cgroup subsys io
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.348538] Initializing cgroup subsys memory
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.349140] Initializing cgroup subsys devices
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.349987] Initializing cgroup subsys freezer
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.350750] Initializing cgroup subsys net_cls
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.351519] Initializing cgroup subsys perf_event
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.352377] Initializing cgroup subsys net_prio
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.353173] Initializing cgroup subsys hugetlb
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.354024] Initializing cgroup subsys pids
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.354822] CPU: Physical Processor ID: 0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.355494] CPU: Processor Core ID: 0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.356162] mce: CPU supports 32 MCE banks
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.356912] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.357887] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.360585] Freeing SMP alternatives memory: 32K
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.368894] ftrace: allocating 32185 entries in 126 pages
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.415359] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.416550] smpboot: Max logical packages: 2
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.417732] x2apic enabled
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.419141] Switched APIC routing to physical x2apic.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.422686] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.529260] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.531109] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.533867] x86: Booting SMP configuration:
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.534749] .... node  #0, CPUs:      #1
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.535815] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.539185]  #2
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.539620] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.542850]  #3
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.543338] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.546550] x86: Booted up 1 node, 4 CPUs
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.547859] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.550625] devtmpfs: initialized
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.555043] evm: security.selinux
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.555691] evm: security.SMACK64
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.556285] evm: security.SMACK64EXEC
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.556835] evm: security.SMACK64TRANSMUTE
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.557643] evm: security.SMACK64MMAP
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.558194] evm: security.ima
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.558810] evm: security.capability
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.559758] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.562565] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.563876] pinctrl core: initialized pinctrl subsystem
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.565429] RTC time: 20:24:22, date: 08/07/18
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.566957] NET: Registered protocol family 16
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.577293] cpuidle: using governor ladder
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.589300] cpuidle: using governor menu
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.590055] PCCT header not found.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.590660] ACPI: bus type PCI registered
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.591252] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.592510] PCI: Using configuration type 1 for base access
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.606490] ACPI: Added _OSI(Module Device)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.607236] ACPI: Added _OSI(Processor Device)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.607953] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.608659] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.611872] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.634913] ACPI: Interpreter enabled
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.635869] ACPI: (supports S0 S3 S4 S5)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.636414] ACPI: Using IOAPIC for interrupt routing
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.637127] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.666429] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.667355] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.668315] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.669227] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.671738] PCI host bridge to bus 0000:00
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.672622] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.673612] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.675053] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.676257] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.677306] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.678227] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.678796] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.692687] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.707226] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.708622] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.713744] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.717599] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.729856] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.735578] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.739744] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.752257] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.754808] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.757235] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.759483] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.761613] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.781789] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.783061] vgaarb: loaded
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.783926] SCSI subsystem initialized
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.784735] libata version 3.00 loaded.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.784757] ACPI: bus type USB registered
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.785535] usbcore: registered new interface driver usbfs
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.786568] usbcore: registered new interface driver hub
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.787669] usbcore: registered new device driver usb
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.788657] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.789822] dmi: Firmware registration failed.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.790732] PCI: Using ACPI for IRQ routing
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.791433] PCI: pci_cache_line_size set to 64 bytes
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.791527] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.791529] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.791665] NetLabel: Initializing
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.792168] NetLabel:  domain hash size = 128
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.792781] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.793604] NetLabel:  unlabeled traffic allowed by default
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.794602] amd_nb: Cannot enumerate AMD northbridges
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.795382] clocksource: Switched to clocksource kvm-clock
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.803406] pnp: PnP ACPI init
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.804001] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.804074] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.804127] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.804179] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.804221] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.804271] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.804321] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.804492] pnp: PnP ACPI: found 7 devices
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.812257] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.813991] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.813994] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.813995] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.813997] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.814035] NET: Registered protocol family 2
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.815075] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.816603] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.817674] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.818849] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.819781] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.822003] NET: Registered protocol family 1
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.822932] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.824066] PCI: CLS 0 bytes, default 64
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    0.824131] Unpacking initramfs...
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.828770] Freeing initrd memory: 21432K
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.829742] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.830803] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.833442] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.835075] hw unit of domain pp0-core 2^-0 Joules
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.835886] hw unit of domain package 2^-0 Joules
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.836581] hw unit of domain dram 2^-0 Joules
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.837490] Scanning for low memory corruption every 60 seconds
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.838793] audit: initializing netlink subsys (disabled)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.839730] audit: type=2000 audit(1533673464.446:1): initialized
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.840867] Initialise system trusted keyring
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.842028] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.843187] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.845376] zbud: loaded
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.846274] VFS: Disk quotas dquot_6.6.0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.847019] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.848581] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.849762] fuse init (API version 7.23)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.850767] Key type big_key registered
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.851481] Allocating IMA MOK and blacklist keyrings.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.853581] Key type asymmetric registered
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.854207] Asymmetric key parser 'x509' registered
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.854954] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.856113] io scheduler noop registered
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.856691] io scheduler deadline registered (default)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.857434] io scheduler cfq registered
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.858071] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.859007] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.859996] intel_idle: does not run on family 6 model 45
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.860085] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.861458] ACPI: Power Button [PWRF]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.862110] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.863416] ACPI: Sleep Button [SLPF]
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.864338] GHES: HEST is not enabled!
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.866590] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.867663] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.871839] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.872739] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.877426] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.900122] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.925564] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.949682] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.973772] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.977677] Linux agpgart interface v0.103
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.981064] loop: module loaded
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.982304] libphy: Fixed MDIO Bus: probed
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.983566] tun: Universal TUN/TAP device driver, 1.6
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    2.985307] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.019841] PPP generic driver version 2.4.2
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.021575] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.023500] ehci-pci: EHCI PCI platform driver
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.025171] ehci-platform: EHCI generic platform driver
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.026693] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.028305] ohci-pci: OHCI PCI platform driver
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.029682] ohci-platform: OHCI generic platform driver
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.031158] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.033424] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.037164] i8042: Warning: Keylock active
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.039372] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.040878] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.042826] mousedev: PS/2 mouse device common for all mice
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.045067] rtc_cmos 00:00: RTC can wake from S4
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.046781] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.049070] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.050755] i2c /dev entries driver
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.052171] device-mapper: uevent: version 1.0.3
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.053593] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.056259] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.058886] NET: Registered protocol family 10
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.060369] NET: Registered protocol family 17
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.061373] Key type dns_resolver registered
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.062768] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.064376] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.065777] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.067099] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.068426] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.070974] registered taskstats version 1
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.072057] Loading compiled-in X.509 certificates
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.074084] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.076957] zswap: loaded using pool lzo/zbud
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.080533] Key type trusted registered
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.085310] Key type encrypted registered
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.086735] ima: No TPM chip found, activating TPM-bypass!
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.088168] evm: HMAC attrs: 0x1
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.089746]   Magic number: 14:35:447
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.090834] acpi LNXCPU:fe: hash matches
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.092129] rtc_cmos 00:00: setting system clock to 2018-08-07 20:24:24 UTC (1533673464)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.094160] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.096285] EDD information not available.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.097459] PM: Hibernation image not present or could not be loaded.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.098882] Freeing unused kernel memory: 1496K
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.099570] Write protecting the kernel read-only data: 14336k
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.101374] Freeing unused kernel memory: 1956K
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.102330] Freeing unused kernel memory: 92K
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.116285] systemd-udevd[119]: starting version 204
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.168424] scsi host0: Virtio SCSI HBA
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.173862] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.178552] AVX version of gcm_enc/dec engaged.
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.179281] AES CTR mode by8 optimization enabled
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.208988] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.209888] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.210993] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.212186] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.212895] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.213036] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.216565]  sda: sda1
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.218201] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.247690] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.835535] tsc: Refined TSC clocksource calibration: 2600.000 MHz
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    3.836997] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3c3232d, max_idle_ns: 440795236700 ns
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    4.080530] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    6.163606] floppy0: no floppy controllers found
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.319393] raid6: sse2x1   gen()  9001 MB/s
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.387392] raid6: sse2x1   xor()  6715 MB/s
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.455392] raid6: sse2x2   gen() 11085 MB/s
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.523396] raid6: sse2x2   xor()  7358 MB/s
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.591394] raid6: sse2x4   gen() 12598 MB/s
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.659397] raid6: sse2x4   xor()  8842 MB/s
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.660374] raid6: using algorithm sse2x4 gen() 12598 MB/s
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.661190] raid6: .... xor() 8842 MB/s, rmw enabled
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.661921] raid6: using ssse3x2 recovery algorithm
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.664162] xor: automatically using best checksumming function:
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.703397]    avx       : 27444.000 MB/sec
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.717049] Btrfs loaded
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.758962] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.760083] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.835423] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.842638] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.843659] EXT4-fs (sda1): recovery complete
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    7.848442] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    8.045397] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    8.148500] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    8.194490] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    8.384933] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    8.909244] random: cloud-init: uninitialized urandom read (32 bytes read, 44 bits of entropy available)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    9.028478] systemd-udevd[703]: starting version 204
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    9.132458] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    9.177993] intel_rapl: no valid rapl domains found in package 0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    9.226275] intel_rapl: no valid rapl domains found in package 0
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    9.234457] ppdev: user-space parallel port driver
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    9.325885] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    9.374495] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    9.433143] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    9.595763] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    9.840456] random: mktemp: uninitialized urandom read (12 bytes read, 59 bits of entropy available)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    9.914424] random: mktemp: uninitialized urandom read (6 bytes read, 60 bits of entropy available)
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [    9.980896] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [   10.020192] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [   10.415582] init: failsafe main process (1094) killed by TERM signal
Aug  7 20:24:31 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO Running set_multiqueue.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO Set channels for eth0 to 4.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Starting Google Accounts daemon.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Creating a new user account for me.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Created user account me.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Creating a new user account for henrik.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [   11.147965] random: nonblocking pool is initialized
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Created user account henrik.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Creating a new user account for emma.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Created user account emma.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Creating a new user account for igor.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Created user account igor.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Created user account konstantinhaase.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Creating a new user account for aj.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Created user account aj.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Creating a new user account for solarce.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Created user account solarce.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 cron[1456]: (CRON) INFO (pidfile fd = 3)
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 cron[1502]: (CRON) STARTUP (fork ok)
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 cron[1502]: (CRON) INFO (Running @reboot jobs)
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Creating a new user account for asari.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Created user account asari.
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 acpid: starting up with netlink and the input layer
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 acpid: 1 rule loaded
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 acpid: waiting for events: event logging is off
Aug  7 20:24:32 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Creating a new user account for bogdana.
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 haveged: haveged starting up
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Created user account bogdana.
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Creating a new user account for konstantin.
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [   11.676512] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [   11.690339] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Created user account konstantin.
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Creating a new user account for carmen.
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Created user account carmen.
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Creating a new user account for maria.
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Created user account maria.
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [   11.809611] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [   11.813061] Bridge firewalling registered
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-accounts: INFO Removing user packer.
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [   11.823943] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [   11.883223] Initializing XFRM netlink socket
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [   11.890926] Netfilter messages via NETLINK v0.30.
Aug  7 20:24:33 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [   11.893933] ctnetlink v0.93: registering with nfnetlink.
Aug  7 20:24:34 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 20:24:34 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [   12.183575] floppy0: no floppy controllers found
Aug  7 20:24:56 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpdate[1850]: adjust time server 169.254.169.254 offset 0.003478 sec
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpd[1885]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpd[1886]: proto: precision = 0.132 usec
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpd[1886]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpd[1886]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpd[1886]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpd[1886]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpd[1886]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpd[1886]: Listen normally on 3 eth0 10.20.0.40 UDP 123
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpd[1886]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpd[1886]: peers refreshed
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpd[1886]: Listening on routing socket on fd #21 for interface updates
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [   41.849472] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 startup-script: INFO Found startup-script in metadata.
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 startup-script: INFO startup-script: job 1 at Tue Aug  7 23:35:00 2018
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 startup-script: INFO startup-script: Return code 0.
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 startup-script: INFO startup-script: Return code 0.
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 startup-script: INFO Finished running startup scripts.
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ec2: 
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ec2: #############################################################
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ec2: 1024 ac:59:16:af:54:70:d2:bc:d3:71:5a:6b:f6:76:2a:2a  root@travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 (DSA)
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ec2: 256 15:04:ca:f6:bf:40:3b:b3:3e:b2:f8:88:67:98:7b:10  root@travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 (ECDSA)
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ec2: 256 6f:d5:cb:b8:3a:cf:07:da:7a:cf:df:b8:7e:24:24:44  root@travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 (ED25519)
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ec2: 2048 52:67:1c:51:10:e3:8c:7b:e7:00:65:50:33:7f:04:a9  root@travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 (RSA)
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 20:25:03 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ec2: #############################################################
Aug  7 20:25:35 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [   73.463488] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 20:26:41 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [  139.526421] device veth604b720 entered promiscuous mode
Aug  7 20:26:41 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [  139.616128] cgroup: docker-runc (4868) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 20:26:41 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [  139.616131] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 20:26:41 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [  139.694219] eth0: renamed from vethb4a9695
Aug  7 20:26:41 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [  139.730791] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 20:26:41 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [  139.732016] docker0: port 1(veth604b720) entered forwarding state
Aug  7 20:26:41 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [  139.732031] docker0: port 1(veth604b720) entered forwarding state
Aug  7 20:26:41 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [  139.732053] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 20:26:45 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpd[1886]: Listen normally on 5 docker0 fe80::42:b1ff:fe16:af2c UDP 123
Aug  7 20:26:45 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpd[1886]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  7 20:26:45 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpd[1886]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 20:26:45 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpd[1886]: peers refreshed
Aug  7 20:26:45 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 ntpd[1886]: new interface(s) found: waking up resolver
Aug  7 20:26:56 travis-job-c7dc3045-29ff-4fc9-9feb-9ff9260790c4 kernel: [  154.783442] docker0: port 1(veth604b720) entered forwarding state
---
travis_time:end:002bda9e:start=1533673784798702945,finish=1533673784805345331,duration=6642386
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:098edde3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0fc8c929
travis_time:start:0fc8c929
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:1024edc5
$ dmesg | grep -i kill
