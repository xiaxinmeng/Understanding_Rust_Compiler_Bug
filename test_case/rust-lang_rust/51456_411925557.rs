plain

[00:03:49] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:49] tidy error: /checkout/src/librustc_resolve/lib.rs:1968: line longer than 100 chars
[00:03:51] some tidy checks failed
[00:03:51] 
[00:03:51] 
[00:03:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:51] 
[00:03:51] 
[00:03:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:51] Build completed unsuccessfully in 0:00:48
[00:03:51] Build completed unsuccessfully in 0:00:48
[00:03:51] Makefile:79: recipe for target 'tidy' failed
[00:03:51] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ab830d0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:03848bd2
$ sudo tail -n 500 /var/log/syslog
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   2 disabled
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   3 disabled
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   4 disabled
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   5 disabled
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   6 disabled
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   7 disabled
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Using GB pages for direct mapping
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] kvm-clock: using sched offset of 1718604615 cycles
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Zone ranges:
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   Device   empty
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Movable zone start for each node
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Early memory node ranges
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Policy zone: Normal
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Hierarchical RCU implementation.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] console [ttyS0] enabled
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.327759] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.329572] pid_max: default: 32768 minimum: 301
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.330465] ACPI: Core revision 20150930
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.337616] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.339051] Security Framework initialized
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.339818] Yama: becoming mindful.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.340641] AppArmor: AppArmor disabled by boot time parameter
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.343489] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.353902] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.359012] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.360454] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.362104] Initializing cgroup subsys io
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.363025] Initializing cgroup subsys memory
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.363861] Initializing cgroup subsys devices
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.365010] Initializing cgroup subsys freezer
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.365827] Initializing cgroup subsys net_cls
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.366813] Initializing cgroup subsys perf_event
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.367714] Initializing cgroup subsys net_prio
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.368769] Initializing cgroup subsys hugetlb
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.369640] Initializing cgroup subsys pids
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.370319] CPU: Physical Processor ID: 0
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.370938] CPU: Processor Core ID: 0
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.371476] mce: CPU supports 32 MCE banks
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.372297] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.373302] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.376379] Freeing SMP alternatives memory: 32K
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.385384] ftrace: allocating 32185 entries in 126 pages
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.433906] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.435131] smpboot: Max logical packages: 2
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.436338] x2apic enabled
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.437923] Switched APIC routing to physical x2apic.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.442540] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.550618] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.552384] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.555684] x86: Booting SMP configuration:
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.556488] .... node  #0, CPUs:      #1
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.557333] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.560711]  #2
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.561180] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.565047]  #3
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.565619] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.569075] x86: Booted up 1 node, 4 CPUs
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.569660] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.572126] devtmpfs: initialized
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.576849] evm: security.selinux
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.577593] evm: security.SMACK64
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.578100] evm: security.SMACK64EXEC
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.578620] evm: security.SMACK64TRANSMUTE
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.579328] evm: security.SMACK64MMAP
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.579893] evm: security.ima
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.580483] evm: security.capability
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.581438] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.583444] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.584776] pinctrl core: initialized pinctrl subsystem
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.585855] RTC time: 22:56:28, date: 08/09/18
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.587468] NET: Registered protocol family 16
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.598637] cpuidle: using governor ladder
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.610638] cpuidle: using governor menu
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.611482] PCCT header not found.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.612475] ACPI: bus type PCI registered
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.613166] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.614327] PCI: Using configuration type 1 for base access
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.627835] ACPI: Added _OSI(Module Device)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.628610] ACPI: Added _OSI(Processor Device)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.629406] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.630102] ACPI: Added _OSI(Processor Aggregator Device)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.633976] ACPI: Executed 2 blocks of module-level executable AML code
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.658908] ACPI: Interpreter enabled
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.659928] ACPI: (supports S0 S3 S4 S5)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.660705] ACPI: Using IOAPIC for interrupt routing
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.661732] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.692788] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.694018] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.695465] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.696819] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.699655] PCI host bridge to bus 0000:00
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.700726] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.702296] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.703221] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.704284] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.705474] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.706456] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.706936] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.722313] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.738357] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.739988] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.746416] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.751994] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.768784] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.775947] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.781306] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.797254] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.800046] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.802782] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.805660] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.808534] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.832951] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.834031] vgaarb: loaded
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.834677] SCSI subsystem initialized
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.835416] libata version 3.00 loaded.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.835442] ACPI: bus type USB registered
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.836361] usbcore: registered new interface driver usbfs
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.837341] usbcore: registered new interface driver hub
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.838206] usbcore: registered new device driver usb
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.839180] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.840530] dmi: Firmware registration failed.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.841665] PCI: Using ACPI for IRQ routing
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.842475] PCI: pci_cache_line_size set to 64 bytes
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.842584] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.842588] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.842720] NetLabel: Initializing
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.843425] NetLabel:  domain hash size = 128
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.844337] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.845272] NetLabel:  unlabeled traffic allowed by default
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.846388] amd_nb: Cannot enumerate AMD northbridges
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.847320] clocksource: Switched to clocksource kvm-clock
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.855834] pnp: PnP ACPI init
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.856551] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.856637] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.856695] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.856755] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.856803] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.856848] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.856894] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.857090] pnp: PnP ACPI: found 7 devices
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.865390] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.867242] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.867246] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.867248] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.867250] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.867293] NET: Registered protocol family 2
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.868199] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.870537] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.871714] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.872866] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.873975] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.875366] NET: Registered protocol family 1
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.876099] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.877113] PCI: CLS 0 bytes, default 64
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    0.877976] Unpacking initramfs...
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    2.988188] Freeing initrd memory: 21432K
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    2.989185] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    2.990318] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    2.992243] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    2.993910] hw unit of domain pp0-core 2^-0 Joules
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    2.995051] hw unit of domain package 2^-0 Joules
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    2.996067] hw unit of domain dram 2^-0 Joules
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    2.996878] Scanning for low memory corruption every 60 seconds
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    2.998461] audit: initializing netlink subsys (disabled)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    2.999716] audit: type=2000 audit(1533855390.788:1): initialized
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.001261] Initialise system trusted keyring
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.002271] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.003286] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.005640] zbud: loaded
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.006500] VFS: Disk quotas dquot_6.6.0
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.007350] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.009130] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.011188] fuse init (API version 7.23)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.012313] Key type big_key registered
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.012927] Allocating IMA MOK and blacklist keyrings.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.015611] Key type asymmetric registered
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.016349] Asymmetric key parser 'x509' registered
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.017144] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.018516] io scheduler noop registered
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.019296] io scheduler deadline registered (default)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.020457] io scheduler cfq registered
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.021493] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.022474] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.023676] intel_idle: does not run on family 6 model 45
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.023789] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.025110] ACPI: Power Button [PWRF]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.025813] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.027098] ACPI: Sleep Button [SLPF]
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.028194] GHES: HEST is not enabled!
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.030620] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.032003] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.037024] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.038144] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.043374] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.066189] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.089468] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.113172] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.136535] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.140328] Linux agpgart interface v0.103
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.143824] loop: module loaded
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.144655] libphy: Fixed MDIO Bus: probed
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.145744] tun: Universal TUN/TAP device driver, 1.6
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.147127] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.190953] PPP generic driver version 2.4.2
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.192324] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.194090] ehci-pci: EHCI PCI platform driver
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.195253] ehci-platform: EHCI generic platform driver
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.196959] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.198671] ohci-pci: OHCI PCI platform driver
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.199702] ohci-platform: OHCI generic platform driver
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.201351] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.203209] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.205559] i8042: Warning: Keylock active
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.208137] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.209702] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.211429] mousedev: PS/2 mouse device common for all mice
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.213632] rtc_cmos 00:00: RTC can wake from S4
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.215468] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.217735] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.219988] i2c /dev entries driver
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.221302] device-mapper: uevent: version 1.0.3
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.222805] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.225317] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.227893] NET: Registered protocol family 10
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.229634] NET: Registered protocol family 17
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.230997] Key type dns_resolver registered
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.232723] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.234787] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.236388] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.237697] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.238973] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.241795] registered taskstats version 1
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.243171] Loading compiled-in X.509 certificates
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.245032] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.248005] zswap: loaded using pool lzo/zbud
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.251891] Key type trusted registered
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.257049] Key type encrypted registered
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.258316] ima: No TPM chip found, activating TPM-bypass!
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.260023] evm: HMAC attrs: 0x1
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.261715]   Magic number: 14:912:956
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.262909] virtio-pci 0000:00:04.0: hash matches
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.264398] memory memory9: hash matches
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.265924] rtc_cmos 00:00: setting system clock to 2018-08-09 22:56:31 UTC (1533855391)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.268739] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.270964] EDD information not available.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.272593] PM: Hibernation image not present or could not be loaded.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.274304] Freeing unused kernel memory: 1496K
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.275470] Write protecting the kernel read-only data: 14336k
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.277774] Freeing unused kernel memory: 1956K
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.278734] Freeing unused kernel memory: 92K
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.294039] systemd-udevd[119]: starting version 204
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.354506] scsi host0: Virtio SCSI HBA
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.358423] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.366023] AVX version of gcm_enc/dec engaged.
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.367068] AES CTR mode by8 optimization enabled
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.402542] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.402564] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.402566] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.402725] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.402726] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.402761] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.403892]  sda: sda1
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.404514] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.416088] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.995417] tsc: Refined TSC clocksource calibration: 2599.786 MHz
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    3.997093] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257971e5419, max_idle_ns: 440795308604 ns
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    4.253091] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    6.371585] floppy0: no floppy controllers found
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    7.547334] raid6: sse2x1   gen()  9068 MB/s
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    7.615340] raid6: sse2x1   xor()  6450 MB/s
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    7.683337] raid6: sse2x2   gen() 11272 MB/s
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    7.751334] raid6: sse2x2   xor()  7504 MB/s
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    7.819333] raid6: sse2x4   gen() 12620 MB/s
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    7.887330] raid6: sse2x4   xor()  8799 MB/s
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    7.888509] raid6: using algorithm sse2x4 gen() 12620 MB/s
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    7.889523] raid6: .... xor() 8799 MB/s, rmw enabled
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    7.890633] raid6: using ssse3x2 recovery algorithm
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    7.892853] xor: automatically using best checksumming function:
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    7.931333]    avx       : 22218.000 MB/sec
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    7.946229] Btrfs loaded
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    7.988919] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    7.990299] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    8.046633] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    8.052961] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    8.054047] EXT4-fs (sda1): recovery complete
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    8.058560] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    8.267682] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    8.375084] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    8.423170] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    8.614918] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    9.173158] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    9.301240] systemd-udevd[701]: starting version 204
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    9.416828] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    9.470939] intel_rapl: no valid rapl domains found in package 0
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    9.524900] ppdev: user-space parallel port driver
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    9.616786] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    9.664893] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    9.727074] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [    9.888722] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [   10.133919] random: mktemp: uninitialized urandom read (12 bytes read, 58 bits of entropy available)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [   10.201600] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [   10.280495] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [   10.320714] EXT4-fs (sda1): resized filesystem to 7864064
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [   10.682903] init: failsafe main process (1092) killed by TERM signal
Aug  9 22:56:38 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO Running set_multiqueue.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO Set channels for eth0 to 4.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e google-accounts: INFO Starting Google Accounts daemon.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e google-accounts: INFO Creating a new user account for me.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [   11.435339] random: nonblocking pool is initialized
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e google-accounts: INFO Created user account me.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e google-accounts: INFO Creating a new user account for bogdana.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e google-accounts: INFO Created user account bogdana.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e google-accounts: INFO Creating a new user account for aj.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e google-accounts: INFO Created user account aj.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e google-accounts: INFO Creating a new user account for asari.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e google-accounts: INFO Created user account asari.
Aug  9 22:56:39 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e google-accounts: INFO Removing user packer.
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e cron[1433]: (CRON) INFO (pidfile fd = 3)
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e cron[1468]: (CRON) STARTUP (fork ok)
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e cron[1468]: (CRON) INFO (Running @reboot jobs)
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e acpid: starting up with netlink and the input layer
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e acpid: 1 rule loaded
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e acpid: waiting for events: event logging is off
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e haveged: haveged starting up
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [   11.936858] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [   11.950603] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [   12.085398] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [   12.089500] Bridge firewalling registered
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [   12.101804] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [   12.166484] Initializing XFRM netlink socket
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [   12.174877] Netfilter messages via NETLINK v0.30.
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [   12.178153] ctnetlink v0.93: registering with nfnetlink.
Aug  9 22:56:40 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [   12.475447] floppy0: no floppy controllers found
Aug  9 22:57:03 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpdate[1767]: adjust time server 169.254.169.254 offset 0.016965 sec
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpd[1803]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpd[1804]: proto: precision = 0.101 usec
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpd[1804]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpd[1804]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpd[1804]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpd[1804]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpd[1804]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpd[1804]: Listen normally on 3 eth0 10.20.0.11 UDP 123
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpd[1804]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpd[1804]: peers refreshed
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpd[1804]: Listening on routing socket on fd #21 for interface updates
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [   42.142212] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e startup-script: INFO Found startup-script in metadata.
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e startup-script: INFO startup-script: job 1 at Fri Aug 10 02:07:00 2018
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e startup-script: INFO startup-script: Return code 0.
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e startup-script: INFO startup-script: Return code 0.
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e startup-script: INFO Finished running startup scripts.
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ec2: 
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ec2: #############################################################
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ec2: 1024 85:1b:4b:cc:d5:75:ce:b3:3c:21:b1:25:38:16:75:ed  root@travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e (DSA)
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ec2: 256 38:3e:a6:69:2d:02:9d:31:cc:04:d2:e9:31:e6:2d:59  root@travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e (ECDSA)
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ec2: 256 61:f6:fd:07:49:ed:eb:f1:40:2d:f3:ad:ad:a0:bf:cc  root@travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e (ED25519)
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ec2: 2048 af:1f:aa:42:04:9e:c8:8f:93:88:54:8d:d8:52:8a:be  root@travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e (RSA)
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 22:57:10 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ec2: #############################################################
Aug  9 22:58:30 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [  121.951571] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 22:59:32 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [  184.002523] device veth5f83102 entered promiscuous mode
Aug  9 22:59:32 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [  184.088377] cgroup: docker-runc (4797) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 22:59:32 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [  184.088380] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  9 22:59:32 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [  184.155883] eth0: renamed from vethf0d405d
Aug  9 22:59:32 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [  184.190321] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  9 22:59:32 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [  184.191557] docker0: port 1(veth5f83102) entered forwarding state
Aug  9 22:59:32 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [  184.191642] docker0: port 1(veth5f83102) entered forwarding state
Aug  9 22:59:32 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [  184.191675] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  9 22:59:35 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpd[1804]: Listen normally on 5 docker0 fe80::42:89ff:fe25:8685 UDP 123
Aug  9 22:59:35 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpd[1804]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  9 22:59:35 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpd[1804]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  9 22:59:35 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpd[1804]: peers refreshed
Aug  9 22:59:35 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e ntpd[1804]: new interface(s) found: waking up resolver
Aug  9 22:59:47 travis-job-d32cf03a-4cd1-4317-b240-20035b2f599e kernel: [  199.195002] docker0: port 1(veth5f83102) entered forwarding state
---
travis_time:end:14707326:start=1533855742402939748,finish=1533855742411776158,duration=8836410
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03cd3992
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:025480ac
travis_time:start:025480ac
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:02e1e490
$ dmesg | grep -i kill
