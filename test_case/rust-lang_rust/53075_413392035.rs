plain

[00:04:53] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:54] tidy error: /checkout/src/librustc/mir/cache.rs:58: line longer than 100 chars
[00:04:55] some tidy checks failed
[00:04:55] 
[00:04:55] 
[00:04:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:55] 
[00:04:55] 
[00:04:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:55] Build completed unsuccessfully in 0:00:55
[00:04:55] Build completed unsuccessfully in 0:00:55
[00:04:55] make: *** [tidy] Error 1
[00:04:55] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:148ec960
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0b0ab454
$ sudo tail -n 500 /var/log/syslog
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   3 disabled
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   4 disabled
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   5 disabled
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   6 disabled
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   7 disabled
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Using GB pages for direct mapping
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] kvm-clock: using sched offset of 1357106031 cycles
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Zone ranges:
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   Device   empty
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Movable zone start for each node
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Early memory node ranges
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Policy zone: Normal
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Hierarchical RCU implementation.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] console [ttyS0] enabled
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.313613] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.314807] pid_max: default: 32768 minimum: 301
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.315522] ACPI: Core revision 20150930
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.322114] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.323240] Security Framework initialized
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.323879] Yama: becoming mindful.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.324422] AppArmor: AppArmor disabled by boot time parameter
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.327199] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.337474] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.341963] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.343050] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.344265] Initializing cgroup subsys io
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.344970] Initializing cgroup subsys memory
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.345696] Initializing cgroup subsys devices
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.346338] Initializing cgroup subsys freezer
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.346950] Initializing cgroup subsys net_cls
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.347612] Initializing cgroup subsys perf_event
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.348257] Initializing cgroup subsys net_prio
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.349005] Initializing cgroup subsys hugetlb
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.349701] Initializing cgroup subsys pids
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.350382] CPU: Physical Processor ID: 0
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.351221] CPU: Processor Core ID: 0
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.353066] mce: CPU supports 32 MCE banks
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.353884] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.354743] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.358259] Freeing SMP alternatives memory: 32K
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.368735] ftrace: allocating 32185 entries in 126 pages
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.425476] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.426528] smpboot: Max logical packages: 2
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.427726] x2apic enabled
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.429716] Switched APIC routing to physical x2apic.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.433333] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.540859] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.542460] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.545555] x86: Booting SMP configuration:
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.546386] .... node  #0, CPUs:      #1
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.547220] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.551666]  #2
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.552216] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.557901]  #3
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.558346] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.562841] x86: Booted up 1 node, 4 CPUs
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.563456] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.566029] devtmpfs: initialized
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.570519] evm: security.selinux
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.571131] evm: security.SMACK64
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.571604] evm: security.SMACK64EXEC
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.572126] evm: security.SMACK64TRANSMUTE
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.572690] evm: security.SMACK64MMAP
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.573206] evm: security.ima
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.573630] evm: security.capability
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.574437] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.575800] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.576800] pinctrl core: initialized pinctrl subsystem
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.577710] RTC time:  1:09:24, date: 08/16/18
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.579328] NET: Registered protocol family 16
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.588885] cpuidle: using governor ladder
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.600882] cpuidle: using governor menu
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.601479] PCCT header not found.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.602047] ACPI: bus type PCI registered
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.602659] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.603751] PCI: Using configuration type 1 for base access
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.617818] ACPI: Added _OSI(Module Device)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.618547] ACPI: Added _OSI(Processor Device)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.619244] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.619922] ACPI: Added _OSI(Processor Aggregator Device)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.623421] ACPI: Executed 2 blocks of module-level executable AML code
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.647703] ACPI: Interpreter enabled
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.648429] ACPI: (supports S0 S3 S4 S5)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.649382] ACPI: Using IOAPIC for interrupt routing
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.650266] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.680294] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.681286] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.682274] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.683184] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.685832] PCI host bridge to bus 0000:00
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.686554] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.687665] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.688745] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.689885] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.691364] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.692552] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.693036] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.708104] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.724835] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.726224] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.731584] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.736384] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.750531] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.756031] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.760532] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.773878] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.776084] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.778234] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.780541] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.782596] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.803657] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.804688] vgaarb: loaded
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.805355] SCSI subsystem initialized
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.805996] libata version 3.00 loaded.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.806022] ACPI: bus type USB registered
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.806628] usbcore: registered new interface driver usbfs
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.807388] usbcore: registered new interface driver hub
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.808188] usbcore: registered new device driver usb
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.809620] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.810686] dmi: Firmware registration failed.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.811492] PCI: Using ACPI for IRQ routing
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.812095] PCI: pci_cache_line_size set to 64 bytes
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.812198] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.812200] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.812306] NetLabel: Initializing
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.812786] NetLabel:  domain hash size = 128
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.813390] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.814078] NetLabel:  unlabeled traffic allowed by default
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.814941] amd_nb: Cannot enumerate AMD northbridges
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.815705] clocksource: Switched to clocksource kvm-clock
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.823308] pnp: PnP ACPI init
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.824080] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.824146] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.824187] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.824254] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.824292] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.824331] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.824370] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.824530] pnp: PnP ACPI: found 7 devices
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.831777] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.833133] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.833136] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.833138] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.833139] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.833173] NET: Registered protocol family 2
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.834155] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.836166] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.837474] TCP: Hash tables configured (established 131072 bind 65536)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.838512] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.839536] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.840695] NET: Registered protocol family 1
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.841366] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.842315] PCI: CLS 0 bytes, default 64
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    0.843097] Unpacking initramfs...
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.963001] Freeing initrd memory: 21432K
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.964626] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.966953] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.969771] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.973081] hw unit of domain pp0-core 2^-0 Joules
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.974999] hw unit of domain package 2^-0 Joules
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.976586] hw unit of domain dram 2^-0 Joules
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.978149] Scanning for low memory corruption every 60 seconds
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.980391] audit: initializing netlink subsys (disabled)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.981877] audit: type=2000 audit(1534381766.951:1): initialized
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.983954] Initialise system trusted keyring
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.985641] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.987583] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.990769] zbud: loaded
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.992191] VFS: Disk quotas dquot_6.6.0
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.993849] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.996211] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    2.998632] fuse init (API version 7.23)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.000148] Key type big_key registered
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.001229] Allocating IMA MOK and blacklist keyrings.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.005577] Key type asymmetric registered
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.006912] Asymmetric key parser 'x509' registered
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.008588] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.010997] io scheduler noop registered
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.012130] io scheduler deadline registered (default)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.013978] io scheduler cfq registered
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.015426] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.017800] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.020094] intel_idle: does not run on family 6 model 62
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.020193] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.022345] ACPI: Power Button [PWRF]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.023460] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.025865] ACPI: Sleep Button [SLPF]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.027886] GHES: HEST is not enabled!
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.031341] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.033322] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.040575] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.042572] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.050251] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.074029] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.098531] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.123365] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.147509] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.152690] Linux agpgart interface v0.103
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.158168] loop: module loaded
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.159417] libphy: Fixed MDIO Bus: probed
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.160890] tun: Universal TUN/TAP device driver, 1.6
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.162565] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.209444] PPP generic driver version 2.4.2
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.211020] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.213174] ehci-pci: EHCI PCI platform driver
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.214979] ehci-platform: EHCI generic platform driver
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.216771] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.218868] ohci-pci: OHCI PCI platform driver
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.220217] ohci-platform: OHCI generic platform driver
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.221915] uhci_hcd: USB Universal Host Controller Interface driver
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.224309] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.227166] i8042: Warning: Keylock active
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.229829] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.231602] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.233976] mousedev: PS/2 mouse device common for all mice
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.237024] rtc_cmos 00:00: RTC can wake from S4
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.238897] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.241666] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.243449] i2c /dev entries driver
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.244886] device-mapper: uevent: version 1.0.3
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.246739] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.250150] ledtrig-cpu: registered to indicate activity on CPUs
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.253239] NET: Registered protocol family 10
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.255344] NET: Registered protocol family 17
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.257248] Key type dns_resolver registered
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.259090] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.261082] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.263000] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.264725] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.266708] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.270136] registered taskstats version 1
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.271585] Loading compiled-in X.509 certificates
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.273796] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.277373] zswap: loaded using pool lzo/zbud
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.281060] Key type trusted registered
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.286346] Key type encrypted registered
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.288017] ima: No TPM chip found, activating TPM-bypass!
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.289912] evm: HMAC attrs: 0x1
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.291455]   Magic number: 14:931:154
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.292938] acpi device:21: hash matches
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.294567] rtc_cmos 00:00: setting system clock to 2018-08-16 01:09:27 UTC (1534381767)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.297372] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.299169] EDD information not available.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.300598] PM: Hibernation image not present or could not be loaded.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.302061] Freeing unused kernel memory: 1496K
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.303646] Write protecting the kernel read-only data: 14336k
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.307417] Freeing unused kernel memory: 1956K
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.309053] Freeing unused kernel memory: 92K
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.325932] systemd-udevd[120]: starting version 204
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.389473] scsi host0: Virtio SCSI HBA
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.394401] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.402385] AVX version of gcm_enc/dec engaged.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.404080] AES CTR mode by8 optimization enabled
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.436136] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.440190] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.440238] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.443924] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.446013] sd 0:0:1:0: [sda] Write Protect is off
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.447809] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.448002] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.453260]  sda: sda1
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.455330] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.975822] tsc: Refined TSC clocksource calibration: 2499.779 MHz
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    3.978273] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2408690887f, max_idle_ns: 440795308497 ns
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    4.274108] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    6.371926] floppy0: no floppy controllers found
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    7.547738] raid6: sse2x1   gen()  8791 MB/s
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    7.615721] raid6: sse2x1   xor()  6827 MB/s
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    7.683726] raid6: sse2x2   gen() 10869 MB/s
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    7.751726] raid6: sse2x2   xor()  7438 MB/s
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    7.819724] raid6: sse2x4   gen() 12326 MB/s
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    7.887724] raid6: sse2x4   xor()  8565 MB/s
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    7.889216] raid6: using algorithm sse2x4 gen() 12326 MB/s
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    7.890910] raid6: .... xor() 8565 MB/s, rmw enabled
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    7.892892] raid6: using ssse3x2 recovery algorithm
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    7.896248] xor: automatically using best checksumming function:
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    7.935721]    avx       : 21389.000 MB/sec
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    7.952239] Btrfs loaded
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    8.012397] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    8.014609] EXT4-fs (sda1): write access will be enabled during recovery
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    8.092596] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    8.106991] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    8.108495] EXT4-fs (sda1): recovery complete
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    8.115083] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    8.336490] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    8.459366] random: mountall: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    8.510830] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    8.696913] random: cloud-init: uninitialized urandom read (32 bytes read, 34 bits of entropy available)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    9.272739] random: cloud-init: uninitialized urandom read (32 bytes read, 41 bits of entropy available)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    9.416413] systemd-udevd[702]: starting version 204
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    9.537448] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    9.640662] ppdev: user-space parallel port driver
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    9.742449] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    9.803151] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [    9.873742] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   10.042877] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   10.310350] random: mktemp: uninitialized urandom read (12 bytes read, 55 bits of entropy available)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   10.385287] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   10.463309] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   10.517187] EXT4-fs (sda1): resized filesystem to 7864064
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   10.768165] init: failsafe main process (1094) killed by TERM signal
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO Running set_multiqueue.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO Set channels for eth0 to 4.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e google-accounts: INFO Starting Google Accounts daemon.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e google-accounts: INFO Creating a new user account for me.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e google-accounts: INFO Created user account me.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e google-accounts: INFO Creating a new user account for bogdana.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e google-accounts: INFO Created user account bogdana.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e google-accounts: INFO Creating a new user account for aj.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e google-accounts: INFO Created user account aj.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e google-accounts: INFO Creating a new user account for asari.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e google-accounts: INFO Created user account asari.
Aug 16 01:09:35 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e google-accounts: INFO Removing user packer.
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e google-clock-skew: INFO Synced system time with hardware clock.
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   11.972407] random: nonblocking pool is initialized
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   12.166036] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   12.170771] Bridge firewalling registered
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   12.183899] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   12.218631] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   12.287382] Initializing XFRM netlink socket
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   12.298450] Netfilter messages via NETLINK v0.30.
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   12.301550] ctnetlink v0.93: registering with nfnetlink.
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e cron[1523]: (CRON) INFO (pidfile fd = 3)
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e cron[1581]: (CRON) STARTUP (fork ok)
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e cron[1581]: (CRON) INFO (Running @reboot jobs)
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e acpid: starting up with netlink and the input layer
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e acpid: 1 rule loaded
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e acpid: waiting for events: event logging is off
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e haveged: haveged starting up
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   12.600533] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 16 01:09:36 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   12.603844] floppy0: no floppy controllers found
Aug 16 01:10:00 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpdate[1793]: adjust time server 169.254.169.254 offset 0.003355 sec
Aug 16 01:10:06 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpd[1826]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 16 01:10:06 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpd[1827]: proto: precision = 0.106 usec
Aug 16 01:10:06 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpd[1827]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 16 01:10:06 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpd[1827]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 16 01:10:06 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpd[1827]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 16 01:10:06 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpd[1827]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 16 01:10:06 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpd[1827]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 16 01:10:06 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpd[1827]: Listen normally on 3 eth0 10.20.0.110 UDP 123
Aug 16 01:10:06 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpd[1827]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 16 01:10:06 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpd[1827]: peers refreshed
Aug 16 01:10:06 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpd[1827]: Listening on routing socket on fd #21 for interface updates
Aug 16 01:10:06 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   42.759229] init: plymouth-upstart-bridge main process ended, respawning
Aug 16 01:10:07 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e startup-script: INFO Found startup-script in metadata.
Aug 16 01:10:07 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 16 01:10:07 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e startup-script: INFO startup-script: job 1 at Thu Aug 16 04:20:00 2018
Aug 16 01:10:07 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e startup-script: INFO startup-script: Return code 0.
Aug 16 01:10:07 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e startup-script: INFO startup-script: Return code 0.
Aug 16 01:10:07 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e startup-script: INFO Finished running startup scripts.
Aug 16 01:10:07 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ec2: 
Aug 16 01:10:07 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ec2: #############################################################
Aug 16 01:10:07 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 16 01:10:07 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ec2: 1024 5f:60:76:fc:2e:89:f9:e9:47:b8:bd:aa:64:44:de:02  root@travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e (DSA)
Aug 16 01:10:07 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ec2: 256 72:b6:a6:ae:81:ab:fc:45:bd:21:49:07:6d:dc:0b:e8  root@travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e (ECDSA)
Aug 16 01:10:07 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ec2: 256 63:0e:21:d6:0c:0a:a1:9c:4d:fa:b0:e2:20:a6:44:d1  root@travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e (ED25519)
Aug 16 01:10:07 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ec2: 2048 53:13:d8:1e:5f:1d:56:a4:9d:65:dd:f7:62:2f:94:c4  root@travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e (RSA)
Aug 16 01:10:07 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 16 01:10:07 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ec2: #############################################################
Aug 16 01:10:38 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [   74.183509] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 16 01:12:23 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [  179.140587] device veth919316e entered promiscuous mode
Aug 16 01:12:23 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [  179.140638] docker0: port 1(veth919316e) entered forwarding state
Aug 16 01:12:23 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [  179.140644] docker0: port 1(veth919316e) entered forwarding state
Aug 16 01:12:23 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [  179.141260] docker0: port 1(veth919316e) entered disabled state
Aug 16 01:12:23 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [  179.235202] cgroup: docker-runc (4908) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 16 01:12:23 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [  179.235204] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 16 01:12:23 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [  179.306935] eth0: renamed from veth9d9fc63
Aug 16 01:12:23 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [  179.348731] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 16 01:12:23 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [  179.350188] docker0: port 1(veth919316e) entered forwarding state
Aug 16 01:12:23 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [  179.350218] docker0: port 1(veth919316e) entered forwarding state
Aug 16 01:12:23 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [  179.350244] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 16 01:12:26 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpd[1827]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 16 01:12:26 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpd[1827]: Listen normally on 6 docker0 fe80::42:9bff:fe45:d96b UDP 123
Aug 16 01:12:26 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpd[1827]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 16 01:12:26 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpd[1827]: peers refreshed
Aug 16 01:12:26 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e ntpd[1827]: new interface(s) found: waking up resolver
Aug 16 01:12:38 travis-job-2ba8b42e-932f-4d1a-948e-ee483ad69d6e kernel: [  194.374526] docker0: port 1(veth919316e) entered forwarding state
---
travis_time:end:010a27ee:start=1534382135133206233,finish=1534382135141617937,duration=8411704
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:29ec4a18
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c929e55
travis_time:start:1c929e55
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:036499fe
$ dmesg | grep -i kill
