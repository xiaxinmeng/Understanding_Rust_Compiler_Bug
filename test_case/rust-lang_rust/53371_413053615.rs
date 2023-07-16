plain

[00:03:54] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:54] tidy error: /checkout/src/test/ui/elide-errors-on-mismatched-tuple.rs: missing trailing newline
[00:03:55] some tidy checks failed
[00:03:55] 
[00:03:55] 
[00:03:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:55] 
[00:03:55] 
[00:03:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:55] Build completed unsuccessfully in 0:00:50
[00:03:55] Build completed unsuccessfully in 0:00:50
[00:03:55] Makefile:79: recipe for target 'tidy' failed
[00:03:55] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02ab44c9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:10c99f50
$ sudo tail -n 500 /var/log/syslog
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   1 disabled
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   2 disabled
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   3 disabled
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   4 disabled
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   5 disabled
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   6 disabled
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   7 disabled
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Using GB pages for direct mapping
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] kvm-clock: using sched offset of 1619913145 cycles
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Zone ranges:
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   Device   empty
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Movable zone start for each node
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Early memory node ranges
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Policy zone: Normal
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Hierarchical RCU implementation.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] console [ttyS0] enabled
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.510186] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.514182] pid_max: default: 32768 minimum: 301
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.516683] ACPI: Core revision 20150930
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.524669] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.527886] Security Framework initialized
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.529677] Yama: becoming mindful.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.531405] AppArmor: AppArmor disabled by boot time parameter
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.535454] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.547569] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.554583] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.557730] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.561253] Initializing cgroup subsys io
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.562760] Initializing cgroup subsys memory
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.564269] Initializing cgroup subsys devices
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.566197] Initializing cgroup subsys freezer
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.567875] Initializing cgroup subsys net_cls
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.569664] Initializing cgroup subsys perf_event
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.571574] Initializing cgroup subsys net_prio
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.573830] Initializing cgroup subsys hugetlb
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.575647] Initializing cgroup subsys pids
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.577531] CPU: Physical Processor ID: 0
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.579233] CPU: Processor Core ID: 0
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.581914] mce: CPU supports 32 MCE banks
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.584266] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.586662] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.591908] Freeing SMP alternatives memory: 32K
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.603775] ftrace: allocating 32185 entries in 126 pages
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.663609] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.667464] smpboot: Max logical packages: 2
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.670296] x2apic enabled
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.672752] Switched APIC routing to physical x2apic.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.678121] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.788200] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.792799] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.798791] x86: Booting SMP configuration:
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.800412] .... node  #0, CPUs:      #1
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.802497] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.808854]  #2
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.810026] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.816234]  #3
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.817501] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.823376] x86: Booted up 1 node, 4 CPUs
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.825975] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.831010] devtmpfs: initialized
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.836965] evm: security.selinux
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.838492] evm: security.SMACK64
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.839735] evm: security.SMACK64EXEC
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.841331] evm: security.SMACK64TRANSMUTE
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.843389] evm: security.SMACK64MMAP
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.845816] evm: security.ima
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.847375] evm: security.capability
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.849255] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.853590] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.856814] pinctrl core: initialized pinctrl subsystem
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.859678] RTC time: 23:49:51, date: 08/14/18
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.862403] NET: Registered protocol family 16
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.868802] cpuidle: using governor ladder
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.876796] cpuidle: using governor menu
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.878648] PCCT header not found.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.880267] ACPI: bus type PCI registered
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.881802] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.885016] PCI: Using configuration type 1 for base access
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.898501] ACPI: Added _OSI(Module Device)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.900890] ACPI: Added _OSI(Processor Device)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.902673] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.905240] ACPI: Added _OSI(Processor Aggregator Device)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.910272] ACPI: Executed 2 blocks of module-level executable AML code
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.938430] ACPI: Interpreter enabled
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.940499] ACPI: (supports S0 S3 S4 S5)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.942431] ACPI: Using IOAPIC for interrupt routing
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.944353] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.978734] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.981281] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.983983] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.987061] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.992671] PCI host bridge to bus 0000:00
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.994540] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    0.997534] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.000387] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.003131] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.006588] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.008774] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.009244] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.032206] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.056236] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.060593] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.070023] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.077874] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.100022] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.110344] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.118061] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.140025] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.145293] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.149325] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.153620] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.157825] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.181379] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.184393] vgaarb: loaded
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.186025] SCSI subsystem initialized
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.187374] libata version 3.00 loaded.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.187397] ACPI: bus type USB registered
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.189105] usbcore: registered new interface driver usbfs
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.191093] usbcore: registered new interface driver hub
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.193587] usbcore: registered new device driver usb
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.195649] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.198005] dmi: Firmware registration failed.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.199703] PCI: Using ACPI for IRQ routing
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.201377] PCI: pci_cache_line_size set to 64 bytes
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.201499] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.201502] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.201635] NetLabel: Initializing
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.202952] NetLabel:  domain hash size = 128
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.204335] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.206127] NetLabel:  unlabeled traffic allowed by default
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.208156] amd_nb: Cannot enumerate AMD northbridges
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.209841] clocksource: Switched to clocksource kvm-clock
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.219209] pnp: PnP ACPI init
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.220922] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.220997] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.221039] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.221091] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.221131] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.221170] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.221209] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.221375] pnp: PnP ACPI: found 7 devices
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.230344] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.233974] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.233976] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.233978] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.233979] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.234013] NET: Registered protocol family 2
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.235544] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.238977] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.241855] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.244525] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.246978] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.250342] NET: Registered protocol family 1
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.251981] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.253995] PCI: CLS 0 bytes, default 64
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    1.254057] Unpacking initramfs...
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.352128] Freeing initrd memory: 21432K
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.353077] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.354484] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.356867] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.358311] hw unit of domain pp0-core 2^-0 Joules
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.359222] hw unit of domain package 2^-0 Joules
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.361316] hw unit of domain dram 2^-0 Joules
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.362372] Scanning for low memory corruption every 60 seconds
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.364201] audit: initializing netlink subsys (disabled)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.365295] audit: type=2000 audit(1534290593.204:1): initialized
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.366693] Initialise system trusted keyring
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.367674] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.368798] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.370936] zbud: loaded
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.371727] VFS: Disk quotas dquot_6.6.0
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.372367] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.373646] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.375039] fuse init (API version 7.23)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.375805] Key type big_key registered
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.376505] Allocating IMA MOK and blacklist keyrings.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.378703] Key type asymmetric registered
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.379869] Asymmetric key parser 'x509' registered
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.381051] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.382444] io scheduler noop registered
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.383220] io scheduler deadline registered (default)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.384189] io scheduler cfq registered
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.385284] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.386519] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.387686] intel_idle: does not run on family 6 model 62
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.387794] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.388878] ACPI: Power Button [PWRF]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.389863] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.391698] ACPI: Sleep Button [SLPF]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.393132] GHES: HEST is not enabled!
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.395749] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.396825] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.401881] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.402894] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.407910] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.431299] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.456096] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.479754] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.503331] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.507238] Linux agpgart interface v0.103
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.511407] loop: module loaded
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.512430] libphy: Fixed MDIO Bus: probed
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.513377] tun: Universal TUN/TAP device driver, 1.6
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.514497] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.550668] PPP generic driver version 2.4.2
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.551927] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.553553] ehci-pci: EHCI PCI platform driver
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.554566] ehci-platform: EHCI generic platform driver
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.555735] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.557041] ohci-pci: OHCI PCI platform driver
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.557971] ohci-platform: OHCI generic platform driver
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.558973] uhci_hcd: USB Universal Host Controller Interface driver
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.560783] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.563673] i8042: Warning: Keylock active
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.565433] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.566709] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.568258] mousedev: PS/2 mouse device common for all mice
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.569891] rtc_cmos 00:00: RTC can wake from S4
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.571353] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.573045] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.574616] i2c /dev entries driver
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.575393] device-mapper: uevent: version 1.0.3
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.576472] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.578346] ledtrig-cpu: registered to indicate activity on CPUs
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.580322] NET: Registered protocol family 10
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.581747] NET: Registered protocol family 17
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.583040] Key type dns_resolver registered
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.584592] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.585937] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.587769] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.589144] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.590617] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.593121] registered taskstats version 1
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.594175] Loading compiled-in X.509 certificates
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.596056] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.599151] zswap: loaded using pool lzo/zbud
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.602540] Key type trusted registered
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.607304] Key type encrypted registered
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.608577] ima: No TPM chip found, activating TPM-bypass!
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.610450] evm: HMAC attrs: 0x1
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.611618]   Magic number: 14:293:858
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.612679] rtc_cmos 00:00: setting system clock to 2018-08-14 23:49:53 UTC (1534290593)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.615539] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.617191] EDD information not available.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.618175] PM: Hibernation image not present or could not be loaded.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.619701] Freeing unused kernel memory: 1496K
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.621178] Write protecting the kernel read-only data: 14336k
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.624229] Freeing unused kernel memory: 1956K
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.626604] Freeing unused kernel memory: 92K
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.643438] systemd-udevd[119]: starting version 204
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.700803] scsi host0: Virtio SCSI HBA
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.708889] AVX version of gcm_enc/dec engaged.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.709887] AES CTR mode by8 optimization enabled
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.710750] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.749301] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.749335] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.751750] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.753160] sd 0:0:1:0: [sda] Write Protect is off
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.754078] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.754291] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.757597]  sda: sda1
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.759074] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    3.770288] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    4.361972] tsc: Refined TSC clocksource calibration: 2499.756 MHz
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    4.363377] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2408537a861, max_idle_ns: 440795269937 ns
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    4.603099] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    6.690109] floppy0: no floppy controllers found
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    7.881889] raid6: sse2x1   gen()  8934 MB/s
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    7.949907] raid6: sse2x1   xor()  6810 MB/s
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.017940] raid6: sse2x2   gen() 11059 MB/s
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.085912] raid6: sse2x2   xor()  7635 MB/s
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.153896] raid6: sse2x4   gen() 12407 MB/s
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.221889] raid6: sse2x4   xor()  8601 MB/s
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.224679] raid6: using algorithm sse2x4 gen() 12407 MB/s
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.226575] raid6: .... xor() 8601 MB/s, rmw enabled
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.228775] raid6: using ssse3x2 recovery algorithm
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.232329] xor: automatically using best checksumming function:
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.273903]    avx       : 21639.000 MB/sec
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.292219] Btrfs loaded
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.347746] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.350280] EXT4-fs (sda1): write access will be enabled during recovery
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.444058] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.459872] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.461562] EXT4-fs (sda1): recovery complete
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.469010] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.706683] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.841005] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    8.895001] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    9.125915] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    9.765715] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [    9.931185] systemd-udevd[702]: starting version 204
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   10.056485] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   10.171056] ppdev: user-space parallel port driver
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   10.301347] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   10.368626] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   10.434911] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   10.604169] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   10.878510] random: mktemp: uninitialized urandom read (12 bytes read, 57 bits of entropy available)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   10.963818] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   11.065084] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   11.106218] EXT4-fs (sda1): resized filesystem to 7864064
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   11.523374] init: failsafe main process (1094) killed by TERM signal
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO Running set_multiqueue.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO Set channels for eth0 to 4.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 14 23:50:01 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 14 23:50:02 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 14 23:50:02 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d google-accounts: INFO Starting Google Accounts daemon.
Aug 14 23:50:02 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d google-accounts: INFO Creating a new user account for me.
Aug 14 23:50:02 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 23:50:02 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d google-accounts: INFO Created user account me.
Aug 14 23:50:02 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d google-accounts: INFO Created user account me.
Aug 14 23:50:02 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d google-accounts: INFO Creating a new user account for aj.
Aug 14 23:50:02 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d google-accounts: INFO Created user account aj.
Aug 14 23:50:02 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d google-accounts: INFO Creating a new user account for carmen.
Aug 14 23:50:02 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d google-accounts: INFO Created user account carmen.
Aug 14 23:50:02 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d google-accounts: INFO Removing user packer.
Aug 14 23:50:03 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d google-clock-skew: INFO Synced system time with hardware clock.
Aug 14 23:50:03 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   12.863340] random: nonblocking pool is initialized
Aug 14 23:50:03 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   12.987461] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 23:50:03 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   12.991617] Bridge firewalling registered
Aug 14 23:50:03 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   13.003380] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 14 23:50:03 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   13.040246] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 23:50:03 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   13.123518] Initializing XFRM netlink socket
Aug 14 23:50:03 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   13.132363] Netfilter messages via NETLINK v0.30.
Aug 14 23:50:03 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   13.135884] ctnetlink v0.93: registering with nfnetlink.
Aug 14 23:50:03 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   13.146012] floppy0: no floppy controllers found
Aug 14 23:50:03 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   13.146062] floppy0: floppy_shutdown: timeout handler died.  
Aug 14 23:50:03 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 23:50:03 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 23:50:04 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 23:50:04 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 23:50:04 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d cron[1617]: (CRON) INFO (pidfile fd = 3)
Aug 14 23:50:04 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d cron[1657]: (CRON) STARTUP (fork ok)
Aug 14 23:50:04 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d cron[1657]: (CRON) INFO (Running @reboot jobs)
Aug 14 23:50:04 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d acpid: starting up with netlink and the input layer
Aug 14 23:50:04 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d acpid: 1 rule loaded
Aug 14 23:50:04 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d acpid: waiting for events: event logging is off
Aug 14 23:50:04 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d haveged: haveged starting up
Aug 14 23:50:04 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   13.869402] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpd[1763]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpd[1764]: proto: precision = 0.101 usec
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpd[1764]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpd[1764]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpd[1764]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpd[1764]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpd[1764]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpd[1764]: Listen normally on 3 eth0 10.20.0.91 UDP 123
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpd[1764]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpd[1764]: peers refreshed
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpd[1764]: Listening on routing socket on fd #21 for interface updates
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   19.060005] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d startup-script: INFO Found startup-script in metadata.
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d startup-script: INFO startup-script: job 1 at Wed Aug 15 03:00:00 2018
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d startup-script: INFO startup-script: Return code 0.
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d startup-script: INFO startup-script: Return code 0.
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d startup-script: INFO Finished running startup scripts.
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ec2: 
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ec2: #############################################################
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ec2: 1024 d1:b8:70:25:79:42:2b:58:4f:6d:8b:dd:d7:3a:c3:cc  root@travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d (DSA)
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ec2: 256 e4:ab:32:a4:80:d2:95:17:1d:b9:52:60:b5:a8:d5:d9  root@travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d (ECDSA)
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ec2: 256 a5:99:f6:2d:8d:59:9d:25:a6:9a:27:a8:b7:da:28:22  root@travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d (ED25519)
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ec2: 2048 cc:76:d4:b3:38:f9:ec:94:ad:91:d2:81:1e:66:c3:79  root@travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d (RSA)
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 14 23:50:09 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ec2: #############################################################
Aug 14 23:50:18 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpdate[2160]: the NTP socket is in use, exiting
Aug 14 23:50:56 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [   65.727651] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 14 23:52:04 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [  134.528744] device veth7415682 entered promiscuous mode
Aug 14 23:52:04 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [  134.528808] docker0: port 1(veth7415682) entered forwarding state
Aug 14 23:52:04 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [  134.528817] docker0: port 1(veth7415682) entered forwarding state
Aug 14 23:52:04 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [  134.529515] docker0: port 1(veth7415682) entered disabled state
Aug 14 23:52:04 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [  134.633687] cgroup: docker-runc (4774) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 23:52:04 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [  134.633689] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 23:52:05 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [  134.711363] eth0: renamed from veth70e86d8
Aug 14 23:52:05 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [  134.754200] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 23:52:05 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [  134.755737] docker0: port 1(veth7415682) entered forwarding state
Aug 14 23:52:05 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [  134.755755] docker0: port 1(veth7415682) entered forwarding state
Aug 14 23:52:05 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [  134.755779] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 14 23:52:08 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpd[1764]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 14 23:52:08 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpd[1764]: Listen normally on 6 docker0 fe80::42:b6ff:fea1:3aa4 UDP 123
Aug 14 23:52:08 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpd[1764]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 14 23:52:08 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpd[1764]: peers refreshed
Aug 14 23:52:08 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d ntpd[1764]: new interface(s) found: waking up resolver
Aug 14 23:52:20 travis-job-e2356d9f-9775-4c4c-8f5d-50ee84b2441d kernel: [  149.792798] docker0: port 1(veth7415682) entered forwarding state
---
travis_time:end:0293fa54:start=1534290893405871041,finish=1534290893412731997,duration=6860956
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:36e33d4b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:006a56b7
travis_time:start:006a56b7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:00ac020c
$ dmesg | grep -i kill
