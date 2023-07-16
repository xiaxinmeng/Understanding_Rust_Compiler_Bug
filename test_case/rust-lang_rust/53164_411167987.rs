plain

[00:03:48] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:49] tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:295: line longer than 100 chars
[00:03:49] tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:296: line longer than 100 chars
[00:03:50] some tidy checks failed
[00:03:50] 
[00:03:50] 
[00:03:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:50] 
[00:03:50] 
[00:03:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:50] Build completed unsuccessfully in 0:00:48
[00:03:50] Build completed unsuccessfully in 0:00:48
[00:03:50] Makefile:79: recipe for target 'tidy' failed
[00:03:50] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:130be14a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:01588b64
$ sudo tail -n 500 /var/log/syslog
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   00000-9FFFF write-back
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   A0000-BFFFF uncachable
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   C0000-FFFFF write-protect
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] MTRR variable ranges enabled:
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   0 base 0000C0000000 mask 3FFFC0000000 uncachable
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   1 disabled
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   2 disabled
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   3 disabled
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   4 disabled
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   5 disabled
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   6 disabled
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   7 disabled
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Using GB pages for direct mapping
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] kvm-clock: using sched offset of 1376947879 cycles
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Zone ranges:
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   Device   empty
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Movable zone start for each node
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Early memory node ranges
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Policy zone: Normal
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] console [ttyS0] enabled
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.319084] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.320375] pid_max: default: 32768 minimum: 301
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.321146] ACPI: Core revision 20150930
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.328029] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.329116] Security Framework initialized
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.329683] Yama: becoming mindful.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.330249] AppArmor: AppArmor disabled by boot time parameter
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.332874] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.342413] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.346846] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.347856] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.349108] Initializing cgroup subsys io
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.349711] Initializing cgroup subsys memory
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.350502] Initializing cgroup subsys devices
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.351558] Initializing cgroup subsys freezer
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.352432] Initializing cgroup subsys net_cls
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.353065] Initializing cgroup subsys perf_event
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.353739] Initializing cgroup subsys net_prio
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.354526] Initializing cgroup subsys hugetlb
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.355196] Initializing cgroup subsys pids
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.355899] CPU: Physical Processor ID: 0
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.356560] CPU: Processor Core ID: 0
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.358431] mce: CPU supports 32 MCE banks
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.359330] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.360251] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.363146] Freeing SMP alternatives memory: 32K
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.371915] ftrace: allocating 32185 entries in 126 pages
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.421670] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.423487] smpboot: Max logical packages: 2
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.424640] x2apic enabled
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.426298] Switched APIC routing to physical x2apic.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.430184] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.538299] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.540009] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.543161] x86: Booting SMP configuration:
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.544139] .... node  #0, CPUs:      #1
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.545056] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.549528]  #2
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.549998] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.555374]  #3
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.555975] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.560248] x86: Booted up 1 node, 4 CPUs
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.560922] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.563267] devtmpfs: initialized
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.567953] evm: security.selinux
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.568563] evm: security.SMACK64
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.569067] evm: security.SMACK64EXEC
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.569592] evm: security.SMACK64TRANSMUTE
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.570488] evm: security.SMACK64MMAP
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.571170] evm: security.ima
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.571647] evm: security.capability
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.572679] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.574127] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.575343] pinctrl core: initialized pinctrl subsystem
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.576443] RTC time: 18:52:34, date: 08/07/18
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.578080] NET: Registered protocol family 16
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.590326] cpuidle: using governor ladder
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.602324] cpuidle: using governor menu
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.603109] PCCT header not found.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.603776] ACPI: bus type PCI registered
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.604332] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.605352] PCI: Using configuration type 1 for base access
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.619307] ACPI: Added _OSI(Module Device)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.619991] ACPI: Added _OSI(Processor Device)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.620603] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.621309] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.624738] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.648802] ACPI: Interpreter enabled
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.649546] ACPI: (supports S0 S3 S4 S5)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.650446] ACPI: Using IOAPIC for interrupt routing
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.651207] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.680684] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.681589] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.682519] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.683791] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.686202] PCI host bridge to bus 0000:00
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.686869] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.687977] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.689058] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.690076] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.691148] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.691947] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.692393] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.710057] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.727184] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.728752] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.734650] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.739391] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.753588] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.759100] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.762943] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.777487] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.780083] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.782286] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.784525] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.786950] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.807831] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.809414] vgaarb: loaded
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.810088] SCSI subsystem initialized
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.810786] libata version 3.00 loaded.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.810818] ACPI: bus type USB registered
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.811449] usbcore: registered new interface driver usbfs
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.812319] usbcore: registered new interface driver hub
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.813174] usbcore: registered new device driver usb
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.814099] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.815382] dmi: Firmware registration failed.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.816238] PCI: Using ACPI for IRQ routing
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.816871] PCI: pci_cache_line_size set to 64 bytes
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.816979] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.816980] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.817106] NetLabel: Initializing
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.817817] NetLabel:  domain hash size = 128
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.818537] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.819346] NetLabel:  unlabeled traffic allowed by default
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.820339] amd_nb: Cannot enumerate AMD northbridges
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.821307] clocksource: Switched to clocksource kvm-clock
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.829112] pnp: PnP ACPI init
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.829775] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.829838] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.829880] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.829927] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.829965] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.830002] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.830040] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.830203] pnp: PnP ACPI: found 7 devices
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.837888] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.839397] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.839400] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.839402] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.839403] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.839452] NET: Registered protocol family 2
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.840450] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.842729] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.843796] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.844746] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.845811] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.847701] NET: Registered protocol family 1
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.848359] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.849367] PCI: CLS 0 bytes, default 64
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    0.849442] Unpacking initramfs...
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    2.986669] Freeing initrd memory: 21432K
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    2.987489] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    2.988564] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    2.990538] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    2.992086] hw unit of domain pp0-core 2^-0 Joules
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    2.993093] hw unit of domain package 2^-0 Joules
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    2.994010] hw unit of domain dram 2^-0 Joules
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    2.994826] Scanning for low memory corruption every 60 seconds
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    2.996895] audit: initializing netlink subsys (disabled)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    2.997796] audit: type=2000 audit(1533667956.750:1): initialized
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    2.999319] Initialise system trusted keyring
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.000405] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.001564] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.003819] zbud: loaded
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.004911] VFS: Disk quotas dquot_6.6.0
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.005600] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.006918] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.008399] fuse init (API version 7.23)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.009242] Key type big_key registered
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.009975] Allocating IMA MOK and blacklist keyrings.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.012050] Key type asymmetric registered
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.012839] Asymmetric key parser 'x509' registered
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.013853] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.015338] io scheduler noop registered
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.016129] io scheduler deadline registered (default)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.017120] io scheduler cfq registered
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.018301] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.019181] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.020422] intel_idle: does not run on family 6 model 62
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.020548] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.021999] ACPI: Power Button [PWRF]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.022584] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.024281] ACPI: Sleep Button [SLPF]
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.025407] GHES: HEST is not enabled!
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.028009] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.029110] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.034233] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.035287] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.040521] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.063038] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.086087] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.109512] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.132811] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.136159] Linux agpgart interface v0.103
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.139931] loop: module loaded
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.140762] libphy: Fixed MDIO Bus: probed
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.141618] tun: Universal TUN/TAP device driver, 1.6
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.142622] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.178050] PPP generic driver version 2.4.2
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.179206] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.180489] ehci-pci: EHCI PCI platform driver
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.181281] ehci-platform: EHCI generic platform driver
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.182184] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.183186] ohci-pci: OHCI PCI platform driver
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.183900] ohci-platform: OHCI generic platform driver
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.185075] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.186386] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.188851] i8042: Warning: Keylock active
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.190779] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.191584] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.192827] mousedev: PS/2 mouse device common for all mice
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.194331] rtc_cmos 00:00: RTC can wake from S4
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.195641] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.197387] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.198695] i2c /dev entries driver
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.199499] device-mapper: uevent: version 1.0.3
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.200592] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.202265] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.204316] NET: Registered protocol family 10
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.205864] NET: Registered protocol family 17
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.206779] Key type dns_resolver registered
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.208163] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.209915] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.211076] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.212065] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.213428] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.215638] registered taskstats version 1
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.216429] Loading compiled-in X.509 certificates
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.218147] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.219989] zswap: loaded using pool lzo/zbud
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.223083] Key type trusted registered
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.227186] Key type encrypted registered
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.227993] ima: No TPM chip found, activating TPM-bypass!
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.229255] evm: HMAC attrs: 0x1
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.230254]   Magic number: 14:785:897
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.231282] rtc_cmos 00:00: setting system clock to 2018-08-07 18:52:37 UTC (1533667957)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.233522] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.234793] EDD information not available.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.235886] PM: Hibernation image not present or could not be loaded.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.237558] Freeing unused kernel memory: 1496K
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.238229] Write protecting the kernel read-only data: 14336k
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.240260] Freeing unused kernel memory: 1956K
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.241522] Freeing unused kernel memory: 92K
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.256779] systemd-udevd[118]: starting version 204
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.318778] scsi host0: Virtio SCSI HBA
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.328153] AVX version of gcm_enc/dec engaged.
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.329004] AES CTR mode by8 optimization enabled
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.329586] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.364428] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.364431] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.366779] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.368215] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.368955] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.369128] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.371972]  sda: sda1
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.373260] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.393918] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.993398] tsc: Refined TSC clocksource calibration: 2499.788 MHz
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    3.994625] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24087146624, max_idle_ns: 440795223400 ns
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    4.231360] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    6.325539] floppy0: no floppy controllers found
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    7.513338] raid6: sse2x1   gen()  9115 MB/s
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    7.581340] raid6: sse2x1   xor()  7048 MB/s
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    7.649337] raid6: sse2x2   gen() 11470 MB/s
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    7.717338] raid6: sse2x2   xor()  8045 MB/s
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    7.785338] raid6: sse2x4   gen() 12564 MB/s
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    7.853342] raid6: sse2x4   xor()  9034 MB/s
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    7.854078] raid6: using algorithm sse2x4 gen() 12564 MB/s
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    7.854964] raid6: .... xor() 9034 MB/s, rmw enabled
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    7.855712] raid6: using ssse3x2 recovery algorithm
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    7.857970] xor: automatically using best checksumming function:
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    7.897334]    avx       : 22156.000 MB/sec
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    7.912230] Btrfs loaded
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    7.952521] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    7.953701] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    8.019857] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    8.031861] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    8.032823] EXT4-fs (sda1): recovery complete
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    8.037420] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    8.225254] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    8.338786] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    8.385018] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    8.565990] random: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    9.079683] random: cloud-init: uninitialized urandom read (32 bytes read, 45 bits of entropy available)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    9.210077] systemd-udevd[701]: starting version 204
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    9.315706] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    9.398554] ppdev: user-space parallel port driver
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    9.530559] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    9.581217] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    9.646661] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [    9.808896] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [   10.060697] random: mktemp: uninitialized urandom read (12 bytes read, 59 bits of entropy available)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [   10.130682] random: mktemp: uninitialized urandom read (6 bytes read, 60 bits of entropy available)
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [   10.199280] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [   10.228085] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [   10.622710] init: failsafe main process (1093) killed by TERM signal
Aug  7 18:52:44 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO Running set_multiqueue.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO Set channels for eth0 to 4.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 google-accounts: INFO Starting Google Accounts daemon.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 google-accounts: INFO Creating a new user account for me.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [   11.377422] random: nonblocking pool is initialized
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 google-accounts: INFO Created user account me.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 google-accounts: INFO Removing user packer.
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 cron[1402]: (CRON) INFO (pidfile fd = 3)
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 cron[1437]: (CRON) STARTUP (fork ok)
Aug  7 18:52:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 cron[1437]: (CRON) INFO (Running @reboot jobs)
Aug  7 18:52:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 acpid: starting up with netlink and the input layer
Aug  7 18:52:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 acpid: 1 rule loaded
Aug  7 18:52:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 acpid: waiting for events: event logging is off
Aug  7 18:52:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 haveged: haveged starting up
Aug  7 18:52:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 18:52:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 18:52:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [   11.839966] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 18:52:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [   11.849050] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 18:52:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 18:52:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [   12.014246] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 18:52:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [   12.017357] Bridge firewalling registered
Aug  7 18:52:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [   12.025361] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 18:52:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [   12.090211] Initializing XFRM netlink socket
Aug  7 18:52:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [   12.097930] Netfilter messages via NETLINK v0.30.
Aug  7 18:52:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [   12.100075] ctnetlink v0.93: registering with nfnetlink.
Aug  7 18:52:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [   12.381444] floppy0: no floppy controllers found
Aug  7 18:53:09 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpdate[1730]: adjust time server 169.254.169.254 offset 0.002826 sec
Aug  7 18:53:15 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpd[1741]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 18:53:15 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpd[1742]: proto: precision = 0.103 usec
Aug  7 18:53:15 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpd[1742]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 18:53:15 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpd[1742]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 18:53:15 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpd[1742]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 18:53:15 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpd[1742]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  7 18:53:15 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpd[1742]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 18:53:15 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpd[1742]: Listen normally on 3 eth0 10.20.0.15 UDP 123
Aug  7 18:53:15 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpd[1742]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 18:53:15 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpd[1742]: peers refreshed
Aug  7 18:53:15 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpd[1742]: Listening on routing socket on fd #21 for interface updates
Aug  7 18:53:16 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [   42.003804] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 18:53:16 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 startup-script: INFO Found startup-script in metadata.
Aug  7 18:53:16 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 18:53:16 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 startup-script: INFO startup-script: job 1 at Tue Aug  7 22:03:00 2018
Aug  7 18:53:16 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 startup-script: INFO startup-script: Return code 0.
Aug  7 18:53:16 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 startup-script: INFO startup-script: Return code 0.
Aug  7 18:53:16 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 startup-script: INFO Finished running startup scripts.
Aug  7 18:53:16 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ec2: 
Aug  7 18:53:16 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ec2: #############################################################
Aug  7 18:53:16 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 18:53:16 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ec2: 1024 f7:04:71:c0:8f:7f:4e:7b:02:10:ce:9b:13:e0:f7:40  root@travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 (DSA)
Aug  7 18:53:16 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ec2: 256 b2:23:80:6b:52:59:df:0a:bc:34:0c:20:f7:6d:75:89  root@travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 (ECDSA)
Aug  7 18:53:16 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ec2: 256 00:4e:cd:3b:df:e2:43:a1:64:07:51:b8:ab:63:07:0b  root@travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 (ED25519)
Aug  7 18:53:16 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ec2: 2048 81:a7:31:fa:7c:33:5b:cd:8d:26:a7:7b:41:a4:03:e5  root@travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 (RSA)
Aug  7 18:53:16 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 18:53:16 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ec2: #############################################################
Aug  7 18:54:39 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [  125.803445] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 18:55:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [  191.907307] device veth40bb4b5 entered promiscuous mode
Aug  7 18:55:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [  191.907383] docker0: port 1(veth40bb4b5) entered forwarding state
Aug  7 18:55:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [  191.907393] docker0: port 1(veth40bb4b5) entered forwarding state
Aug  7 18:55:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [  191.907857] docker0: port 1(veth40bb4b5) entered disabled state
Aug  7 18:55:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [  191.986681] cgroup: docker-runc (4730) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 18:55:45 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [  191.986684] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 18:55:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [  192.065193] eth0: renamed from vethb440459
Aug  7 18:55:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [  192.100513] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 18:55:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [  192.101626] docker0: port 1(veth40bb4b5) entered forwarding state
Aug  7 18:55:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [  192.101642] docker0: port 1(veth40bb4b5) entered forwarding state
Aug  7 18:55:46 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [  192.101668] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 18:55:49 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpd[1742]: Listen normally on 5 docker0 fe80::42:83ff:fe51:396c UDP 123
Aug  7 18:55:49 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpd[1742]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  7 18:55:49 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpd[1742]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 18:55:49 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpd[1742]: peers refreshed
Aug  7 18:55:49 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 ntpd[1742]: new interface(s) found: waking up resolver
Aug  7 18:56:01 travis-job-8817a4ff-c843-45c0-ae27-f200a20347e5 kernel: [  207.138009] docker0: port 1(veth40bb4b5) entered forwarding state
---
travis_time:end:275fcfd6:start=1533668311650191945,finish=1533668311656940586,duration=6748641
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f9d8753
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:152aa031
travis_time:start:152aa031
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:068fe548
$ dmesg | grep -i kill
