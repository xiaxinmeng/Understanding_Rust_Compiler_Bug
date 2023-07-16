plain

[00:04:03] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:03] tidy error: /checkout/src/libcore/macros.rs:353: trailing whitespace
[00:04:03] tidy error: /checkout/src/libcore/macros.rs: missing trailing newline
[00:04:04] some tidy checks failed
[00:04:04] 
[00:04:04] 
[00:04:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:04] 
[00:04:04] 
[00:04:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:04] Build completed unsuccessfully in 0:00:50
[00:04:04] Build completed unsuccessfully in 0:00:50
[00:04:04] Makefile:79: recipe for target 'tidy' failed
[00:04:04] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:201a0834
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:00fdff1b
$ sudo tail -n 500 /var/log/syslog
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] kvm-clock: using sched offset of 1732469395 cycles
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] Zone ranges:
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000]   Device   empty
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] Movable zone start for each node
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] Early memory node ranges
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] Policy zone: Normal
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] Hierarchical RCU implementation.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] console [ttyS0] enabled
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.437614] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.440504] pid_max: default: 32768 minimum: 301
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.442012] ACPI: Core revision 20150930
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.449862] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.452024] Security Framework initialized
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.453715] Yama: becoming mindful.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.454774] AppArmor: AppArmor disabled by boot time parameter
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.458443] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.470303] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.476665] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.479360] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.481897] Initializing cgroup subsys io
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.483301] Initializing cgroup subsys memory
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.484780] Initializing cgroup subsys devices
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.486051] Initializing cgroup subsys freezer
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.488037] Initializing cgroup subsys net_cls
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.489447] Initializing cgroup subsys perf_event
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.490884] Initializing cgroup subsys net_prio
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.492179] Initializing cgroup subsys hugetlb
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.493722] Initializing cgroup subsys pids
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.495066] CPU: Physical Processor ID: 0
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.496527] CPU: Processor Core ID: 0
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.498944] mce: CPU supports 32 MCE banks
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.500441] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.502097] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.506659] Freeing SMP alternatives memory: 32K
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.518119] ftrace: allocating 32185 entries in 126 pages
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.578509] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.581085] smpboot: Max logical packages: 2
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.583033] x2apic enabled
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.585376] Switched APIC routing to physical x2apic.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.590473] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.699817] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.703187] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.707959] x86: Booting SMP configuration:
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.709293] .... node  #0, CPUs:      #1
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.710795] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.716246]  #2
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.717006] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.722638]  #3
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.723783] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.729489] x86: Booted up 1 node, 4 CPUs
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.730662] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.734892] devtmpfs: initialized
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.740065] evm: security.selinux
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.741190] evm: security.SMACK64
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.742160] evm: security.SMACK64EXEC
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.743313] evm: security.SMACK64TRANSMUTE
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.744845] evm: security.SMACK64MMAP
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.746489] evm: security.ima
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.747439] evm: security.capability
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.749045] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.752151] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.754659] pinctrl core: initialized pinctrl subsystem
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.756428] RTC time:  1:00:45, date: 08/13/18
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.758983] NET: Registered protocol family 16
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.771876] cpuidle: using governor ladder
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.783888] cpuidle: using governor menu
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.785571] PCCT header not found.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.786892] ACPI: bus type PCI registered
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.788151] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.790159] PCI: Using configuration type 1 for base access
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.804928] ACPI: Added _OSI(Module Device)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.806392] ACPI: Added _OSI(Processor Device)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.807989] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.809454] ACPI: Added _OSI(Processor Aggregator Device)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.814049] ACPI: Executed 2 blocks of module-level executable AML code
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.840532] ACPI: Interpreter enabled
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.842058] ACPI: (supports S0 S3 S4 S5)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.843226] ACPI: Using IOAPIC for interrupt routing
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.844779] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.875956] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.877975] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.880000] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.882000] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.886407] PCI host bridge to bus 0000:00
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.887647] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.889547] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.891888] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.894181] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.896556] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.898295] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.898758] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.923967] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.948014] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.951056] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.961424] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.968330] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.987599] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    0.995854] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.002635] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.022705] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.027108] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.031394] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.035441] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.039309] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.061822] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.063712] vgaarb: loaded
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.065135] SCSI subsystem initialized
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.066466] libata version 3.00 loaded.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.066486] ACPI: bus type USB registered
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.067734] usbcore: registered new interface driver usbfs
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.069808] usbcore: registered new interface driver hub
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.071727] usbcore: registered new device driver usb
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.073440] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.075436] dmi: Firmware registration failed.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.076997] PCI: Using ACPI for IRQ routing
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.078242] PCI: pci_cache_line_size set to 64 bytes
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.078380] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.078383] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.078572] NetLabel: Initializing
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.079819] NetLabel:  domain hash size = 128
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.081213] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.082677] NetLabel:  unlabeled traffic allowed by default
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.084490] amd_nb: Cannot enumerate AMD northbridges
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.086617] clocksource: Switched to clocksource kvm-clock
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.095697] pnp: PnP ACPI init
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.097126] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.097219] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.097283] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.097332] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.097370] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.097409] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.097447] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.097604] pnp: PnP ACPI: found 7 devices
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.106172] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.109220] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.109222] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.109224] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.109226] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.109264] NET: Registered protocol family 2
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.110860] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.113451] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.115973] TCP: Hash tables configured (established 131072 bind 65536)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.118221] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.120170] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.123056] NET: Registered protocol family 1
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.124457] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.126720] PCI: CLS 0 bytes, default 64
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    1.126780] Unpacking initramfs...
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.317201] Freeing initrd memory: 21432K
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.317993] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.318901] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.320601] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.321971] hw unit of domain pp0-core 2^-0 Joules
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.322806] hw unit of domain package 2^-0 Joules
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.323469] hw unit of domain dram 2^-0 Joules
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.324185] Scanning for low memory corruption every 60 seconds
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.325693] audit: initializing netlink subsys (disabled)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.326543] audit: type=2000 audit(1534122047.480:1): initialized
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.327799] Initialise system trusted keyring
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.328727] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.329798] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.331962] zbud: loaded
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.332589] VFS: Disk quotas dquot_6.6.0
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.333261] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.334645] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.335861] fuse init (API version 7.23)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.336617] Key type big_key registered
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.337456] Allocating IMA MOK and blacklist keyrings.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.339486] Key type asymmetric registered
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.340106] Asymmetric key parser 'x509' registered
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.340854] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.342205] io scheduler noop registered
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.342826] io scheduler deadline registered (default)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.343583] io scheduler cfq registered
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.344328] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.345123] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.346047] intel_idle: does not run on family 6 model 62
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.346151] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.347180] ACPI: Power Button [PWRF]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.347748] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.348873] ACPI: Sleep Button [SLPF]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.349768] GHES: HEST is not enabled!
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.352616] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.353744] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.358965] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.359980] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.365023] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.387436] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.411246] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.434511] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.457815] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.460781] Linux agpgart interface v0.103
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.463989] loop: module loaded
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.465297] libphy: Fixed MDIO Bus: probed
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.466580] tun: Universal TUN/TAP device driver, 1.6
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.468160] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.499420] PPP generic driver version 2.4.2
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.500819] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.502698] ehci-pci: EHCI PCI platform driver
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.503762] ehci-platform: EHCI generic platform driver
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.505161] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.506803] ohci-pci: OHCI PCI platform driver
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.508219] ohci-platform: OHCI generic platform driver
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.509867] uhci_hcd: USB Universal Host Controller Interface driver
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.512077] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.515275] i8042: Warning: Keylock active
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.517451] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.518958] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.520718] mousedev: PS/2 mouse device common for all mice
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.522486] rtc_cmos 00:00: RTC can wake from S4
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.524112] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.526175] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.527676] i2c /dev entries driver
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.528796] device-mapper: uevent: version 1.0.3
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.530270] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.532665] ledtrig-cpu: registered to indicate activity on CPUs
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.534917] NET: Registered protocol family 10
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.536097] NET: Registered protocol family 17
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.536958] Key type dns_resolver registered
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.538055] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.539787] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.541246] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.542493] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.543987] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.546392] registered taskstats version 1
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.547244] Loading compiled-in X.509 certificates
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.548766] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.550744] zswap: loaded using pool lzo/zbud
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.553423] Key type trusted registered
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.557625] Key type encrypted registered
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.558585] ima: No TPM chip found, activating TPM-bypass!
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.559764] evm: HMAC attrs: 0x1
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.560841]   Magic number: 14:53:3
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.561851] rtc_cmos 00:00: setting system clock to 2018-08-13 01:00:48 UTC (1534122048)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.563738] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.564976] EDD information not available.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.565856] PM: Hibernation image not present or could not be loaded.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.567292] Freeing unused kernel memory: 1496K
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.568089] Write protecting the kernel read-only data: 14336k
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.569805] Freeing unused kernel memory: 1956K
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.570956] Freeing unused kernel memory: 92K
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.584384] systemd-udevd[119]: starting version 204
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.632845] scsi host0: Virtio SCSI HBA
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.637907] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.639831] AVX version of gcm_enc/dec engaged.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.640541] AES CTR mode by8 optimization enabled
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.680521] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.680575] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.682815] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.683803] sd 0:0:1:0: [sda] Write Protect is off
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.684542] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.684618] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.687413]  sda: sda1
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.688735] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    3.718965] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    4.322808] tsc: Refined TSC clocksource calibration: 2499.793 MHz
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    4.323892] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24087658c32, max_idle_ns: 440795284564 ns
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    4.551851] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    6.642803] floppy0: no floppy controllers found
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    7.826711] raid6: sse2x1   gen()  8666 MB/s
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    7.894710] raid6: sse2x1   xor()  6640 MB/s
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    7.962672] raid6: sse2x2   gen() 10892 MB/s
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.030681] raid6: sse2x2   xor()  7556 MB/s
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.098662] raid6: sse2x4   gen() 12445 MB/s
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.166660] raid6: sse2x4   xor()  8795 MB/s
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.168215] raid6: using algorithm sse2x4 gen() 12445 MB/s
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.169754] raid6: .... xor() 8795 MB/s, rmw enabled
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.171093] raid6: using ssse3x2 recovery algorithm
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.173872] xor: automatically using best checksumming function:
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.214661]    avx       : 21767.000 MB/sec
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.230293] Btrfs loaded
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.284938] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.287168] EXT4-fs (sda1): write access will be enabled during recovery
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.372901] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.387782] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.389421] EXT4-fs (sda1): recovery complete
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.396388] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.624297] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.749880] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    8.802468] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    9.013137] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    9.629295] random: cloud-init: uninitialized urandom read (32 bytes read, 42 bits of entropy available)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    9.780959] systemd-udevd[702]: starting version 204
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [    9.905097] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   10.014597] ppdev: user-space parallel port driver
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   10.114972] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   10.176003] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   10.241837] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   10.413319] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   10.674096] random: mktemp: uninitialized urandom read (12 bytes read, 57 bits of entropy available)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   10.756744] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   10.840826] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   10.888323] EXT4-fs (sda1): resized filesystem to 7864064
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   11.350090] init: failsafe main process (1093) killed by TERM signal
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO Running set_multiqueue.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO Set channels for eth0 to 4.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Starting Google Accounts daemon.
Aug 13 01:00:56 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Creating a new user account for me.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Created user account me.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Creating a new user account for henrik.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Created user account henrik.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Creating a new user account for emma.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Created user account emma.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Creating a new user account for igor.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Created user account igor.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Created user account konstantinhaase.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Creating a new user account for aj.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   12.416721] random: nonblocking pool is initialized
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Created user account aj.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Creating a new user account for solarce.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-clock-skew: INFO Synced system time with hardware clock.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Created user account solarce.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Creating a new user account for asari.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Created user account asari.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Creating a new user account for bogdana.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Created user account bogdana.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Creating a new user account for konstantin.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Created user account konstantin.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Creating a new user account for carmen.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Created user account carmen.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Creating a new user account for maria.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   12.784739] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Created user account maria.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   12.788943] Bridge firewalling registered
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   12.802072] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c google-accounts: INFO Removing user packer.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   12.836132] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   12.904569] Initializing XFRM netlink socket
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   12.912797] Netfilter messages via NETLINK v0.30.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   12.915830] ctnetlink v0.93: registering with nfnetlink.
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   12.962697] floppy0: no floppy controllers found
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   12.962897] work still pending
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c cron[1704]: (CRON) INFO (pidfile fd = 3)
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c cron[1735]: (CRON) STARTUP (fork ok)
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c cron[1735]: (CRON) INFO (Running @reboot jobs)
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c acpid: starting up with netlink and the input layer
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c acpid: 1 rule loaded
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c acpid: waiting for events: event logging is off
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c haveged: haveged starting up
Aug 13 01:00:57 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   13.443695] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 01:01:20 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpdate[1858]: adjust time server 169.254.169.254 offset 0.015490 sec
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpd[1893]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpd[1894]: proto: precision = 0.103 usec
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpd[1894]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpd[1894]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpd[1894]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpd[1894]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpd[1894]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpd[1894]: Listen normally on 3 eth0 10.20.0.192 UDP 123
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpd[1894]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpd[1894]: peers refreshed
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpd[1894]: Listening on routing socket on fd #21 for interface updates
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   43.653785] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c startup-script: INFO Found startup-script in metadata.
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c startup-script: INFO startup-script: job 1 at Mon Aug 13 04:11:00 2018
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c startup-script: INFO startup-script: Return code 0.
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c startup-script: INFO startup-script: Return code 0.
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c startup-script: INFO Finished running startup scripts.
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ec2: 
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ec2: #############################################################
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ec2: 1024 26:dd:db:57:8a:bf:4c:5a:43:fd:08:de:b2:b3:59:db  root@travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c (DSA)
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ec2: 256 d3:41:1c:90:80:a2:16:0d:03:2c:05:d7:94:04:88:1a  root@travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c (ECDSA)
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ec2: 256 b7:b2:e6:ee:64:e4:6b:bc:cb:76:fb:d2:6a:86:c5:2d  root@travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c (ED25519)
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ec2: 2048 e1:41:ca:9f:37:42:d2:92:24:d0:6e:1b:85:d9:92:ff  root@travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c (RSA)
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 13 01:01:28 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ec2: #############################################################
Aug 13 01:01:59 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [   74.757454] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 13 01:03:03 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [  138.951910] device vethc0486ae entered promiscuous mode
Aug 13 01:03:03 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [  139.033189] cgroup: docker-runc (4877) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 13 01:03:03 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [  139.033192] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 13 01:03:03 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [  139.112874] eth0: renamed from vethc638aca
Aug 13 01:03:03 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [  139.158741] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 13 01:03:03 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [  139.159968] docker0: port 1(vethc0486ae) entered forwarding state
Aug 13 01:03:03 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [  139.159983] docker0: port 1(vethc0486ae) entered forwarding state
Aug 13 01:03:03 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [  139.160003] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 13 01:03:07 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpd[1894]: Listen normally on 5 docker0 fe80::42:33ff:fedf:96d8 UDP 123
Aug 13 01:03:07 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpd[1894]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 13 01:03:07 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpd[1894]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 13 01:03:07 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpd[1894]: peers refreshed
Aug 13 01:03:07 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c ntpd[1894]: new interface(s) found: waking up resolver
Aug 13 01:03:18 travis-job-4d7490df-67b1-4ee4-a477-1e31260c3f1c kernel: [  154.207950] docker0: port 1(vethc0486ae) entered forwarding state
---
travis_time:end:002f9e2c:start=1534122365795795366,finish=1534122365803646711,duration=7851345
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00205a96
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0143da32
travis_time:start:0143da32
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:01a56f4d
$ dmesg | grep -i kill
