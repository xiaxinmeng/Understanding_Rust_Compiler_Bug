plain

[00:03:52] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:52] tidy error: /checkout/src/test/ui/const-eval/issue-53157.rs: missing trailing newline
[00:03:53] some tidy checks failed
[00:03:53] 
[00:03:53] 
[00:03:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:53] 
[00:03:53] 
[00:03:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:53] Build completed unsuccessfully in 0:00:52
[00:03:53] Build completed unsuccessfully in 0:00:52
[00:03:53] make: *** [tidy] Error 1
[00:03:53] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:353015ae
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:151d4e20
$ sudo tail -n 500 /var/log/syslog
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27D0 000014 (v00 Google)
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] kvm-clock: using sched offset of 9789637172 cycles
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] Zone ranges:
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 16:39:29 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000]   Device   empty
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] Movable zone start for each node
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] Early memory node ranges
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] Policy zone: Normal
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] console [ttyS0] enabled
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.307956] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.309215] pid_max: default: 32768 minimum: 301
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.310016] ACPI: Core revision 20150930
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.316500] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.317560] Security Framework initialized
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.318192] Yama: becoming mindful.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.318685] AppArmor: AppArmor disabled by boot time parameter
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.321237] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.330423] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.334697] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.335804] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.337222] Initializing cgroup subsys io
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.337824] Initializing cgroup subsys memory
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.338428] Initializing cgroup subsys devices
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.339112] Initializing cgroup subsys freezer
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.339965] Initializing cgroup subsys net_cls
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.340615] Initializing cgroup subsys perf_event
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.341273] Initializing cgroup subsys net_prio
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.342003] Initializing cgroup subsys hugetlb
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.342816] Initializing cgroup subsys pids
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.343497] CPU: Physical Processor ID: 0
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.344064] CPU: Processor Core ID: 0
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.344585] mce: CPU supports 32 MCE banks
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.345400] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.346183] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.348836] Freeing SMP alternatives memory: 32K
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.357213] ftrace: allocating 32185 entries in 126 pages
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.403430] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.404526] smpboot: Max logical packages: 2
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.405640] x2apic enabled
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.407014] Switched APIC routing to physical x2apic.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.410548] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.519208] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.520675] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.522871] x86: Booting SMP configuration:
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.523494] .... node  #0, CPUs:      #1
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.524247] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.527700]  #2
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.528240] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.531453]  #3
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.531926] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.535277] x86: Booted up 1 node, 4 CPUs
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.535960] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.538373] devtmpfs: initialized
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.542249] evm: security.selinux
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.542801] evm: security.SMACK64
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.543365] evm: security.SMACK64EXEC
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.544007] evm: security.SMACK64TRANSMUTE
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.544571] evm: security.SMACK64MMAP
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.545127] evm: security.ima
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.545546] evm: security.capability
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.546395] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.548331] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.549426] pinctrl core: initialized pinctrl subsystem
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.550347] RTC time: 16:39:19, date: 08/07/18
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.551781] NET: Registered protocol family 16
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.563236] cpuidle: using governor ladder
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.575243] cpuidle: using governor menu
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.576084] PCCT header not found.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.576703] ACPI: bus type PCI registered
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.577323] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.578576] PCI: Using configuration type 1 for base access
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.592199] ACPI: Added _OSI(Module Device)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.592867] ACPI: Added _OSI(Processor Device)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.593497] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.594150] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.597342] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.620410] ACPI: Interpreter enabled
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.621063] ACPI: (supports S0 S3 S4 S5)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.621606] ACPI: Using IOAPIC for interrupt routing
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.622362] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.651232] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.652303] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.653252] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.654153] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.656340] PCI host bridge to bus 0000:00
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.656913] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.657845] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.658770] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.659770] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.660769] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.661796] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.662204] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.674234] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.687683] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.689147] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.694123] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.697765] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.710263] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.715095] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.718902] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.730447] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.732834] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.735031] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.737030] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.738858] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.758479] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.759590] vgaarb: loaded
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.760225] SCSI subsystem initialized
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.760930] libata version 3.00 loaded.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.760959] ACPI: bus type USB registered
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.761713] usbcore: registered new interface driver usbfs
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.762515] usbcore: registered new interface driver hub
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.763462] usbcore: registered new device driver usb
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.764605] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.765583] dmi: Firmware registration failed.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.766376] PCI: Using ACPI for IRQ routing
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.767000] PCI: pci_cache_line_size set to 64 bytes
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.767100] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.767102] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.767214] NetLabel: Initializing
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.767939] NetLabel:  domain hash size = 128
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.768612] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.769342] NetLabel:  unlabeled traffic allowed by default
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.770301] amd_nb: Cannot enumerate AMD northbridges
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.771070] clocksource: Switched to clocksource kvm-clock
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.778444] pnp: PnP ACPI init
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.779239] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.779306] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.779349] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.779399] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.779440] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.779488] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.779530] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.779696] pnp: PnP ACPI: found 7 devices
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.787332] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.789048] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.789051] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.789053] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.789054] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.789087] NET: Registered protocol family 2
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.789883] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.791088] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.792100] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.793020] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.793887] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.795549] NET: Registered protocol family 1
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.796273] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.797209] PCI: CLS 0 bytes, default 64
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    0.797262] Unpacking initramfs...
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.757174] Freeing initrd memory: 21432K
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.757897] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.758889] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.760688] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.761923] hw unit of domain pp0-core 2^-0 Joules
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.762707] hw unit of domain package 2^-0 Joules
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.763371] hw unit of domain dram 2^-0 Joules
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.764077] Scanning for low memory corruption every 60 seconds
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.765413] audit: initializing netlink subsys (disabled)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.766453] audit: type=2000 audit(1533659961.946:1): initialized
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.767733] Initialise system trusted keyring
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.768587] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.769610] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.771784] zbud: loaded
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.772432] VFS: Disk quotas dquot_6.6.0
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.773180] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.774449] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.775728] fuse init (API version 7.23)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.776569] Key type big_key registered
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.777190] Allocating IMA MOK and blacklist keyrings.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.778824] Key type asymmetric registered
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.779555] Asymmetric key parser 'x509' registered
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.780413] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.781598] io scheduler noop registered
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.782188] io scheduler deadline registered (default)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.782955] io scheduler cfq registered
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.783611] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.784398] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.785335] intel_idle: does not run on family 6 model 45
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.785420] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.786519] ACPI: Power Button [PWRF]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.787164] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.788213] ACPI: Sleep Button [SLPF]
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.789385] GHES: HEST is not enabled!
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.791754] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.792704] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.796485] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.797470] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.801406] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.823723] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.846529] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.869299] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.891558] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.894579] Linux agpgart interface v0.103
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.897604] loop: module loaded
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.898607] libphy: Fixed MDIO Bus: probed
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.899708] tun: Universal TUN/TAP device driver, 1.6
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.900445] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.927637] PPP generic driver version 2.4.2
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.928455] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.929641] ehci-pci: EHCI PCI platform driver
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.930385] ehci-platform: EHCI generic platform driver
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.931406] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.932489] ohci-pci: OHCI PCI platform driver
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.932999] ohci-platform: OHCI generic platform driver
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.934075] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.935437] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.937554] i8042: Warning: Keylock active
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.939311] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.940014] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.940896] mousedev: PS/2 mouse device common for all mice
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.942345] rtc_cmos 00:00: RTC can wake from S4
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.943402] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.945011] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.946378] i2c /dev entries driver
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.947236] device-mapper: uevent: version 1.0.3
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.948130] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.949378] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.951235] NET: Registered protocol family 10
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.952518] NET: Registered protocol family 17
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.953159] Key type dns_resolver registered
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.954188] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.955264] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.956309] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.957329] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.958863] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.960744] registered taskstats version 1
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.961519] Loading compiled-in X.509 certificates
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.963323] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.965583] zswap: loaded using pool lzo/zbud
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.968132] Key type trusted registered
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.972860] Key type encrypted registered
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.974155] ima: No TPM chip found, activating TPM-bypass!
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.975935] evm: HMAC attrs: 0x1
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.977591]   Magic number: 14:367:691
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.978406] acpi LNXCPU:32: hash matches
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.979552] rtc_cmos 00:00: setting system clock to 2018-08-07 16:39:22 UTC (1533659962)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.981256] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.982604] EDD information not available.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.983367] PM: Hibernation image not present or could not be loaded.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.984621] Freeing unused kernel memory: 1496K
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.985265] Write protecting the kernel read-only data: 14336k
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.986999] Freeing unused kernel memory: 1956K
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    2.987905] Freeing unused kernel memory: 92K
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.000173] systemd-udevd[118]: starting version 204
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.050177] scsi host0: Virtio SCSI HBA
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.056367] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.057987] AVX version of gcm_enc/dec engaged.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.058819] AES CTR mode by8 optimization enabled
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.090818] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.090836] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.092835] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.094196] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.094947] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.095120] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.099929]  sda: sda1
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.101468] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.139402] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.763211] tsc: Refined TSC clocksource calibration: 2600.001 MHz
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.764409] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3ce1c4c, max_idle_ns: 440795206275 ns
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    3.972166] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    6.047201] floppy0: no floppy controllers found
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.199121] raid6: sse2x1   gen()  9076 MB/s
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.271096] raid6: sse2x1   xor()  6737 MB/s
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.343109] raid6: sse2x2   gen() 11250 MB/s
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.415095] raid6: sse2x2   xor()  7408 MB/s
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.487096] raid6: sse2x4   gen() 13068 MB/s
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.559113] raid6: sse2x4   xor()  8973 MB/s
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.563770] raid6: using algorithm sse2x4 gen() 13068 MB/s
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.569382] raid6: .... xor() 8973 MB/s, rmw enabled
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.574574] raid6: using ssse3x2 recovery algorithm
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.580931] xor: automatically using best checksumming function:
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.623088]    avx       : 27813.000 MB/sec
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.640095] Btrfs loaded
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.680778] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.687794] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.747719] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.758415] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.763551] EXT4-fs (sda1): recovery complete
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.772117] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    7.979883] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    8.091711] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    8.152287] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    8.342203] random: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    8.857947] random: cloud-init: uninitialized urandom read (32 bytes read, 46 bits of entropy available)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    8.982723] systemd-udevd[702]: starting version 204
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    9.079529] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    9.150220] intel_rapl: no valid rapl domains found in package 0
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    9.217117] ppdev: user-space parallel port driver
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    9.297719] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    9.341643] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    9.399472] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    9.555917] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    9.814952] random: mktemp: uninitialized urandom read (12 bytes read, 62 bits of entropy available)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    9.887478] random: mktemp: uninitialized urandom read (6 bytes read, 62 bits of entropy available)
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    9.955376] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [    9.993069] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [   10.412230] init: failsafe main process (1093) killed by TERM signal
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO Running set_multiqueue.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO Set channels for eth0 to 4.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Starting Google Accounts daemon.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Creating a new user account for me.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Created user account me.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Creating a new user account for henrik.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Created user account henrik.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Creating a new user account for emma.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Created user account emma.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Creating a new user account for igor.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Created user account igor.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Created user account konstantinhaase.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Creating a new user account for aj.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Created user account aj.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Creating a new user account for solarce.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Created user account solarce.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Creating a new user account for asari.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Created user account asari.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Creating a new user account for bogdana.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Created user account bogdana.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Creating a new user account for konstantin.
Aug  7 16:39:30 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Created user account konstantin.
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Creating a new user account for carmen.
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Created user account carmen.
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Creating a new user account for maria.
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Created user account maria.
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-accounts: INFO Removing user packer.
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [   11.742498] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [   11.747089] Bridge firewalling registered
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [   11.758565] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [   11.786916] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [   11.841286] Initializing XFRM netlink socket
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [   11.847584] Netfilter messages via NETLINK v0.30.
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [   11.849918] ctnetlink v0.93: registering with nfnetlink.
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [   11.995516] random: nonblocking pool is initialized
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [   12.175184] floppy0: no floppy controllers found
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 16:39:31 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 16:39:36 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 cron[1705]: (CRON) INFO (pidfile fd = 3)
Aug  7 16:39:36 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 cron[1741]: (CRON) STARTUP (fork ok)
Aug  7 16:39:36 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 16:39:36 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 16:39:36 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 cron[1741]: (CRON) INFO (Running @reboot jobs)
Aug  7 16:39:36 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 acpid: starting up with netlink and the input layer
Aug  7 16:39:36 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 acpid: 1 rule loaded
Aug  7 16:39:36 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 acpid: waiting for events: event logging is off
Aug  7 16:39:36 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 haveged: haveged starting up
Aug  7 16:39:36 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [   16.777229] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ntpd[1850]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ntpd[1851]: proto: precision = 0.170 usec
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ntpd[1851]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ntpd[1851]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
---
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 startup-script: INFO startup-script: job 1 at Tue Aug  7 19:49:00 2018
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 startup-script: INFO startup-script: Return code 0.
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 startup-script: INFO Finished running startup scripts.
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ec2: 
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ec2: #############################################################
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ec2: 1024 04:f1:c8:b0:91:8b:31:19:3a:77:d2:84:2e:ab:07:81  root@travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 (DSA)
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ec2: 256 e4:9c:0d:36:85:5a:20:8d:bb:86:69:9f:c2:08:97:90  root@travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 (ECDSA)
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ec2: 256 40:b6:08:77:96:d3:85:01:20:e1:64:b4:81:f4:62:74  root@travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 (ED25519)
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ec2: 2048 cc:26:ec:2c:d0:9c:21:1c:6c:65:e7:a2:c6:9b:85:14  root@travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 (RSA)
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 16:39:41 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ec2: #############################################################
Aug  7 16:39:46 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ntpdate[2922]: the NTP socket is in use, exiting
Aug  7 16:40:23 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [   64.271765] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 16:41:23 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  124.361930] device veth1c7e01c entered promiscuous mode
Aug  7 16:41:23 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  124.362013] docker0: port 1(veth1c7e01c) entered forwarding state
Aug  7 16:41:23 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  124.362022] docker0: port 1(veth1c7e01c) entered forwarding state
Aug  7 16:41:23 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  124.362389] docker0: port 1(veth1c7e01c) entered disabled state
Aug  7 16:41:23 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  124.444398] cgroup: docker-runc (4843) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 16:41:23 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  124.444402] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 16:41:23 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  124.512017] eth0: renamed from veth4a18a44
Aug  7 16:41:23 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  124.553088] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 16:41:23 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  124.554220] docker0: port 1(veth1c7e01c) entered forwarding state
Aug  7 16:41:23 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  124.554239] docker0: port 1(veth1c7e01c) entered forwarding state
Aug  7 16:41:23 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  124.554266] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 16:41:27 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ntpd[1851]: Listen normally on 5 docker0 fe80::42:5eff:fe14:36f0 UDP 123
Aug  7 16:41:27 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ntpd[1851]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  7 16:41:27 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ntpd[1851]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 16:41:27 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ntpd[1851]: peers refreshed
Aug  7 16:41:27 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 ntpd[1851]: new interface(s) found: waking up resolver
Aug  7 16:41:38 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  139.614679] docker0: port 1(veth1c7e01c) entered forwarding state
Aug  7 16:44:18 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  299.420885] veth4a18a44: renamed from eth0
Aug  7 16:44:18 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  299.431708] docker0: port 1(veth1c7e01c) entered disabled state
Aug  7 16:44:18 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  299.475217] docker0: port 1(veth1c7e01c) entered disabled state
Aug  7 16:44:18 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  299.476790] device veth1c7e01c left promiscuous mode
Aug  7 16:44:18 travis-job-93a80955-61d6-4c1a-a23d-8ab9be430f40 kernel: [  299.476793] docker0: port 1(veth1c7e01c) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:01ddd410
---
travis_time:end:2fa03dc0:start=1533660259242342267,finish=1533660259247512793,duration=5170526
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:35e76cb6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:050473e2
travis_time:start:050473e2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0112cfd1
$ dmesg | grep -i kill
