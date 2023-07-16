plain

[00:05:22] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:22] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:72: TODO is deprecated; use FIXME
[00:05:22] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:100: TODO is deprecated; use FIXME
[00:05:22] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:108: TODO is deprecated; use FIXME
[00:05:22] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:136: TODO is deprecated; use FIXME
[00:05:22] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:164: TODO is deprecated; use FIXME
[00:05:22] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:172: TODO is deprecated; use FIXME
[00:05:22] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:195: TODO is deprecated; use FIXME
[00:05:22] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:283: TODO is deprecated; use FIXME
[00:05:22] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:318: TODO is deprecated; use FIXME
[00:05:23] some tidy checks failed
[00:05:23] 
[00:05:23] 
[00:05:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:23] 
[00:05:23] 
[00:05:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:23] Build completed unsuccessfully in 0:00:53
[00:05:23] Build completed unsuccessfully in 0:00:53
[00:05:23] make: *** [tidy] Error 1
[00:05:23] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:190eaf1d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:32444940
$ sudo tail -n 500 /var/log/syslog
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] kvm-clock: using sched offset of 1947078849 cycles
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] Zone ranges:
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000]   Device   empty
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] Movable zone start for each node
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] Early memory node ranges
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] Policy zone: Normal
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] console [ttyS0] enabled
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.553129] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.556608] pid_max: default: 32768 minimum: 301
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.558946] ACPI: Core revision 20150930
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.567051] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.570327] Security Framework initialized
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.572246] Yama: becoming mindful.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.573728] AppArmor: AppArmor disabled by boot time parameter
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.578453] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.591609] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.599715] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.603473] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.607952] Initializing cgroup subsys io
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.609647] Initializing cgroup subsys memory
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.611568] Initializing cgroup subsys devices
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.614101] Initializing cgroup subsys freezer
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.616948] Initializing cgroup subsys net_cls
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.619333] Initializing cgroup subsys perf_event
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.621463] Initializing cgroup subsys net_prio
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.624048] Initializing cgroup subsys hugetlb
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.625822] Initializing cgroup subsys pids
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.628202] CPU: Physical Processor ID: 0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.629719] CPU: Processor Core ID: 0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.632127] mce: CPU supports 32 MCE banks
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.634379] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.636962] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.642541] Freeing SMP alternatives memory: 32K
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.655865] ftrace: allocating 32185 entries in 126 pages
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.718429] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.721801] smpboot: Max logical packages: 2
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.724651] x2apic enabled
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.727407] Switched APIC routing to physical x2apic.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.733072] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.842766] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.847016] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.852460] x86: Booting SMP configuration:
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.854844] .... node  #0, CPUs:      #1
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.856762] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.862113]  #2
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.862919] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.867722]  #3
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.868807] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.874779] x86: Booted up 1 node, 4 CPUs
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.877416] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.882089] devtmpfs: initialized
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.888362] evm: security.selinux
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.889944] evm: security.SMACK64
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.891642] evm: security.SMACK64EXEC
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.893273] evm: security.SMACK64TRANSMUTE
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.895502] evm: security.SMACK64MMAP
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.897430] evm: security.ima
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.899329] evm: security.capability
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.901385] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.906282] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.909441] pinctrl core: initialized pinctrl subsystem
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.911938] RTC time: 21:01:52, date: 08/12/18
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.915077] NET: Registered protocol family 16
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.930844] cpuidle: using governor ladder
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.942810] cpuidle: using governor menu
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.945130] PCCT header not found.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.947289] ACPI: bus type PCI registered
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.950534] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.953768] PCI: Using configuration type 1 for base access
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.968779] ACPI: Added _OSI(Module Device)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.970332] ACPI: Added _OSI(Processor Device)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.972110] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.974949] ACPI: Added _OSI(Processor Aggregator Device)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    0.980407] ACPI: Executed 2 blocks of module-level executable AML code
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.009614] ACPI: Interpreter enabled
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.012368] ACPI: (supports S0 S3 S4 S5)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.014530] ACPI: Using IOAPIC for interrupt routing
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.016912] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.052232] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.055886] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.058856] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.062299] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.069357] PCI host bridge to bus 0000:00
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.071034] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.074987] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.078433] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.081736] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.085284] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.087974] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.088497] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.115695] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.147861] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.152288] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.165344] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.177002] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.206216] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.219012] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.230145] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.260042] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.265834] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.271047] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.276805] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.281825] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.304898] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.307475] vgaarb: loaded
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.308808] SCSI subsystem initialized
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.310690] libata version 3.00 loaded.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.310724] ACPI: bus type USB registered
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.312970] usbcore: registered new interface driver usbfs
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.315051] usbcore: registered new interface driver hub
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.317266] usbcore: registered new device driver usb
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.320027] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.323192] dmi: Firmware registration failed.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.325377] PCI: Using ACPI for IRQ routing
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.327562] PCI: pci_cache_line_size set to 64 bytes
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.327670] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.327673] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.327828] NetLabel: Initializing
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.329203] NetLabel:  domain hash size = 128
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.330569] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.332198] NetLabel:  unlabeled traffic allowed by default
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.334399] amd_nb: Cannot enumerate AMD northbridges
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.336492] clocksource: Switched to clocksource kvm-clock
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.347194] pnp: PnP ACPI init
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.349336] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.349409] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.349511] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.349567] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.349618] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.349663] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.349705] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.349883] pnp: PnP ACPI: found 7 devices
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.359103] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.362613] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.362616] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.362618] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.362619] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.362661] NET: Registered protocol family 2
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.364437] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.367937] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.370553] TCP: Hash tables configured (established 131072 bind 65536)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.373141] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.376650] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.379466] NET: Registered protocol family 1
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.381003] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.383877] PCI: CLS 0 bytes, default 64
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    1.384775] Unpacking initramfs...
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.633334] Freeing initrd memory: 21432K
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.634790] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.637237] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.640907] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.643643] hw unit of domain pp0-core 2^-0 Joules
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.645382] hw unit of domain package 2^-0 Joules
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.647282] hw unit of domain dram 2^-0 Joules
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.649623] Scanning for low memory corruption every 60 seconds
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.652565] audit: initializing netlink subsys (disabled)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.654850] audit: type=2000 audit(1534107714.562:1): initialized
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.657207] Initialise system trusted keyring
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.659807] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.662366] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.666203] zbud: loaded
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.668076] VFS: Disk quotas dquot_6.6.0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.669921] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.672835] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.675708] fuse init (API version 7.23)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.677634] Key type big_key registered
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.679113] Allocating IMA MOK and blacklist keyrings.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.686150] Key type asymmetric registered
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.688443] Asymmetric key parser 'x509' registered
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.690446] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.693764] io scheduler noop registered
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.695052] io scheduler deadline registered (default)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.696849] io scheduler cfq registered
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.698971] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.701070] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.704408] intel_idle: does not run on family 6 model 45
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.704570] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.707767] ACPI: Power Button [PWRF]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.710023] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.713973] ACPI: Sleep Button [SLPF]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.716366] GHES: HEST is not enabled!
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.722052] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.725060] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.737085] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.741188] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.754887] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.780965] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.808672] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.834616] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.861706] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.868096] Linux agpgart interface v0.103
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.874305] loop: module loaded
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.876630] libphy: Fixed MDIO Bus: probed
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.879262] tun: Universal TUN/TAP device driver, 1.6
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.881696] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.953531] PPP generic driver version 2.4.2
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.956655] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.960665] ehci-pci: EHCI PCI platform driver
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.964223] ehci-platform: EHCI generic platform driver
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.966744] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.969637] ohci-pci: OHCI PCI platform driver
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.972119] ohci-platform: OHCI generic platform driver
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.974788] uhci_hcd: USB Universal Host Controller Interface driver
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.978182] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.982991] i8042: Warning: Keylock active
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.986763] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.989292] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.993015] mousedev: PS/2 mouse device common for all mice
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    3.997556] rtc_cmos 00:00: RTC can wake from S4
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.000341] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.004113] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.008003] i2c /dev entries driver
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.010630] device-mapper: uevent: version 1.0.3
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.013044] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.017362] ledtrig-cpu: registered to indicate activity on CPUs
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.021004] NET: Registered protocol family 10
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.024516] NET: Registered protocol family 17
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.027250] Key type dns_resolver registered
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.029341] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.033501] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.036047] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.038678] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.041811] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.045979] registered taskstats version 1
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.047628] Loading compiled-in X.509 certificates
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.050394] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.055608] zswap: loaded using pool lzo/zbud
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.060579] Key type trusted registered
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.068988] Key type encrypted registered
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.070815] ima: No TPM chip found, activating TPM-bypass!
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.073490] evm: HMAC attrs: 0x1
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.075546]   Magic number: 14:104:45
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.077230] rtc_cmos 00:00: setting system clock to 2018-08-12 21:01:55 UTC (1534107715)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.081148] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.083336] EDD information not available.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.085094] PM: Hibernation image not present or could not be loaded.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.086751] Freeing unused kernel memory: 1496K
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.088564] Write protecting the kernel read-only data: 14336k
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.092019] Freeing unused kernel memory: 1956K
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.094381] Freeing unused kernel memory: 92K
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.117240] systemd-udevd[119]: starting version 204
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.197591] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.209650] scsi host0: Virtio SCSI HBA
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.217426] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.225823] AVX version of gcm_enc/dec engaged.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.227897] AES CTR mode by8 optimization enabled
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.294535] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.294760] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.294762] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.295215] sd 0:0:1:0: [sda] Write Protect is off
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.295218] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.295309] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.297974]  sda: sda1
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.300729] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.648655] tsc: Refined TSC clocksource calibration: 2599.813 MHz
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    4.651607] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x25798b4459b, max_idle_ns: 440795257440 ns
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    5.054597] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    7.176693] floppy0: no floppy controllers found
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.336510] raid6: sse2x1   gen()  8913 MB/s
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.404505] raid6: sse2x1   xor()  6800 MB/s
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.472511] raid6: sse2x2   gen() 11132 MB/s
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.540505] raid6: sse2x2   xor()  7546 MB/s
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.608504] raid6: sse2x4   gen() 12420 MB/s
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.676506] raid6: sse2x4   xor()  8399 MB/s
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.677441] raid6: using algorithm sse2x4 gen() 12420 MB/s
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.678502] raid6: .... xor() 8399 MB/s, rmw enabled
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.679531] raid6: using ssse3x2 recovery algorithm
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.682082] xor: automatically using best checksumming function:
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.720506]    avx       : 22173.000 MB/sec
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.735759] Btrfs loaded
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.799271] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.800743] EXT4-fs (sda1): write access will be enabled during recovery
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.893967] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.904091] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.905478] EXT4-fs (sda1): recovery complete
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    8.911846] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    9.214680] random: init: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    9.398334] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    9.475872] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [    9.788881] random: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   10.583101] random: cloud-init: uninitialized urandom read (32 bytes read, 45 bits of entropy available)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   10.763221] systemd-udevd[702]: starting version 204
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   10.947027] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   10.997275] intel_rapl: no valid rapl domains found in package 0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   11.034687] intel_rapl: no valid rapl domains found in package 0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   11.056973] ppdev: user-space parallel port driver
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   11.205802] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   11.276742] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   11.353371] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   11.530168] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   11.835472] random: mktemp: uninitialized urandom read (12 bytes read, 60 bits of entropy available)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   11.940164] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   12.051274] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   12.123416] EXT4-fs (sda1): resized filesystem to 7864064
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   12.606580] init: failsafe main process (1096) killed by TERM signal
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO Running set_multiqueue.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO Set channels for eth0 to 4.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Starting Google Accounts daemon.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Creating a new user account for me.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Created user account me.
Aug 12 21:02:04 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Creating a new user account for henrik.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-clock-skew: INFO Synced system time with hardware clock.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Created user account henrik.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Creating a new user account for emma.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Created user account emma.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Creating a new user account for igor.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Created user account igor.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Created user account konstantinhaase.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Creating a new user account for aj.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Created user account aj.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Creating a new user account for solarce.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   14.057116] floppy0: no floppy controllers found
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   14.065056] random: nonblocking pool is initialized
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Created user account solarce.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Creating a new user account for asari.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Created user account asari.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Creating a new user account for bogdana.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   14.231756] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   14.236677] Bridge firewalling registered
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   14.250990] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Created user account bogdana.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   14.300279] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Creating a new user account for konstantin.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Created user account konstantin.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Creating a new user account for carmen.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   14.389995] Initializing XFRM netlink socket
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   14.398569] Netfilter messages via NETLINK v0.30.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   14.401912] ctnetlink v0.93: registering with nfnetlink.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Created user account carmen.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Creating a new user account for maria.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Created user account maria.
Aug 12 21:02:05 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 google-accounts: INFO Removing user packer.
Aug 12 21:02:08 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 cron[1714]: (CRON) INFO (pidfile fd = 3)
Aug 12 21:02:08 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 12 21:02:08 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 12 21:02:08 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 cron[1749]: (CRON) STARTUP (fork ok)
Aug 12 21:02:08 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 cron[1749]: (CRON) INFO (Running @reboot jobs)
Aug 12 21:02:08 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 acpid: starting up with netlink and the input layer
Aug 12 21:02:08 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 acpid: 1 rule loaded
Aug 12 21:02:08 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 acpid: waiting for events: event logging is off
Aug 12 21:02:08 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 haveged: haveged starting up
Aug 12 21:02:09 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   17.631458] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpd[1849]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpd[1850]: proto: precision = 0.102 usec
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpd[1850]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpd[1850]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpd[1850]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpd[1850]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpd[1850]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpd[1850]: Listen normally on 3 eth0 10.20.0.21 UDP 123
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpd[1850]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpd[1850]: peers refreshed
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpd[1850]: Listening on routing socket on fd #21 for interface updates
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [   22.873072] init: plymouth-upstart-bridge main process ended, respawning
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 startup-script: INFO Found startup-script in metadata.
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 startup-script: INFO startup-script: job 1 at Mon Aug 13 00:12:00 2018
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 startup-script: INFO startup-script: Return code 0.
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 startup-script: INFO startup-script: Return code 0.
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 startup-script: INFO Finished running startup scripts.
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ec2: 
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ec2: #############################################################
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ec2: 1024 95:70:1e:84:76:41:42:74:31:a5:36:c0:81:8e:a2:28  root@travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 (DSA)
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ec2: 256 ac:db:58:44:7f:00:67:34:28:7e:9e:bf:c1:c8:96:67  root@travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 (ECDSA)
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ec2: 256 a0:97:7a:1b:d8:16:12:ac:fb:03:98:13:c9:bf:6f:61  root@travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 (ED25519)
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ec2: 2048 8a:6a:2d:f4:69:09:22:3c:55:9d:b3:14:fc:55:a6:98  root@travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 (RSA)
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 12 21:02:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ec2: #############################################################
Aug 12 21:02:20 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpdate[2243]: the NTP socket is in use, exiting
Aug 12 21:03:51 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [  119.983246] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 12 21:04:59 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [  187.700592] device veth7fb05e4 entered promiscuous mode
Aug 12 21:04:59 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [  187.824403] cgroup: docker-runc (4860) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 12 21:04:59 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [  187.824406] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 12 21:04:59 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [  187.908064] eth0: renamed from vetha813495
Aug 12 21:04:59 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [  187.947964] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 12 21:04:59 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [  187.949326] docker0: port 1(veth7fb05e4) entered forwarding state
Aug 12 21:04:59 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [  187.949342] docker0: port 1(veth7fb05e4) entered forwarding state
Aug 12 21:04:59 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [  187.949367] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 12 21:05:03 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpd[1850]: Listen normally on 5 docker0 fe80::42:adff:fe28:9807 UDP 123
Aug 12 21:05:03 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpd[1850]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 12 21:05:03 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpd[1850]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 12 21:05:03 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpd[1850]: peers refreshed
Aug 12 21:05:03 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 ntpd[1850]: new interface(s) found: waking up resolver
Aug 12 21:05:14 travis-job-1f3d2c48-ccb0-4e0b-9cce-782eba96b1e2 kernel: [  202.951056] docker0: port 1(veth7fb05e4) entered forwarding state
---
travis_time:end:00d20348:start=1534108156729494167,finish=1534108156736410268,duration=6916101
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2151d3f4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a366714
travis_time:start:1a366714
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:03a772d0
$ dmesg | grep -i kill
