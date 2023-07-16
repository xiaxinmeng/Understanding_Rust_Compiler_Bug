plain

[00:04:01] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:01] tidy error: /checkout/src/librustdoc/html/render.rs:3783: line longer than 100 chars
[00:04:03] some tidy checks failed
[00:04:03] 
[00:04:03] 
[00:04:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:03] 
[00:04:03] 
[00:04:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:03] Build completed unsuccessfully in 0:00:54
[00:04:03] Build completed unsuccessfully in 0:00:54
[00:04:03] make: *** [tidy] Error 1
[00:04:03] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c304f45
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:13fff80c
$ sudo tail -n 500 /var/log/syslog
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   2 disabled
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   3 disabled
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   4 disabled
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   5 disabled
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   6 disabled
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   7 disabled
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Using GB pages for direct mapping
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] kvm-clock: using sched offset of 1623275964 cycles
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Zone ranges:
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   Device   empty
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Movable zone start for each node
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Early memory node ranges
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Policy zone: Normal
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Hierarchical RCU implementation.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] console [ttyS0] enabled
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.602691] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.606539] pid_max: default: 32768 minimum: 301
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.609024] ACPI: Core revision 20150930
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.617009] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.620024] Security Framework initialized
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.622479] Yama: becoming mindful.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.625436] AppArmor: AppArmor disabled by boot time parameter
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.633434] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.646378] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.653966] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.657755] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.662447] Initializing cgroup subsys io
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.665241] Initializing cgroup subsys memory
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.667652] Initializing cgroup subsys devices
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.670126] Initializing cgroup subsys freezer
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.672615] Initializing cgroup subsys net_cls
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.674856] Initializing cgroup subsys perf_event
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.677419] Initializing cgroup subsys net_prio
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.680323] Initializing cgroup subsys hugetlb
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.682866] Initializing cgroup subsys pids
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.685768] CPU: Physical Processor ID: 0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.688975] CPU: Processor Core ID: 0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.692100] mce: CPU supports 32 MCE banks
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.695251] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.699768] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.706012] Freeing SMP alternatives memory: 32K
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.717848] ftrace: allocating 32185 entries in 126 pages
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.774339] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.778724] smpboot: Max logical packages: 2
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.782630] x2apic enabled
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.785650] Switched APIC routing to physical x2apic.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.791467] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.900178] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.905561] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.911060] x86: Booting SMP configuration:
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.914009] .... node  #0, CPUs:      #1
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.916216] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.922545]  #2
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.923731] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.931290]  #3
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.932430] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.939334] x86: Booted up 1 node, 4 CPUs
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.941368] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.946580] devtmpfs: initialized
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.952070] evm: security.selinux
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.953640] evm: security.SMACK64
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.955095] evm: security.SMACK64EXEC
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.956620] evm: security.SMACK64TRANSMUTE
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.959220] evm: security.SMACK64MMAP
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.960843] evm: security.ima
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.962392] evm: security.capability
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.964493] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.969068] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.971964] pinctrl core: initialized pinctrl subsystem
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.974688] RTC time: 11:16:09, date: 08/11/18
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.977760] NET: Registered protocol family 16
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    0.988243] cpuidle: using governor ladder
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.000240] cpuidle: using governor menu
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.002380] PCCT header not found.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.004139] ACPI: bus type PCI registered
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.005912] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.009166] PCI: Using configuration type 1 for base access
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.026455] ACPI: Added _OSI(Module Device)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.031272] ACPI: Added _OSI(Processor Device)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.033625] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.036213] ACPI: Added _OSI(Processor Aggregator Device)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.041968] ACPI: Executed 2 blocks of module-level executable AML code
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.067647] ACPI: Interpreter enabled
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.069432] ACPI: (supports S0 S3 S4 S5)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.071932] ACPI: Using IOAPIC for interrupt routing
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.074802] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.108124] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.111831] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.115917] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.119075] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.125937] PCI host bridge to bus 0000:00
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.128201] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.134383] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.138863] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.143231] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.147722] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.150764] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.151227] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.175454] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.200808] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.204897] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.214566] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.223696] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.248819] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.259219] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.267360] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.291980] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.297354] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.302351] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.307211] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.312148] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.334719] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.337451] vgaarb: loaded
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.339146] SCSI subsystem initialized
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.341194] libata version 3.00 loaded.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.341221] ACPI: bus type USB registered
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.343059] usbcore: registered new interface driver usbfs
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.345685] usbcore: registered new interface driver hub
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.348294] usbcore: registered new device driver usb
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.350979] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.354385] dmi: Firmware registration failed.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.357140] PCI: Using ACPI for IRQ routing
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.359082] PCI: pci_cache_line_size set to 64 bytes
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.359179] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.359181] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.359313] NetLabel: Initializing
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.361052] NetLabel:  domain hash size = 128
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.363201] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.365700] NetLabel:  unlabeled traffic allowed by default
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.368763] amd_nb: Cannot enumerate AMD northbridges
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.371356] clocksource: Switched to clocksource kvm-clock
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.381033] pnp: PnP ACPI init
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.383056] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.383126] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.383173] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.383224] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.383266] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.383310] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.383378] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.383557] pnp: PnP ACPI: found 7 devices
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.393956] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.399144] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.399147] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.399148] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.399150] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.399187] NET: Registered protocol family 2
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.401397] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.404952] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.408750] TCP: Hash tables configured (established 131072 bind 65536)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.411955] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.414717] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.418859] NET: Registered protocol family 1
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.420899] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.424706] PCI: CLS 0 bytes, default 64
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    1.424782] Unpacking initramfs...
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.489384] Freeing initrd memory: 21432K
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.492673] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.497805] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.504036] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.508525] hw unit of domain pp0-core 2^-0 Joules
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.510902] hw unit of domain package 2^-0 Joules
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.513762] hw unit of domain dram 2^-16 Joules
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.516465] Scanning for low memory corruption every 60 seconds
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.520306] audit: initializing netlink subsys (disabled)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.523513] audit: type=2000 audit(1533986171.568:1): initialized
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.527868] Initialise system trusted keyring
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.530470] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.534191] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.539766] zbud: loaded
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.541891] VFS: Disk quotas dquot_6.6.0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.544262] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.548129] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.551790] fuse init (API version 7.23)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.554170] Key type big_key registered
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.556028] Allocating IMA MOK and blacklist keyrings.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.562546] Key type asymmetric registered
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.564976] Asymmetric key parser 'x509' registered
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.567695] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.571546] io scheduler noop registered
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.573424] io scheduler deadline registered (default)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.576734] io scheduler cfq registered
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.579070] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.582475] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.585740] intel_idle: does not run on family 6 model 63
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.585853] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.590010] ACPI: Power Button [PWRF]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.592252] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.595732] ACPI: Sleep Button [SLPF]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.598058] GHES: HEST is not enabled!
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.603055] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.606254] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.615901] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.618697] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.629638] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.654997] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.681020] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.707273] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.733100] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.740564] Linux agpgart interface v0.103
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.748776] loop: module loaded
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.751050] libphy: Fixed MDIO Bus: probed
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.752958] tun: Universal TUN/TAP device driver, 1.6
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.756079] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.803287] PPP generic driver version 2.4.2
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.807511] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.811910] ehci-pci: EHCI PCI platform driver
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.814397] ehci-platform: EHCI generic platform driver
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.817775] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.821297] ohci-pci: OHCI PCI platform driver
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.825907] ohci-platform: OHCI generic platform driver
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.829398] uhci_hcd: USB Universal Host Controller Interface driver
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.834121] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.839821] i8042: Warning: Keylock active
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.843959] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.847681] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.851000] mousedev: PS/2 mouse device common for all mice
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.854719] rtc_cmos 00:00: RTC can wake from S4
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.857789] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.861383] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.865161] i2c /dev entries driver
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.866925] device-mapper: uevent: version 1.0.3
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.869116] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.873696] ledtrig-cpu: registered to indicate activity on CPUs
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.878797] NET: Registered protocol family 10
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.882496] NET: Registered protocol family 17
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.884737] Key type dns_resolver registered
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.887664] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.891147] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.894251] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.897288] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.900551] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.906028] registered taskstats version 1
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.908226] Loading compiled-in X.509 certificates
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.912248] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.918050] zswap: loaded using pool lzo/zbud
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.923753] Key type trusted registered
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.931584] Key type encrypted registered
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.934624] ima: No TPM chip found, activating TPM-bypass!
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.937637] evm: HMAC attrs: 0x1
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.940633]   Magic number: 14:722:276
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.944173] tty tty13: hash matches
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.946778] rtc_cmos 00:00: setting system clock to 2018-08-11 11:16:12 UTC (1533986172)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.951320] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.954982] EDD information not available.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.956870] PM: Hibernation image not present or could not be loaded.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.958413] Freeing unused kernel memory: 1496K
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.961153] Write protecting the kernel read-only data: 14336k
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.965075] Freeing unused kernel memory: 1956K
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.967813] Freeing unused kernel memory: 92K
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    3.986017] systemd-udevd[119]: starting version 204
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    4.049222] scsi host0: Virtio SCSI HBA
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    4.052709] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    4.066047] AVX2 version of gcm_enc/dec engaged.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    4.066352] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    4.074176] AES CTR mode by8 optimization enabled
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    4.141225] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    4.141365] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    4.141367] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    4.141815] sd 0:0:1:0: [sda] Write Protect is off
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    4.141817] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    4.141977] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    4.146829]  sda: sda1
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    4.148787] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    4.515542] tsc: Refined TSC clocksource calibration: 2300.000 MHz
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    4.519540] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735223b2, max_idle_ns: 440795277976 ns
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    4.888564] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    7.039630] floppy0: no floppy controllers found
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.203364] raid6: sse2x1   gen()  8762 MB/s
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.271364] raid6: sse2x1   xor()  6546 MB/s
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.339373] raid6: sse2x2   gen() 10640 MB/s
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.407379] raid6: sse2x2   xor()  7058 MB/s
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.475379] raid6: sse2x4   gen() 12379 MB/s
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.543370] raid6: sse2x4   xor()  8614 MB/s
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.611375] raid6: avx2x1   gen() 16766 MB/s
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.679370] raid6: avx2x2   gen() 19379 MB/s
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.747374] raid6: avx2x4   gen() 21916 MB/s
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.749616] raid6: using algorithm avx2x4 gen() 21916 MB/s
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.752296] raid6: using avx2x2 recovery algorithm
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.756664] xor: automatically using best checksumming function:
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.799403]    avx       : 26794.000 MB/sec
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.815895] Btrfs loaded
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.880107] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.883713] EXT4-fs (sda1): write access will be enabled during recovery
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.965560] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.975016] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.978249] EXT4-fs (sda1): recovery complete
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    8.986754] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    9.246092] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    9.388496] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    9.445124] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [    9.654175] random: cloud-init: uninitialized urandom read (32 bytes read, 33 bits of entropy available)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   10.255333] random: cloud-init: uninitialized urandom read (32 bytes read, 41 bits of entropy available)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   10.408326] systemd-udevd[702]: starting version 204
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   10.545275] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   10.596445] intel_rapl: no valid rapl domains found in package 0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   10.656008] ppdev: user-space parallel port driver
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   10.761514] random: mktemp: uninitialized urandom read (6 bytes read, 50 bits of entropy available)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   10.822773] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   10.890841] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   11.057960] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   11.364985] random: mktemp: uninitialized urandom read (12 bytes read, 55 bits of entropy available)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   11.442865] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   11.527206] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   11.585153] EXT4-fs (sda1): resized filesystem to 7864064
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   11.872729] init: failsafe main process (1096) killed by TERM signal
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO Running set_multiqueue.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO Set channels for eth0 to 4.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 11 11:16:20 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e google-clock-skew: INFO Clock drift token has changed: 0.
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e google-accounts: INFO Starting Google Accounts daemon.
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e google-accounts: INFO Creating a new user account for me.
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e google-accounts: INFO Created user account me.
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e google-accounts: INFO Creating a new user account for aj.
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e google-accounts: INFO Created user account aj.
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e google-accounts: INFO Creating a new user account for carmen.
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e google-accounts: INFO Created user account carmen.
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e google-accounts: INFO Removing user packer.
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   13.296511] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   13.300357] Bridge firewalling registered
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   13.312746] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   13.324755] random: nonblocking pool is initialized
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   13.344085] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   13.398239] Initializing XFRM netlink socket
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   13.404421] Netfilter messages via NETLINK v0.30.
Aug 11 11:16:21 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   13.407083] ctnetlink v0.93: registering with nfnetlink.
Aug 11 11:16:22 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e google-clock-skew: INFO Synced system time with hardware clock.
Aug 11 11:16:22 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   13.647465] floppy0: no floppy controllers found
Aug 11 11:16:22 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e cron[1547]: (CRON) INFO (pidfile fd = 3)
Aug 11 11:16:22 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e cron[1581]: (CRON) STARTUP (fork ok)
Aug 11 11:16:22 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e cron[1581]: (CRON) INFO (Running @reboot jobs)
Aug 11 11:16:22 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 11 11:16:22 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e pollinate: To re-seed this system again, use the -r|--reseed option
Aug 11 11:16:22 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e acpid: starting up with netlink and the input layer
Aug 11 11:16:22 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e acpid: 1 rule loaded
Aug 11 11:16:22 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e acpid: waiting for events: event logging is off
Aug 11 11:16:22 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 11 11:16:22 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e pollinate: To re-seed this system again, use the -r|--reseed option
Aug 11 11:16:22 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e haveged: haveged starting up
Aug 11 11:16:22 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   13.870563] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 11 11:16:45 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpdate[1785]: adjust time server 169.254.169.254 offset 0.000896 sec
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpd[1819]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpd[1822]: proto: precision = 0.163 usec
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpd[1822]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpd[1822]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpd[1822]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpd[1822]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpd[1822]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpd[1822]: Listen normally on 3 eth0 10.20.0.18 UDP 123
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpd[1822]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpd[1822]: peers refreshed
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpd[1822]: Listening on routing socket on fd #21 for interface updates
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   44.054249] init: plymouth-upstart-bridge main process ended, respawning
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e startup-script: INFO Found startup-script in metadata.
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e startup-script: INFO startup-script: job 1 at Sat Aug 11 14:26:00 2018
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e startup-script: INFO startup-script: Return code 0.
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e startup-script: INFO startup-script: Return code 0.
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e startup-script: INFO Finished running startup scripts.
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ec2: 
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ec2: #############################################################
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ec2: 1024 f1:97:06:d6:a6:62:05:e2:73:32:18:9f:eb:52:35:98  root@travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e (DSA)
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ec2: 256 ce:0e:78:5c:4f:c2:b8:d0:b2:65:f2:6a:2a:4f:b1:ea  root@travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e (ECDSA)
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ec2: 256 04:67:39:5f:98:bf:ce:3f:99:02:1e:c3:01:a8:ae:ba  root@travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e (ED25519)
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ec2: 2048 30:11:30:1e:4b:0c:f2:75:91:45:f5:d2:34:b7:54:95  root@travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e (RSA)
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 11 11:16:52 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ec2: #############################################################
Aug 11 11:17:01 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e CRON[2952]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 11 11:17:24 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [   76.290040] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 11 11:18:32 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [  143.962029] device vethcb9bd2b entered promiscuous mode
Aug 11 11:18:32 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [  144.058880] cgroup: docker-runc (4809) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 11 11:18:32 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [  144.058884] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 11 11:18:32 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [  144.121960] eth0: renamed from veth83e7d2d
Aug 11 11:18:32 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [  144.162015] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 11 11:18:32 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [  144.163149] docker0: port 1(vethcb9bd2b) entered forwarding state
Aug 11 11:18:32 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [  144.163168] docker0: port 1(vethcb9bd2b) entered forwarding state
Aug 11 11:18:32 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [  144.163188] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 11 11:18:35 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpd[1822]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 11 11:18:35 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpd[1822]: Listen normally on 6 docker0 fe80::42:9aff:feba:dbdb UDP 123
Aug 11 11:18:35 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpd[1822]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 11 11:18:35 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpd[1822]: peers refreshed
Aug 11 11:18:35 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e ntpd[1822]: new interface(s) found: waking up resolver
Aug 11 11:18:47 travis-job-9e95818e-05b7-40f4-a4b3-3f4e52d4d52e kernel: [  159.181627] docker0: port 1(vethcb9bd2b) entered forwarding state
---
travis_time:end:0ba8cce8:start=1533986489540752941,finish=1533986489546783358,duration=6030417
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2835e724
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ae00820
travis_time:start:1ae00820
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:15244e4d
$ dmesg | grep -i kill
