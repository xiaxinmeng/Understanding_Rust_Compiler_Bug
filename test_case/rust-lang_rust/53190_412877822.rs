plain

[00:04:05] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:05] tidy error: /checkout/src/test/run-make/git_clone_sha1.sh:9: line longer than 100 chars
[00:04:05] tidy error: /checkout/src/test/run-make/git_clone_sha1.sh: incorrect license
[00:04:06] some tidy checks failed
[00:04:06] 
[00:04:06] 
[00:04:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:06] 
[00:04:06] 
[00:04:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:06] Build completed unsuccessfully in 0:00:55
[00:04:06] Build completed unsuccessfully in 0:00:55
[00:04:06] make: *** [tidy] Error 1
[00:04:06] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:219790e0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0792e98a
$ sudo tail -n 500 /var/log/syslog
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   5 disabled
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   6 disabled
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   7 disabled
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Using GB pages for direct mapping
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] kvm-clock: using sched offset of 1752923472 cycles
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Zone ranges:
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   Device   empty
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Movable zone start for each node
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Early memory node ranges
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Policy zone: Normal
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] console [ttyS0] enabled
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.669061] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.675163] pid_max: default: 32768 minimum: 301
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.678250] ACPI: Core revision 20150930
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.687180] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.693068] Security Framework initialized
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.697092] Yama: becoming mindful.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.699554] AppArmor: AppArmor disabled by boot time parameter
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.705069] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.718044] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.725518] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.730959] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.736220] Initializing cgroup subsys io
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.738586] Initializing cgroup subsys memory
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.741379] Initializing cgroup subsys devices
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.744128] Initializing cgroup subsys freezer
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.747628] Initializing cgroup subsys net_cls
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.750878] Initializing cgroup subsys perf_event
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.754167] Initializing cgroup subsys net_prio
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.757538] Initializing cgroup subsys hugetlb
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.760183] Initializing cgroup subsys pids
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.762835] CPU: Physical Processor ID: 0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.765241] CPU: Processor Core ID: 0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.767521] mce: CPU supports 32 MCE banks
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.770344] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.774548] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.780622] Freeing SMP alternatives memory: 32K
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.793192] ftrace: allocating 32185 entries in 126 pages
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.850575] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.855370] smpboot: Max logical packages: 2
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.858696] x2apic enabled
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.861774] Switched APIC routing to physical x2apic.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.867752] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.977777] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.983075] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.989329] x86: Booting SMP configuration:
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.991721] .... node  #0, CPUs:      #1
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    0.994715] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.001780]  #2
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.003115] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.009328]  #3
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.010939] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.018267] x86: Booted up 1 node, 4 CPUs
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.020927] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.027119] devtmpfs: initialized
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.033124] evm: security.selinux
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.035848] evm: security.SMACK64
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.037523] evm: security.SMACK64EXEC
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.040050] evm: security.SMACK64TRANSMUTE
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.042787] evm: security.SMACK64MMAP
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.045427] evm: security.ima
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.047376] evm: security.capability
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.050370] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.055972] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.061193] pinctrl core: initialized pinctrl subsystem
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.065035] RTC time: 13:41:52, date: 08/14/18
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.069270] NET: Registered protocol family 16
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.081855] cpuidle: using governor ladder
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.093828] cpuidle: using governor menu
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.096590] PCCT header not found.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.098776] ACPI: bus type PCI registered
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.101699] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.106677] PCI: Using configuration type 1 for base access
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.123904] ACPI: Added _OSI(Module Device)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.126768] ACPI: Added _OSI(Processor Device)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.130647] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.133526] ACPI: Added _OSI(Processor Aggregator Device)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.140091] ACPI: Executed 2 blocks of module-level executable AML code
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.169648] ACPI: Interpreter enabled
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.173245] ACPI: (supports S0 S3 S4 S5)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.175771] ACPI: Using IOAPIC for interrupt routing
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.179468] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.215602] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.220039] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.225205] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.230640] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.239841] PCI host bridge to bus 0000:00
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.242943] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.247859] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.251708] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.256268] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.260779] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.264555] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.265001] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.293529] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.322977] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.332703] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.343639] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.352951] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.375488] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.385290] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.394382] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.419070] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.425661] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.433078] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.440755] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.448633] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.473359] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.477821] vgaarb: loaded
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.480968] SCSI subsystem initialized
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.484437] libata version 3.00 loaded.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.484467] ACPI: bus type USB registered
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.488930] usbcore: registered new interface driver usbfs
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.493688] usbcore: registered new interface driver hub
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.497160] usbcore: registered new device driver usb
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.502239] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.507592] dmi: Firmware registration failed.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.510018] PCI: Using ACPI for IRQ routing
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.513283] PCI: pci_cache_line_size set to 64 bytes
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.513385] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.513387] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.513547] NetLabel: Initializing
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.515930] NetLabel:  domain hash size = 128
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.518888] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.521973] NetLabel:  unlabeled traffic allowed by default
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.525678] amd_nb: Cannot enumerate AMD northbridges
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.529189] clocksource: Switched to clocksource kvm-clock
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.539755] pnp: PnP ACPI init
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.542107] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.542180] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.542226] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.542281] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.542325] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.542370] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.542447] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.542622] pnp: PnP ACPI: found 7 devices
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.553503] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.563333] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.563336] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.563338] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.563340] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.563375] NET: Registered protocol family 2
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.566344] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.573333] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.577969] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.582687] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.586726] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.591501] NET: Registered protocol family 1
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.594603] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.599004] PCI: CLS 0 bytes, default 64
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    1.599064] Unpacking initramfs...
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.702166] Freeing initrd memory: 21432K
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.704773] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.709097] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.716337] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.722265] hw unit of domain pp0-core 2^-0 Joules
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.725957] hw unit of domain package 2^-0 Joules
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.729153] hw unit of domain dram 2^-0 Joules
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.731561] Scanning for low memory corruption every 60 seconds
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.735944] audit: initializing netlink subsys (disabled)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.738557] audit: type=2000 audit(1534254114.445:1): initialized
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.742165] Initialise system trusted keyring
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.745494] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.750431] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.755458] zbud: loaded
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.757969] VFS: Disk quotas dquot_6.6.0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.760033] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.765571] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.770044] fuse init (API version 7.23)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.774102] Key type big_key registered
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.777376] Allocating IMA MOK and blacklist keyrings.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.785271] Key type asymmetric registered
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.788600] Asymmetric key parser 'x509' registered
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.791864] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.797935] io scheduler noop registered
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.800221] io scheduler deadline registered (default)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.803246] io scheduler cfq registered
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.806459] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.810022] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.813552] intel_idle: does not run on family 6 model 45
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.813663] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.818042] ACPI: Power Button [PWRF]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.820729] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.825045] ACPI: Sleep Button [SLPF]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.828507] GHES: HEST is not enabled!
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.833254] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.836854] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.847453] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.851066] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.862700] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.889341] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.917666] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.943778] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.971749] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.979904] Linux agpgart interface v0.103
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.988065] loop: module loaded
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.992314] libphy: Fixed MDIO Bus: probed
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    3.995732] tun: Universal TUN/TAP device driver, 1.6
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.000807] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.053814] PPP generic driver version 2.4.2
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.057278] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.061244] ehci-pci: EHCI PCI platform driver
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.064896] ehci-platform: EHCI generic platform driver
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.068258] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.073779] ohci-pci: OHCI PCI platform driver
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.076353] ohci-platform: OHCI generic platform driver
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.079723] uhci_hcd: USB Universal Host Controller Interface driver
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.083742] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.088904] i8042: Warning: Keylock active
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.093319] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.096840] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.100494] mousedev: PS/2 mouse device common for all mice
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.105150] rtc_cmos 00:00: RTC can wake from S4
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.108746] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.113356] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.116792] i2c /dev entries driver
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.119116] device-mapper: uevent: version 1.0.3
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.123292] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.132139] ledtrig-cpu: registered to indicate activity on CPUs
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.138255] NET: Registered protocol family 10
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.142151] NET: Registered protocol family 17
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.145509] Key type dns_resolver registered
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.149117] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.154249] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.158172] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.161973] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.166723] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.175328] registered taskstats version 1
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.177651] Loading compiled-in X.509 certificates
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.181766] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.187759] zswap: loaded using pool lzo/zbud
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.194017] Key type trusted registered
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.201748] Key type encrypted registered
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.206096] ima: No TPM chip found, activating TPM-bypass!
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.208938] evm: HMAC attrs: 0x1
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.212501]   Magic number: 14:574:685
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.214912]  clocksource: hash matches
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.218025] rtc_cmos 00:00: setting system clock to 2018-08-14 13:41:55 UTC (1534254115)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.222998] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.226785] EDD information not available.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.229967] PM: Hibernation image not present or could not be loaded.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.231701] Freeing unused kernel memory: 1496K
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.234791] Write protecting the kernel read-only data: 14336k
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.240263] Freeing unused kernel memory: 1956K
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.243810] Freeing unused kernel memory: 92K
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.263712] systemd-udevd[119]: starting version 204
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.307518] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.327607] scsi host0: Virtio SCSI HBA
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.342286] AVX version of gcm_enc/dec engaged.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.342912] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.351488] AES CTR mode by8 optimization enabled
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.411114] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.415123] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.420127] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.424028] sd 0:0:1:0: [sda] Write Protect is off
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.427132] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.427329] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.436816]  sda: sda1
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.439903] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.729366] tsc: Refined TSC clocksource calibration: 2599.999 MHz
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    4.733738] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3b2ad7e, max_idle_ns: 440795282337 ns
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    5.163547] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    7.329482] floppy0: no floppy controllers found
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    8.501258] raid6: sse2x1   gen()  8694 MB/s
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    8.569261] raid6: sse2x1   xor()  6448 MB/s
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    8.637246] raid6: sse2x2   gen() 10698 MB/s
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    8.705254] raid6: sse2x2   xor()  7074 MB/s
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    8.773252] raid6: sse2x4   gen() 12444 MB/s
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    8.845270] raid6: sse2x4   xor()  8341 MB/s
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    8.848444] raid6: using algorithm sse2x4 gen() 12444 MB/s
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    8.852764] raid6: .... xor() 8341 MB/s, rmw enabled
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    8.856596] raid6: using ssse3x2 recovery algorithm
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    8.861802] xor: automatically using best checksumming function:
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    8.901205]    avx       : 26710.000 MB/sec
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    8.918396] Btrfs loaded
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    8.993246] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    8.998675] EXT4-fs (sda1): write access will be enabled during recovery
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    9.113229] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    9.127860] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    9.131519] EXT4-fs (sda1): recovery complete
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    9.142087] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    9.471014] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    9.651737] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [    9.722649] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   10.004909] random: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   10.759121] random: cloud-init: uninitialized urandom read (32 bytes read, 44 bits of entropy available)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   10.915672] systemd-udevd[701]: starting version 204
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   11.071301] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   11.120384] intel_rapl: no valid rapl domains found in package 0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   11.166059] intel_rapl: no valid rapl domains found in package 0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   11.181708] ppdev: user-space parallel port driver
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   11.306490] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   11.358370] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   11.413894] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   11.573317] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   11.846667] random: mktemp: uninitialized urandom read (12 bytes read, 59 bits of entropy available)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   11.924942] random: mktemp: uninitialized urandom read (6 bytes read, 60 bits of entropy available)
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   12.003664] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   12.056093] EXT4-fs (sda1): resized filesystem to 7864064
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   12.757130] init: failsafe main process (1092) killed by TERM signal
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO Running set_multiqueue.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO Set channels for eth0 to 4.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   13.505059] random: nonblocking pool is initialized
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 google-accounts: INFO Starting Google Accounts daemon.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 14 13:42:04 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 google-accounts: INFO Creating a new user account for me.
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 google-accounts: INFO Created user account me.
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 google-accounts: INFO Created user account me.
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 google-accounts: INFO Creating a new user account for bogdana.
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 google-accounts: INFO Created user account bogdana.
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 google-accounts: INFO Creating a new user account for aj.
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 google-accounts: INFO Created user account aj.
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 google-accounts: INFO Creating a new user account for asari.
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 google-accounts: INFO Created user account asari.
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 google-accounts: INFO Removing user packer.
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   14.153382] floppy0: no floppy controllers found
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   14.165110] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   14.168285] Bridge firewalling registered
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   14.207345] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   14.245163] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 cron[1440]: (CRON) INFO (pidfile fd = 3)
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 cron[1485]: (CRON) STARTUP (fork ok)
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 cron[1485]: (CRON) INFO (Running @reboot jobs)
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 acpid: starting up with netlink and the input layer
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 acpid: 1 rule loaded
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 acpid: waiting for events: event logging is off
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   14.332766] Initializing XFRM netlink socket
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   14.341647] Netfilter messages via NETLINK v0.30.
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   14.344712] ctnetlink v0.93: registering with nfnetlink.
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 13:42:05 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 haveged: haveged starting up
Aug 14 13:42:06 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   14.733612] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpd[1769]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpd[1770]: proto: precision = 0.102 usec
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpd[1770]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpd[1770]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpd[1770]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpd[1770]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpd[1770]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpd[1770]: Listen normally on 3 eth0 10.20.0.162 UDP 123
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpd[1770]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpd[1770]: peers refreshed
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpd[1770]: Listening on routing socket on fd #21 for interface updates
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [   19.952128] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 startup-script: INFO Found startup-script in metadata.
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 startup-script: INFO startup-script: job 1 at Tue Aug 14 16:52:00 2018
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 startup-script: INFO startup-script: Return code 0.
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 startup-script: INFO startup-script: Return code 0.
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 startup-script: INFO Finished running startup scripts.
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ec2: 
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ec2: #############################################################
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ec2: 1024 17:4b:77:ec:24:e6:06:4d:9c:43:4e:9a:0f:44:ad:d8  root@travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 (DSA)
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ec2: 256 6a:37:e4:a5:e7:b0:19:a2:44:91:b1:63:f6:ca:c9:24  root@travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 (ECDSA)
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ec2: 256 69:b9:3e:b6:61:e2:b9:e3:4b:8d:ea:23:a2:7e:57:3c  root@travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 (ED25519)
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ec2: 2048 72:c7:20:89:3e:68:d4:4b:27:f9:22:5f:78:ff:71:fd  root@travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 (RSA)
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 14 13:42:11 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ec2: #############################################################
Aug 14 13:42:20 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpdate[2167]: the NTP socket is in use, exiting
Aug 14 13:43:58 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [  127.011295] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 14 13:45:07 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [  196.078385] device vethe39a678 entered promiscuous mode
Aug 14 13:45:07 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [  196.078509] docker0: port 1(vethe39a678) entered forwarding state
Aug 14 13:45:07 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [  196.078521] docker0: port 1(vethe39a678) entered forwarding state
Aug 14 13:45:07 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [  196.079309] docker0: port 1(vethe39a678) entered disabled state
Aug 14 13:45:08 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [  196.691560] cgroup: docker-runc (4743) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 13:45:08 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [  196.691564] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 13:45:08 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [  196.767150] eth0: renamed from vethfa86426
Aug 14 13:45:08 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [  196.807031] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 13:45:08 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [  196.808356] docker0: port 1(vethe39a678) entered forwarding state
Aug 14 13:45:08 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [  196.808377] docker0: port 1(vethe39a678) entered forwarding state
Aug 14 13:45:08 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [  196.808406] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 14 13:45:12 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpd[1770]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 14 13:45:12 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpd[1770]: Listen normally on 6 docker0 fe80::42:54ff:fe05:df06 UDP 123
Aug 14 13:45:12 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpd[1770]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 14 13:45:12 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpd[1770]: peers refreshed
Aug 14 13:45:12 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 ntpd[1770]: new interface(s) found: waking up resolver
Aug 14 13:45:23 travis-job-8eb547db-a8a0-416b-8b5e-b97f6a328e23 kernel: [  211.846355] docker0: port 1(vethe39a678) entered forwarding state
---
travis_time:end:06021cff:start=1534254486731592015,finish=1534254486738415604,duration=6823589
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0251dde9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05391cf0
travis_time:start:05391cf0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:00b10808
$ dmesg | grep -i kill
