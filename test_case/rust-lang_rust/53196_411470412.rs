plain
travis_time:start:tidy
tidy check
[00:03:49] * 554 error codes
[00:03:49] * highest error code: E0711
[00:03:50] thread 'main' panicked at 'fs::read_dir(path) failed on /checkout/src/test/compile-fail with No such file or directory (os error 2)', tools/tidy/src/lib.rs:92:18
[00:03:50] 
[00:03:50] 
[00:03:50] 
[00:03:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:50] 
[00:03:50] 
[00:03:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:50] Build completed unsuccessfully in 0:00:50
[00:03:50] Build completed unsuccessfully in 0:00:50
[00:03:50] Makefile:79: recipe for target 'tidy' failed
[00:03:50] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:031f80fc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:146bae38
$ sudo tail -n 500 /var/log/syslog
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   2 disabled
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   3 disabled
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   4 disabled
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   5 disabled
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   6 disabled
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   7 disabled
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Using GB pages for direct mapping
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] kvm-clock: using sched offset of 1413022729 cycles
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Zone ranges:
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   Device   empty
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Movable zone start for each node
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Early memory node ranges
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Policy zone: Normal
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] console [ttyS0] enabled
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.323108] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.324302] pid_max: default: 32768 minimum: 301
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.325025] ACPI: Core revision 20150930
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.331596] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.332736] Security Framework initialized
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.333322] Yama: becoming mindful.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.333818] AppArmor: AppArmor disabled by boot time parameter
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.336281] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.346406] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.351187] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.352183] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.353406] Initializing cgroup subsys io
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.353969] Initializing cgroup subsys memory
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.354751] Initializing cgroup subsys devices
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.355689] Initializing cgroup subsys freezer
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.356320] Initializing cgroup subsys net_cls
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.356944] Initializing cgroup subsys perf_event
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.357604] Initializing cgroup subsys net_prio
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.358332] Initializing cgroup subsys hugetlb
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.359025] Initializing cgroup subsys pids
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.359693] CPU: Physical Processor ID: 0
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.360249] CPU: Processor Core ID: 0
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.362089] mce: CPU supports 32 MCE banks
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.362959] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.363812] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.366645] Freeing SMP alternatives memory: 32K
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.375241] ftrace: allocating 32185 entries in 126 pages
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.423478] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.424540] smpboot: Max logical packages: 2
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.425710] x2apic enabled
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.427217] Switched APIC routing to physical x2apic.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.430711] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.538365] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.539831] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.542880] x86: Booting SMP configuration:
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.543502] .... node  #0, CPUs:      #1
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.544359] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.548931]  #2
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.549410] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.554950]  #3
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.555415] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.559850] x86: Booted up 1 node, 4 CPUs
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.560492] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.563528] devtmpfs: initialized
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.567980] evm: security.selinux
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.568476] evm: security.SMACK64
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.568943] evm: security.SMACK64EXEC
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.569453] evm: security.SMACK64TRANSMUTE
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.570021] evm: security.SMACK64MMAP
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.570549] evm: security.ima
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.571006] evm: security.capability
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.571835] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.573187] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.574268] pinctrl core: initialized pinctrl subsystem
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.575181] RTC time: 16:21:06, date: 08/08/18
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.576610] NET: Registered protocol family 16
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.586387] cpuidle: using governor ladder
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.598386] cpuidle: using governor menu
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.599305] PCCT header not found.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.600030] ACPI: bus type PCI registered
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.600742] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.601906] PCI: Using configuration type 1 for base access
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.615210] ACPI: Added _OSI(Module Device)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.615878] ACPI: Added _OSI(Processor Device)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.616487] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.617139] ACPI: Added _OSI(Processor Aggregator Device)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.620444] ACPI: Executed 2 blocks of module-level executable AML code
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.644045] ACPI: Interpreter enabled
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.644711] ACPI: (supports S0 S3 S4 S5)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.645271] ACPI: Using IOAPIC for interrupt routing
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.645978] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.674833] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.675764] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.676727] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.677646] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.679879] PCI host bridge to bus 0000:00
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.680462] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.681514] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.682461] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.683462] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.684483] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.685266] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.685664] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.699235] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.712623] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.714012] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.718881] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.722453] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.734435] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.739407] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.743650] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.756327] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.758496] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.760363] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.762313] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.764273] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.785273] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.786290] vgaarb: loaded
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.786871] SCSI subsystem initialized
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.787720] libata version 3.00 loaded.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.787749] ACPI: bus type USB registered
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.788395] usbcore: registered new interface driver usbfs
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.789146] usbcore: registered new interface driver hub
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.789902] usbcore: registered new device driver usb
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.790751] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.791690] dmi: Firmware registration failed.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.792530] PCI: Using ACPI for IRQ routing
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.793139] PCI: pci_cache_line_size set to 64 bytes
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.793236] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.793237] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.793346] NetLabel: Initializing
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.793844] NetLabel:  domain hash size = 128
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.794477] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.795165] NetLabel:  unlabeled traffic allowed by default
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.796068] amd_nb: Cannot enumerate AMD northbridges
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.796935] clocksource: Switched to clocksource kvm-clock
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.804414] pnp: PnP ACPI init
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.804994] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.805053] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.805095] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.805141] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.805180] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.805219] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.805256] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.805411] pnp: PnP ACPI: found 7 devices
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.812373] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.813676] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.813678] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.813679] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.813680] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.813707] NET: Registered protocol family 2
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.814502] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.816445] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.817523] TCP: Hash tables configured (established 131072 bind 65536)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.818461] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.819347] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.821123] NET: Registered protocol family 1
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.821785] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.822805] PCI: CLS 0 bytes, default 64
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    0.822853] Unpacking initramfs...
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.888495] Freeing initrd memory: 21432K
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.889325] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.890289] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.891804] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.893182] hw unit of domain pp0-core 2^-0 Joules
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.893856] hw unit of domain package 2^-0 Joules
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.894513] hw unit of domain dram 2^-0 Joules
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.895215] Scanning for low memory corruption every 60 seconds
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.896455] audit: initializing netlink subsys (disabled)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.897396] audit: type=2000 audit(1533745268.363:1): initialized
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.898532] Initialise system trusted keyring
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.899419] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.900357] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.902325] zbud: loaded
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.903019] VFS: Disk quotas dquot_6.6.0
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.903610] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.904755] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.905912] fuse init (API version 7.23)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.906621] Key type big_key registered
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.907203] Allocating IMA MOK and blacklist keyrings.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.908742] Key type asymmetric registered
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.909572] Asymmetric key parser 'x509' registered
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.910329] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.911404] io scheduler noop registered
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.911961] io scheduler deadline registered (default)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.912705] io scheduler cfq registered
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.913394] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.914160] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.915071] intel_idle: does not run on family 6 model 62
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.915168] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.916165] ACPI: Power Button [PWRF]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.916910] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.917920] ACPI: Sleep Button [SLPF]
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.918798] GHES: HEST is not enabled!
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.921449] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.922421] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.926406] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.927379] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.944962] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.967358] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    2.990305] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.013934] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.037583] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.041465] Linux agpgart interface v0.103
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.044667] loop: module loaded
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.045524] libphy: Fixed MDIO Bus: probed
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.046526] tun: Universal TUN/TAP device driver, 1.6
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.047960] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.078466] PPP generic driver version 2.4.2
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.079905] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.081635] ehci-pci: EHCI PCI platform driver
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.082806] ehci-platform: EHCI generic platform driver
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.084357] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.086275] ohci-pci: OHCI PCI platform driver
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.087535] ohci-platform: OHCI generic platform driver
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.088913] uhci_hcd: USB Universal Host Controller Interface driver
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.090680] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.093088] i8042: Warning: Keylock active
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.095187] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.096546] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.098064] mousedev: PS/2 mouse device common for all mice
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.099933] rtc_cmos 00:00: RTC can wake from S4
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.101549] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.103315] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.104985] i2c /dev entries driver
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.105928] device-mapper: uevent: version 1.0.3
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.107278] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.109353] ledtrig-cpu: registered to indicate activity on CPUs
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.111240] NET: Registered protocol family 10
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.112736] NET: Registered protocol family 17
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.113845] Key type dns_resolver registered
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.115279] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.116538] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.117979] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.119359] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.120715] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.123317] registered taskstats version 1
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.124332] Loading compiled-in X.509 certificates
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.126336] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.128819] zswap: loaded using pool lzo/zbud
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.131786] Key type trusted registered
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.136147] Key type encrypted registered
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.137264] ima: No TPM chip found, activating TPM-bypass!
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.138784] evm: HMAC attrs: 0x1
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.139894]   Magic number: 14:133:388
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.140958] thermal cooling_device3: hash matches
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.142328] acpi LNXCPU:e8: hash matches
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.143519] acpi LNXCPU:bb: hash matches
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.144966] rtc_cmos 00:00: setting system clock to 2018-08-08 16:21:08 UTC (1533745268)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.147784] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.149300] EDD information not available.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.150579] PM: Hibernation image not present or could not be loaded.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.151933] Freeing unused kernel memory: 1496K
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.152610] Write protecting the kernel read-only data: 14336k
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.154391] Freeing unused kernel memory: 1956K
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.155422] Freeing unused kernel memory: 92K
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.168395] systemd-udevd[119]: starting version 204
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.219300] scsi host0: Virtio SCSI HBA
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.222752] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.228692] AVX version of gcm_enc/dec engaged.
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.229527] AES CTR mode by8 optimization enabled
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.261700] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.261702] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.261705] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.261869] sd 0:0:1:0: [sda] Write Protect is off
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.261871] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.261912] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.263620]  sda: sda1
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.264474] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.297626] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.893037] tsc: Refined TSC clocksource calibration: 2499.771 MHz
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    3.894960] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2408616cfcd, max_idle_ns: 440795216752 ns
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    4.134856] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    6.213090] floppy0: no floppy controllers found
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.388982] raid6: sse2x1   gen()  8875 MB/s
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.456976] raid6: sse2x1   xor()  6756 MB/s
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.525017] raid6: sse2x2   gen() 11258 MB/s
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.593075] raid6: sse2x2   xor()  7621 MB/s
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.661016] raid6: sse2x4   gen() 12474 MB/s
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.729023] raid6: sse2x4   xor()  8750 MB/s
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.730787] raid6: using algorithm sse2x4 gen() 12474 MB/s
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.732273] raid6: .... xor() 8750 MB/s, rmw enabled
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.733625] raid6: using ssse3x2 recovery algorithm
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.736464] xor: automatically using best checksumming function:
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.776983]    avx       : 21811.000 MB/sec
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.792182] Btrfs loaded
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.836250] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.838451] EXT4-fs (sda1): write access will be enabled during recovery
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.904759] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.917341] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.918810] EXT4-fs (sda1): recovery complete
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    7.925350] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    8.098747] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    8.192496] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    8.233683] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    8.398010] random: cloud-init: uninitialized urandom read (32 bytes read, 34 bits of entropy available)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    8.898003] random: cloud-init: uninitialized urandom read (32 bytes read, 42 bits of entropy available)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    9.029665] systemd-udevd[702]: starting version 204
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    9.114914] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    9.234336] ppdev: user-space parallel port driver
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    9.313523] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    9.358229] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    9.420929] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    9.583048] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    9.817102] random: mktemp: uninitialized urandom read (12 bytes read, 56 bits of entropy available)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    9.886587] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    9.959707] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [    9.987416] EXT4-fs (sda1): resized filesystem to 7864064
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [   10.506941] init: failsafe main process (1094) killed by TERM signal
Aug  8 16:21:15 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO Running set_multiqueue.
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO Set channels for eth0 to 4.
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 google-accounts: INFO Starting Google Accounts daemon.
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 google-accounts: INFO Creating a new user account for me.
Aug  8 16:21:16 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 google-clock-skew: INFO Synced system time with hardware clock.
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 google-accounts: INFO Created user account me.
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 google-accounts: INFO Created user account me.
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 google-accounts: INFO Creating a new user account for bogdana.
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 google-accounts: INFO Created user account bogdana.
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 google-accounts: INFO Creating a new user account for aj.
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 google-accounts: INFO Created user account aj.
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 google-accounts: INFO Creating a new user account for asari.
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 google-accounts: INFO Created user account asari.
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 google-accounts: INFO Removing user packer.
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [   11.525514] random: nonblocking pool is initialized
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [   11.851472] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [   11.855650] Bridge firewalling registered
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [   11.866544] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [   11.896966] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [   11.950479] Initializing XFRM netlink socket
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [   11.957286] Netfilter messages via NETLINK v0.30.
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [   11.959879] ctnetlink v0.93: registering with nfnetlink.
Aug  8 16:21:17 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [   12.205139] floppy0: no floppy controllers found
Aug  8 16:21:18 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 cron[1607]: (CRON) INFO (pidfile fd = 3)
Aug  8 16:21:18 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 cron[1646]: (CRON) STARTUP (fork ok)
Aug  8 16:21:18 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 16:21:18 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 16:21:18 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 cron[1646]: (CRON) INFO (Running @reboot jobs)
Aug  8 16:21:18 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 acpid: starting up with netlink and the input layer
Aug  8 16:21:18 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 acpid: 1 rule loaded
Aug  8 16:21:18 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 acpid: waiting for events: event logging is off
Aug  8 16:21:18 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 16:21:18 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 16:21:18 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 haveged: haveged starting up
Aug  8 16:21:18 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [   12.480150] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 16:21:41 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpdate[1775]: adjust time server 169.254.169.254 offset 0.003061 sec
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpd[1811]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpd[1812]: proto: precision = 0.102 usec
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpd[1812]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpd[1812]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpd[1812]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpd[1812]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpd[1812]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpd[1812]: Listen normally on 3 eth0 10.20.255.19 UDP 123
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpd[1812]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpd[1812]: peers refreshed
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpd[1812]: Listening on routing socket on fd #21 for interface updates
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [   42.632877] init: plymouth-upstart-bridge main process ended, respawning
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 startup-script: INFO Found startup-script in metadata.
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 startup-script: INFO startup-script: job 1 at Wed Aug  8 19:31:00 2018
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 startup-script: INFO startup-script: Return code 0.
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 startup-script: INFO startup-script: Return code 0.
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 startup-script: INFO Finished running startup scripts.
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ec2: 
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ec2: #############################################################
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ec2: 1024 ad:e6:5e:0a:cb:54:f5:66:96:03:b7:f6:5d:1f:c1:b5  root@travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 (DSA)
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ec2: 256 9f:f2:5e:50:89:60:9c:77:00:c7:3b:e1:0c:22:cf:1c  root@travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 (ECDSA)
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ec2: 256 3c:6f:a6:c0:79:32:d9:a0:14:9c:3b:be:78:4a:49:2a  root@travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 (ED25519)
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ec2: 2048 49:d1:e9:8b:a2:73:9a:8c:e5:9e:9d:74:4e:85:a1:60  root@travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 (RSA)
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  8 16:21:48 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ec2: #############################################################
Aug  8 16:22:21 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [   75.607256] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  8 16:23:23 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [  138.040530] device vethb69829a entered promiscuous mode
Aug  8 16:23:23 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [  138.113006] cgroup: docker-runc (4797) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  8 16:23:23 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [  138.113009] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  8 16:23:23 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [  138.180581] eth0: renamed from vetha28aae0
Aug  8 16:23:23 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [  138.222527] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  8 16:23:23 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [  138.223616] docker0: port 1(vethb69829a) entered forwarding state
Aug  8 16:23:23 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [  138.223631] docker0: port 1(vethb69829a) entered forwarding state
Aug  8 16:23:23 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [  138.223653] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  8 16:23:27 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpd[1812]: Listen normally on 5 docker0 fe80::42:62ff:fec2:ece2 UDP 123
Aug  8 16:23:27 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpd[1812]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  8 16:23:27 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpd[1812]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  8 16:23:27 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpd[1812]: peers refreshed
Aug  8 16:23:27 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 ntpd[1812]: new interface(s) found: waking up resolver
Aug  8 16:23:38 travis-job-ae222664-3175-46e5-8296-c3a3421f66f1 kernel: [  153.256433] docker0: port 1(vethb69829a) entered forwarding state
---
travis_time:end:02a70b42:start=1533745572722993876,finish=1533745572729316083,duration=6322207
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0938a80c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a98f230
travis_time:start:0a98f230
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:071b020c
$ dmesg | grep -i kill
