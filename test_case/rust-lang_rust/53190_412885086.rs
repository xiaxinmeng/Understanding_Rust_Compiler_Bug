plain

[00:04:04] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:05] tidy error: /checkout/src/test/run-make/git_clone_sha1.sh: incorrect license
[00:04:06] some tidy checks failed
[00:04:06] 
[00:04:06] 
[00:04:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:06] 
[00:04:06] 
[00:04:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:06] Build completed unsuccessfully in 0:00:52
[00:04:06] Build completed unsuccessfully in 0:00:52
[00:04:06] make: *** [tidy] Error 1
[00:04:06] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05bc52a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:01eb7f84
$ sudo tail -n 500 /var/log/syslog
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] kvm-clock: using sched offset of 1554812672 cycles
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] Zone ranges:
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000]   Device   empty
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] Movable zone start for each node
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] Early memory node ranges
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] Policy zone: Normal
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] console [ttyS0] enabled
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.472452] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.475756] pid_max: default: 32768 minimum: 301
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.477166] ACPI: Core revision 20150930
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.485138] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.487746] Security Framework initialized
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.489176] Yama: becoming mindful.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.490281] AppArmor: AppArmor disabled by boot time parameter
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.494090] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.505650] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.512278] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.514720] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.517514] Initializing cgroup subsys io
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.518817] Initializing cgroup subsys memory
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.520070] Initializing cgroup subsys devices
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.521454] Initializing cgroup subsys freezer
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.523106] Initializing cgroup subsys net_cls
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.524452] Initializing cgroup subsys perf_event
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.525846] Initializing cgroup subsys net_prio
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.527500] Initializing cgroup subsys hugetlb
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.528704] Initializing cgroup subsys pids
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.530497] CPU: Physical Processor ID: 0
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.531741] CPU: Processor Core ID: 0
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.534372] mce: CPU supports 32 MCE banks
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.535960] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.537681] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.542046] Freeing SMP alternatives memory: 32K
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.553636] ftrace: allocating 32185 entries in 126 pages
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.612512] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.614777] smpboot: Max logical packages: 2
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.616621] x2apic enabled
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.618667] Switched APIC routing to physical x2apic.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.623120] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.730612] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.733625] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.738085] x86: Booting SMP configuration:
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.739488] .... node  #0, CPUs:      #1
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.740970] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.746819]  #2
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.747797] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.753369]  #3
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.754201] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.761164] x86: Booted up 1 node, 4 CPUs
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.762759] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.767383] devtmpfs: initialized
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.772815] evm: security.selinux
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.774475] evm: security.SMACK64
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.775950] evm: security.SMACK64EXEC
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.777362] evm: security.SMACK64TRANSMUTE
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.778685] evm: security.SMACK64MMAP
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.779713] evm: security.ima
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.780964] evm: security.capability
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.782883] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.786051] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.788278] pinctrl core: initialized pinctrl subsystem
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.790170] RTC time: 13:55:38, date: 08/14/18
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.792659] NET: Registered protocol family 16
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.802690] cpuidle: using governor ladder
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.814697] cpuidle: using governor menu
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.816052] PCCT header not found.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.817754] ACPI: bus type PCI registered
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.819079] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.821458] PCI: Using configuration type 1 for base access
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.836688] ACPI: Added _OSI(Module Device)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.838368] ACPI: Added _OSI(Processor Device)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.839648] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.841480] ACPI: Added _OSI(Processor Aggregator Device)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.846050] ACPI: Executed 2 blocks of module-level executable AML code
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.872402] ACPI: Interpreter enabled
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.875273] ACPI: (supports S0 S3 S4 S5)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.877624] ACPI: Using IOAPIC for interrupt routing
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.879495] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.912941] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.914899] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.917607] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.920226] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.924758] PCI host bridge to bus 0000:00
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.926007] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.928181] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.930160] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.932433] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.934968] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.936948] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.937452] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.966827] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.996615] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    0.999866] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.012384] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.021950] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.048566] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.059204] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.068007] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.092787] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.097954] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.103801] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.108985] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.113385] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.136245] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.138830] vgaarb: loaded
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.140037] SCSI subsystem initialized
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.141839] libata version 3.00 loaded.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.141864] ACPI: bus type USB registered
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.143207] usbcore: registered new interface driver usbfs
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.145317] usbcore: registered new interface driver hub
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.147087] usbcore: registered new device driver usb
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.148793] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.151191] dmi: Firmware registration failed.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.152892] PCI: Using ACPI for IRQ routing
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.154314] PCI: pci_cache_line_size set to 64 bytes
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.154422] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.154425] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.154549] NetLabel: Initializing
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.155702] NetLabel:  domain hash size = 128
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.156943] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.158609] NetLabel:  unlabeled traffic allowed by default
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.160696] amd_nb: Cannot enumerate AMD northbridges
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.162249] clocksource: Switched to clocksource kvm-clock
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.171181] pnp: PnP ACPI init
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.172348] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.172416] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.172460] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.172507] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.172546] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.172584] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.172642] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.172802] pnp: PnP ACPI: found 7 devices
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.182255] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.185239] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.185241] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.185243] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.185244] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.185310] NET: Registered protocol family 2
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.186833] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.189847] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.192295] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.194943] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.196855] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.198961] NET: Registered protocol family 1
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.200287] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.202655] PCI: CLS 0 bytes, default 64
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    1.203503] Unpacking initramfs...
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.359118] Freeing initrd memory: 21432K
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.360529] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.362823] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.366419] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.369209] hw unit of domain pp0-core 2^-0 Joules
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.370754] hw unit of domain package 2^-0 Joules
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.372531] hw unit of domain dram 2^-0 Joules
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.374067] Scanning for low memory corruption every 60 seconds
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.377245] audit: initializing netlink subsys (disabled)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.379110] audit: type=2000 audit(1534254940.631:1): initialized
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.381394] Initialise system trusted keyring
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.383427] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.385932] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.389115] zbud: loaded
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.390481] VFS: Disk quotas dquot_6.6.0
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.391887] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.394197] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.396719] fuse init (API version 7.23)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.398553] Key type big_key registered
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.399993] Allocating IMA MOK and blacklist keyrings.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.404299] Key type asymmetric registered
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.406227] Asymmetric key parser 'x509' registered
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.408868] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.412439] io scheduler noop registered
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.413656] io scheduler deadline registered (default)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.415190] io scheduler cfq registered
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.416483] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.418129] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.419991] intel_idle: does not run on family 6 model 62
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.420107] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.422304] ACPI: Power Button [PWRF]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.423494] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.425550] ACPI: Sleep Button [SLPF]
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.427253] GHES: HEST is not enabled!
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.430855] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.432753] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.441392] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.443492] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.451769] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.475717] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.500053] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.524431] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.549025] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.554300] Linux agpgart interface v0.103
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.558598] loop: module loaded
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.559856] libphy: Fixed MDIO Bus: probed
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.561025] tun: Universal TUN/TAP device driver, 1.6
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.562674] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.611880] PPP generic driver version 2.4.2
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.613603] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.615679] ehci-pci: EHCI PCI platform driver
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.617095] ehci-platform: EHCI generic platform driver
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.618772] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.621002] ohci-pci: OHCI PCI platform driver
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.622452] ohci-platform: OHCI generic platform driver
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.624030] uhci_hcd: USB Universal Host Controller Interface driver
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.626153] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.629021] i8042: Warning: Keylock active
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.631377] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.632994] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.635118] mousedev: PS/2 mouse device common for all mice
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.637475] rtc_cmos 00:00: RTC can wake from S4
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.639635] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.641961] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.644100] i2c /dev entries driver
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.645230] device-mapper: uevent: version 1.0.3
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.646836] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.649853] ledtrig-cpu: registered to indicate activity on CPUs
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.652941] NET: Registered protocol family 10
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.654831] NET: Registered protocol family 17
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.656465] Key type dns_resolver registered
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.658144] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.660010] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.661981] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.664309] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.666358] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.669633] registered taskstats version 1
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.670832] Loading compiled-in X.509 certificates
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.673086] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.676221] zswap: loaded using pool lzo/zbud
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.679992] Key type trusted registered
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.685760] Key type encrypted registered
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.687360] ima: No TPM chip found, activating TPM-bypass!
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.689192] evm: HMAC attrs: 0x1
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.690980]   Magic number: 14:333:938
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.692561] rtc_cmos 00:00: setting system clock to 2018-08-14 13:55:41 UTC (1534254941)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.695437] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.697383] EDD information not available.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.698965] PM: Hibernation image not present or could not be loaded.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.700462] Freeing unused kernel memory: 1496K
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.701847] Write protecting the kernel read-only data: 14336k
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.704487] Freeing unused kernel memory: 1956K
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.706219] Freeing unused kernel memory: 92K
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.722954] systemd-udevd[120]: starting version 204
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.787655] scsi host0: Virtio SCSI HBA
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.795072] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.800540] AVX version of gcm_enc/dec engaged.
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.801965] AES CTR mode by8 optimization enabled
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.834628] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.845629] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.845651] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.850716] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.853299] sd 0:0:1:0: [sda] Write Protect is off
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.854786] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.854941] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.860404]  sda: sda1
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    3.862706] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    4.370352] tsc: Refined TSC clocksource calibration: 2499.784 MHz
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    4.372956] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24086dc9c08, max_idle_ns: 440795236697 ns
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    4.672220] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    6.774450] floppy0: no floppy controllers found
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    7.950286] raid6: sse2x1   gen()  9203 MB/s
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.018283] raid6: sse2x1   xor()  6758 MB/s
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.086288] raid6: sse2x2   gen() 11561 MB/s
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.154276] raid6: sse2x2   xor()  7160 MB/s
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.222282] raid6: sse2x4   gen() 12412 MB/s
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.290273] raid6: sse2x4   xor()  8855 MB/s
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.290949] raid6: using algorithm sse2x4 gen() 12412 MB/s
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.291897] raid6: .... xor() 8855 MB/s, rmw enabled
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.292623] raid6: using ssse3x2 recovery algorithm
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.294852] xor: automatically using best checksumming function:
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.334281]    avx       : 21936.000 MB/sec
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.348824] Btrfs loaded
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.397503] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.398676] EXT4-fs (sda1): write access will be enabled during recovery
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.452389] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.462868] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.463814] EXT4-fs (sda1): recovery complete
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.468863] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.662009] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.768135] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.815491] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    8.987630] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    9.480669] random: cloud-init: uninitialized urandom read (32 bytes read, 44 bits of entropy available)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    9.602170] systemd-udevd[704]: starting version 204
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    9.704936] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    9.809700] ppdev: user-space parallel port driver
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    9.912179] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [    9.955748] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   10.022842] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   10.181213] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   10.426101] random: mktemp: uninitialized urandom read (12 bytes read, 59 bits of entropy available)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   10.494506] random: mktemp: uninitialized urandom read (6 bytes read, 60 bits of entropy available)
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   10.568998] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   10.603713] EXT4-fs (sda1): resized filesystem to 7864064
Aug 14 13:55:48 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   11.049535] init: failsafe main process (1095) killed by TERM signal
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO Running set_multiqueue.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO Set channels for eth0 to 4.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Starting Google Accounts daemon.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   11.758939] random: nonblocking pool is initialized
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Creating a new user account for me.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Created user account me.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Creating a new user account for henrik.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Created user account henrik.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Creating a new user account for emma.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Created user account emma.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Creating a new user account for igor.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Created user account igor.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 cron[1432]: (CRON) INFO (pidfile fd = 3)
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 cron[1473]: (CRON) STARTUP (fork ok)
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 cron[1473]: (CRON) INFO (Running @reboot jobs)
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Created user account konstantinhaase.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 acpid: starting up with netlink and the input layer
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 acpid: 1 rule loaded
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 acpid: waiting for events: event logging is off
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Creating a new user account for aj.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Created user account aj.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Creating a new user account for solarce.
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 haveged: haveged starting up
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 13:55:49 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   12.222488] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Created user account solarce.
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   12.232361] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Creating a new user account for asari.
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Created user account asari.
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Creating a new user account for bogdana.
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Created user account bogdana.
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-clock-skew: INFO Synced system time with hardware clock.
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Creating a new user account for konstantin.
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Created user account konstantin.
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   12.412640] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   12.416683] Bridge firewalling registered
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Creating a new user account for carmen.
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   12.424752] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Created user account carmen.
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   12.487433] Initializing XFRM netlink socket
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Creating a new user account for maria.
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   12.496050] Netfilter messages via NETLINK v0.30.
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   12.498547] ctnetlink v0.93: registering with nfnetlink.
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Created user account maria.
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 google-accounts: INFO Removing user packer.
Aug 14 13:55:50 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   12.766443] floppy0: no floppy controllers found
Aug 14 13:56:13 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpdate[1843]: adjust time server 169.254.169.254 offset 0.001811 sec
Aug 14 13:56:19 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpd[1872]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 14 13:56:19 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpd[1873]: proto: precision = 0.128 usec
Aug 14 13:56:19 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpd[1873]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 13:56:19 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpd[1873]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 13:56:19 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpd[1873]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 13:56:19 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpd[1873]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 14 13:56:19 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpd[1873]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 14 13:56:19 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpd[1873]: Listen normally on 3 eth0 10.20.1.89 UDP 123
Aug 14 13:56:19 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpd[1873]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 14 13:56:19 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpd[1873]: peers refreshed
Aug 14 13:56:19 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpd[1873]: Listening on routing socket on fd #21 for interface updates
Aug 14 13:56:20 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [   42.419096] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 13:56:20 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 startup-script: INFO Found startup-script in metadata.
Aug 14 13:56:20 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 14 13:56:20 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 startup-script: INFO startup-script: job 1 at Tue Aug 14 17:06:00 2018
Aug 14 13:56:20 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 startup-script: INFO startup-script: Return code 0.
Aug 14 13:56:20 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 startup-script: INFO startup-script: Return code 0.
Aug 14 13:56:20 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 startup-script: INFO Finished running startup scripts.
Aug 14 13:56:20 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ec2: 
Aug 14 13:56:20 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ec2: #############################################################
Aug 14 13:56:20 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 14 13:56:20 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ec2: 1024 75:a5:cc:1c:b1:e3:02:54:04:25:27:6c:27:99:92:ce  root@travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 (DSA)
Aug 14 13:56:20 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ec2: 256 7d:6d:f9:66:02:9b:13:6d:fa:90:42:41:a1:0c:a5:e3  root@travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 (ECDSA)
Aug 14 13:56:20 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ec2: 256 e2:6e:50:5b:bd:55:ac:b5:cd:15:64:4f:88:8f:48:de  root@travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 (ED25519)
Aug 14 13:56:20 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ec2: 2048 12:e4:e2:c6:da:d5:71:c4:f9:8f:c9:6b:74:db:b7:6d  root@travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 (RSA)
Aug 14 13:56:20 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 14 13:56:20 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ec2: #############################################################
Aug 14 13:57:45 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [  128.222162] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 14 13:58:53 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [  195.900689] device veth1d9a39c entered promiscuous mode
Aug 14 13:58:53 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [  195.900751] docker0: port 1(veth1d9a39c) entered forwarding state
Aug 14 13:58:53 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [  195.900758] docker0: port 1(veth1d9a39c) entered forwarding state
Aug 14 13:58:53 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [  195.901167] docker0: port 1(veth1d9a39c) entered disabled state
Aug 14 13:58:53 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [  195.980071] cgroup: docker-runc (4864) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 13:58:53 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [  195.980074] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 13:58:53 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [  196.047743] eth0: renamed from veth3fe12a0
Aug 14 13:58:53 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [  196.082036] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 13:58:53 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [  196.082975] docker0: port 1(veth1d9a39c) entered forwarding state
Aug 14 13:58:53 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [  196.082991] docker0: port 1(veth1d9a39c) entered forwarding state
Aug 14 13:58:53 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [  196.083016] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 14 13:58:56 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpd[1873]: Listen normally on 5 docker0 fe80::42:a9ff:fedb:8a0d UDP 123
Aug 14 13:58:56 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpd[1873]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 14 13:58:56 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpd[1873]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 14 13:58:56 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpd[1873]: peers refreshed
Aug 14 13:58:56 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 ntpd[1873]: new interface(s) found: waking up resolver
Aug 14 13:59:08 travis-job-1fe59146-3b13-41a2-bf0e-ee74152e4a77 kernel: [  211.107502] docker0: port 1(veth1d9a39c) entered forwarding state
---
travis_time:end:0ae2635e:start=1534255313852048124,finish=1534255313858220699,duration=6172575
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01a842ec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05d690f3
travis_time:start:05d690f3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:014d18e4
$ dmesg | grep -i kill
