plain
[00:03:46] tidy error: /checkout/src/libstd/sync/mpsc/mod.rs:1259: trailing whitespace
[00:03:47] some tidy checks failed
[00:03:47] 
[00:03:47] 
[00:03:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:47] 
[00:03:47] 
[00:03:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:47] Build completed unsuccessfully in 0:00:50
[00:03:47] Build completed unsuccessfully in 0:00:50
[00:03:47] Makefile:79: recipe for target 'tidy' failed
[00:03:47] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d183452
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:1f874520
$ sudo tail -n 500 /var/log/syslog
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] kvm-clock: using sched offset of 1585325377 cycles
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] Zone ranges:
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000]   Device   empty
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] Movable zone start for each node
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] Early memory node ranges
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] Policy zone: Normal
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] console [ttyS0] enabled
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.352566] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.353850] pid_max: default: 32768 minimum: 301
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.354611] ACPI: Core revision 20150930
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.361255] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.362311] Security Framework initialized
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.363089] Yama: becoming mindful.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.363607] AppArmor: AppArmor disabled by boot time parameter
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.367484] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.377926] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.383081] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.384766] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.386149] Initializing cgroup subsys io
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.387003] Initializing cgroup subsys memory
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.388071] Initializing cgroup subsys devices
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.389323] Initializing cgroup subsys freezer
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.390055] Initializing cgroup subsys net_cls
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.390687] Initializing cgroup subsys perf_event
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.391340] Initializing cgroup subsys net_prio
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.392160] Initializing cgroup subsys hugetlb
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.392830] Initializing cgroup subsys pids
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.393628] CPU: Physical Processor ID: 0
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.394203] CPU: Processor Core ID: 0
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.396218] mce: CPU supports 32 MCE banks
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.397386] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.398257] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.401341] Freeing SMP alternatives memory: 32K
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.411566] ftrace: allocating 32185 entries in 126 pages
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.462325] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.463901] smpboot: Max logical packages: 2
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.465225] x2apic enabled
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.467556] Switched APIC routing to physical x2apic.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.471302] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.579686] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.581595] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.585063] x86: Booting SMP configuration:
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.585695] .... node  #0, CPUs:      #1
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.586581] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.591571]  #2
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.592130] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.597298]  #3
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.597807] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.602378] x86: Booted up 1 node, 4 CPUs
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.603499] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.606566] devtmpfs: initialized
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.611452] evm: security.selinux
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.612562] evm: security.SMACK64
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.613276] evm: security.SMACK64EXEC
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.614142] evm: security.SMACK64TRANSMUTE
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.614927] evm: security.SMACK64MMAP
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.616162] evm: security.ima
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.616943] evm: security.capability
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.618117] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.619864] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.621290] pinctrl core: initialized pinctrl subsystem
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.622640] RTC time: 14:44:44, date: 08/07/18
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.624570] NET: Registered protocol family 16
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.635722] cpuidle: using governor ladder
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.647751] cpuidle: using governor menu
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.648752] PCCT header not found.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.649954] ACPI: bus type PCI registered
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.650831] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.652518] PCI: Using configuration type 1 for base access
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.664974] ACPI: Added _OSI(Module Device)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.665993] ACPI: Added _OSI(Processor Device)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.667276] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.668120] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.671870] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.698386] ACPI: Interpreter enabled
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.699275] ACPI: (supports S0 S3 S4 S5)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.700192] ACPI: Using IOAPIC for interrupt routing
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.701005] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.730761] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.732547] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.734076] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.735354] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.738162] PCI host bridge to bus 0000:00
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.738734] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.739837] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.740924] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.741974] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.743321] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.744286] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.744739] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.763386] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.782186] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.783723] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.791146] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.796200] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.813777] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.820532] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.826391] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.843359] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.846219] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.848937] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.851652] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.854219] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.875262] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.876465] vgaarb: loaded
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.877255] SCSI subsystem initialized
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.878199] libata version 3.00 loaded.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.878227] ACPI: bus type USB registered
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.879058] usbcore: registered new interface driver usbfs
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.879851] usbcore: registered new interface driver hub
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.880681] usbcore: registered new device driver usb
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.881626] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.882718] dmi: Firmware registration failed.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.883556] PCI: Using ACPI for IRQ routing
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.884235] PCI: pci_cache_line_size set to 64 bytes
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.884339] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.884340] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.884472] NetLabel: Initializing
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.884970] NetLabel:  domain hash size = 128
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.885601] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.886317] NetLabel:  unlabeled traffic allowed by default
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.887266] amd_nb: Cannot enumerate AMD northbridges
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.888214] clocksource: Switched to clocksource kvm-clock
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.896716] pnp: PnP ACPI init
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.897409] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.897478] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.897523] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.897573] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.897625] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.897668] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.897708] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.897876] pnp: PnP ACPI: found 7 devices
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.905778] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.907340] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.907343] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.907345] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.907346] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.907382] NET: Registered protocol family 2
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.908378] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.910890] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.912178] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.913268] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.914547] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.916404] NET: Registered protocol family 1
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.917061] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.917987] PCI: CLS 0 bytes, default 64
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    0.918043] Unpacking initramfs...
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.057594] Freeing initrd memory: 21432K
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.059635] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.060745] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.063275] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.065538] hw unit of domain pp0-core 2^-0 Joules
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.066396] hw unit of domain package 2^-0 Joules
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.067150] hw unit of domain dram 2^-0 Joules
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.068741] Scanning for low memory corruption every 60 seconds
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.070705] audit: initializing netlink subsys (disabled)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.071892] audit: type=2000 audit(1533653086.530:1): initialized
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.073727] Initialise system trusted keyring
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.075115] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.076039] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.078172] zbud: loaded
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.078957] VFS: Disk quotas dquot_6.6.0
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.079634] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.080947] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.082533] fuse init (API version 7.23)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.083571] Key type big_key registered
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.084648] Allocating IMA MOK and blacklist keyrings.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.086642] Key type asymmetric registered
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.087408] Asymmetric key parser 'x509' registered
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.088338] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.090013] io scheduler noop registered
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.090644] io scheduler deadline registered (default)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.091546] io scheduler cfq registered
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.093218] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.094185] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.095280] intel_idle: does not run on family 6 model 62
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.095402] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.096637] ACPI: Power Button [PWRF]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.097391] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.098480] ACPI: Sleep Button [SLPF]
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.099456] GHES: HEST is not enabled!
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.102062] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.103077] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.108647] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.109642] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.115281] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.137985] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.161504] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.184863] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.208686] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.213053] Linux agpgart interface v0.103
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.216968] loop: module loaded
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.218645] libphy: Fixed MDIO Bus: probed
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.219896] tun: Universal TUN/TAP device driver, 1.6
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.221407] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.265216] PPP generic driver version 2.4.2
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.266570] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.268547] ehci-pci: EHCI PCI platform driver
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.270077] ehci-platform: EHCI generic platform driver
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.271992] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.274354] ohci-pci: OHCI PCI platform driver
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.275746] ohci-platform: OHCI generic platform driver
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.277206] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.279614] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.283046] i8042: Warning: Keylock active
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.285167] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.286814] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.288660] mousedev: PS/2 mouse device common for all mice
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.290809] rtc_cmos 00:00: RTC can wake from S4
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.292603] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.294517] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.296179] i2c /dev entries driver
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.297447] device-mapper: uevent: version 1.0.3
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.298767] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.301312] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.303795] NET: Registered protocol family 10
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.305361] NET: Registered protocol family 17
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.306615] Key type dns_resolver registered
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.308334] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.309974] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.311709] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.313158] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.314719] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.317906] registered taskstats version 1
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.318949] Loading compiled-in X.509 certificates
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.321112] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.323687] zswap: loaded using pool lzo/zbud
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.326797] Key type trusted registered
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.331271] Key type encrypted registered
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.332583] ima: No TPM chip found, activating TPM-bypass!
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.333898] evm: HMAC attrs: 0x1
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.335289]   Magic number: 14:705:737
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.336446] acpi LNXCPU:66: hash matches
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.337639] rtc_cmos 00:00: setting system clock to 2018-08-07 14:44:47 UTC (1533653087)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.340664] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.342607] EDD information not available.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.344177] PM: Hibernation image not present or could not be loaded.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.345733] Freeing unused kernel memory: 1496K
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.346966] Write protecting the kernel read-only data: 14336k
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.349458] Freeing unused kernel memory: 1956K
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.351125] Freeing unused kernel memory: 92K
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.366506] systemd-udevd[119]: starting version 204
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.429775] scsi host0: Virtio SCSI HBA
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.435950] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.440063] AVX version of gcm_enc/dec engaged.
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.441572] AES CTR mode by8 optimization enabled
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.474739] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.474748] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.476973] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.478125] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.478872] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.479039] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.481879]  sda: sda1
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.483229] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    3.488870] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    4.068348] tsc: Refined TSC clocksource calibration: 2499.766 MHz
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    4.069279] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24085cabc78, max_idle_ns: 440795288552 ns
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    4.326012] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    6.416429] floppy0: no floppy controllers found
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    7.596247] raid6: sse2x1   gen()  9125 MB/s
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    7.664250] raid6: sse2x1   xor()  7080 MB/s
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    7.732262] raid6: sse2x2   gen() 11244 MB/s
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    7.800245] raid6: sse2x2   xor()  7887 MB/s
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    7.868244] raid6: sse2x4   gen() 12935 MB/s
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    7.936259] raid6: sse2x4   xor()  8647 MB/s
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    7.937190] raid6: using algorithm sse2x4 gen() 12935 MB/s
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    7.938025] raid6: .... xor() 8647 MB/s, rmw enabled
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    7.938745] raid6: using ssse3x2 recovery algorithm
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    7.941142] xor: automatically using best checksumming function:
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    7.980258]    avx       : 22050.000 MB/sec
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    7.994457] Btrfs loaded
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    8.038538] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    8.039696] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    8.104555] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    8.117124] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    8.118041] EXT4-fs (sda1): recovery complete
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    8.123274] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    8.330655] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    8.440499] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    8.493691] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    8.684304] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    9.241696] random: cloud-init: uninitialized urandom read (32 bytes read, 44 bits of entropy available)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    9.392925] systemd-udevd[702]: starting version 204
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    9.499905] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    9.623403] ppdev: user-space parallel port driver
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    9.695703] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    9.748082] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    9.806842] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [    9.971220] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [   10.206884] random: mktemp: uninitialized urandom read (12 bytes read, 59 bits of entropy available)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [   10.284296] random: mktemp: uninitialized urandom read (6 bytes read, 60 bits of entropy available)
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [   10.360336] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [   10.400039] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [   10.730162] init: failsafe main process (1093) killed by TERM signal
Aug  7 14:44:54 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO Running set_multiqueue.
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO Set channels for eth0 to 4.
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [   11.484460] random: nonblocking pool is initialized
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Starting Google Accounts daemon.
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Creating a new user account for me.
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 cron[1389]: (CRON) INFO (pidfile fd = 3)
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 cron[1425]: (CRON) STARTUP (fork ok)
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 cron[1425]: (CRON) INFO (Running @reboot jobs)
Aug  7 14:44:55 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 acpid: starting up with netlink and the input layer
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 acpid: 1 rule loaded
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 acpid: waiting for events: event logging is off
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Created user account me.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Creating a new user account for henrik.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 haveged: haveged starting up
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Created user account henrik.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [   11.981889] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Creating a new user account for emma.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [   11.994080] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Created user account emma.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Creating a new user account for igor.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Created user account igor.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [   12.122185] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [   12.126541] Bridge firewalling registered
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [   12.136345] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Created user account konstantinhaase.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Creating a new user account for aj.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Created user account aj.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Creating a new user account for solarce.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [   12.221157] Initializing XFRM netlink socket
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [   12.230084] Netfilter messages via NETLINK v0.30.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [   12.233679] ctnetlink v0.93: registering with nfnetlink.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Created user account solarce.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Creating a new user account for asari.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Created user account asari.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Creating a new user account for bogdana.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Created user account bogdana.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Creating a new user account for konstantin.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Created user account konstantin.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Creating a new user account for carmen.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Created user account carmen.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Creating a new user account for maria.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Created user account maria.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 google-accounts: INFO Removing user packer.
Aug  7 14:44:56 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [   12.576344] floppy0: no floppy controllers found
Aug  7 14:45:01 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpdate[975]: adjust time server 169.254.169.254 offset 0.007282 sec
Aug  7 14:45:18 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpdate[1846]: adjust time server 169.254.169.254 offset 0.003990 sec
Aug  7 14:45:25 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpd[1881]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 14:45:25 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpd[1882]: proto: precision = 0.104 usec
Aug  7 14:45:25 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpd[1882]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 14:45:25 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpd[1882]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 14:45:25 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpd[1882]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  7 14:45:25 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpd[1882]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 14:45:25 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpd[1882]: Listen normally on 3 eth0 10.20.2.6 UDP 123
Aug  7 14:45:25 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpd[1882]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 14:45:25 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpd[1882]: peers refreshed
Aug  7 14:45:25 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpd[1882]: Listening on routing socket on fd #21 for interface updates
Aug  7 14:45:25 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [   42.178816] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 14:45:26 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 startup-script: INFO Starting startup scripts.
Aug  7 14:45:26 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 startup-script: INFO Found startup-script in metadata.
Aug  7 14:45:26 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 14:45:26 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 startup-script: INFO startup-script: job 1 at Tue Aug  7 17:55:00 2018
Aug  7 14:45:26 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 startup-script: INFO startup-script: Return code 0.
Aug  7 14:45:26 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 startup-script: INFO Finished running startup scripts.
Aug  7 14:45:26 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ec2: 
Aug  7 14:45:26 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ec2: #############################################################
Aug  7 14:45:26 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 14:45:26 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ec2: 1024 8a:e4:64:0a:72:bb:87:8a:df:06:3b:bb:9b:97:c2:83  root@travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 (DSA)
Aug  7 14:45:26 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ec2: 256 4c:14:c5:5c:b9:c4:02:b6:6a:f7:2c:21:8f:38:8d:4f  root@travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 (ECDSA)
Aug  7 14:45:26 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ec2: 256 05:fd:eb:bc:af:b6:1e:29:c6:f8:8c:73:00:87:09:31  root@travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 (ED25519)
Aug  7 14:45:26 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ec2: 2048 45:6d:8c:7d:9f:eb:70:61:4c:a2:59:c4:c4:a9:5e:16  root@travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 (RSA)
Aug  7 14:45:26 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 14:45:26 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ec2: #############################################################
Aug  7 14:46:47 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [  123.582977] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 14:47:41 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [  178.217626] device veth62814ab entered promiscuous mode
Aug  7 14:47:41 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [  178.217708] docker0: port 1(veth62814ab) entered forwarding state
Aug  7 14:47:41 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [  178.217716] docker0: port 1(veth62814ab) entered forwarding state
Aug  7 14:47:41 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [  178.218539] docker0: port 1(veth62814ab) entered disabled state
Aug  7 14:47:42 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [  178.300204] cgroup: docker-runc (4875) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 14:47:42 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [  178.300206] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 14:47:42 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [  178.383776] eth0: renamed from veth4ffed8e
Aug  7 14:47:42 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [  178.422521] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 14:47:42 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [  178.423702] docker0: port 1(veth62814ab) entered forwarding state
Aug  7 14:47:42 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [  178.423717] docker0: port 1(veth62814ab) entered forwarding state
Aug  7 14:47:42 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [  178.423739] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 14:47:45 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpd[1882]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  7 14:47:45 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpd[1882]: Listen normally on 6 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 14:47:45 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpd[1882]: Listen normally on 7 docker0 fe80::42:f8ff:febe:b2 UDP 123
Aug  7 14:47:45 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpd[1882]: peers refreshed
Aug  7 14:47:45 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 ntpd[1882]: new interface(s) found: waking up resolver
Aug  7 14:47:57 travis-job-a4c696c4-62d9-48b4-8a87-b34ec9bdc6d3 kernel: [  193.475548] docker0: port 1(veth62814ab) entered forwarding state
---
travis_time:end:10e7206f:start=1533653436271950542,finish=1533653436279326126,duration=7375584
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1ed0c677
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:30fa2886
travis_time:start:30fa2886
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:175c1130
$ dmesg | grep -i kill
