plain
[01:32:48] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "cargotest", path: "src/tools/cargotest", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 1.362
[01:32:48] testing https://github.com/servo/webrender
[01:32:48] Initialized empty Git repository in /checkout/obj/build/ct/webrender/.git/
[01:32:48] fatal: Could not parse object '57250b2b8fa63934f80e5376a29f7dcb3f759ad6'.
[01:33:16] fatal: unable to access 'https://github.com/servo/webrender/': Could not resolve host: github.com
[01:33:16] thread 'main' panicked at 'assertion failed: status.success()', tools/cargotest/main.rs:128:13
[01:33:16] 
[01:33:16] 
[01:33:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[01:33:16] expected success, got: exit code: 101
[01:33:16] expected success, got: exit code: 101
[01:33:16] 
[01:33:16] 
[01:33:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:33:16] Build completed unsuccessfully in 0:30:56
[01:33:16] make: *** [check-aux] Error 1
[01:33:16] Makefile:60: recipe for target 'check-aux' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1e7fb86b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:11cadf26
$ sudo tail -n 500 /var/log/syslog
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] Using GB pages for direct mapping
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] kvm-clock: using sched offset of 1586423023 cycles
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] Zone ranges:
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000]   Device   empty
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] Movable zone start for each node
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] Early memory node ranges
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] Policy zone: Normal
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] console [ttyS0] enabled
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.362234] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.363585] pid_max: default: 32768 minimum: 301
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.364350] ACPI: Core revision 20150930
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.370995] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.372374] Security Framework initialized
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.373186] Yama: becoming mindful.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.373994] AppArmor: AppArmor disabled by boot time parameter
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.376997] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.388377] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.393904] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.395063] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.396828] Initializing cgroup subsys io
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.397953] Initializing cgroup subsys memory
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.399339] Initializing cgroup subsys devices
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.400809] Initializing cgroup subsys freezer
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.402486] Initializing cgroup subsys net_cls
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.403547] Initializing cgroup subsys perf_event
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.404923] Initializing cgroup subsys net_prio
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.406020] Initializing cgroup subsys hugetlb
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.407548] Initializing cgroup subsys pids
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.408610] CPU: Physical Processor ID: 0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.409591] CPU: Processor Core ID: 0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.410224] mce: CPU supports 32 MCE banks
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.411500] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.412397] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.415567] Freeing SMP alternatives memory: 32K
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.426018] ftrace: allocating 32185 entries in 126 pages
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.477494] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.478824] smpboot: Max logical packages: 2
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.480294] x2apic enabled
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.482556] Switched APIC routing to physical x2apic.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.486781] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.593372] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.595400] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.599236] x86: Booting SMP configuration:
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.600228] .... node  #0, CPUs:      #1
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.601075] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.604921]  #2
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.605853] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.609404]  #3
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.609928] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.613470] x86: Booted up 1 node, 4 CPUs
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.614119] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.617102] devtmpfs: initialized
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.621891] evm: security.selinux
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.622985] evm: security.SMACK64
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.623811] evm: security.SMACK64EXEC
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.624729] evm: security.SMACK64TRANSMUTE
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.625893] evm: security.SMACK64MMAP
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.626735] evm: security.ima
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.627337] evm: security.capability
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.628437] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.629960] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.631863] pinctrl core: initialized pinctrl subsystem
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.632856] RTC time:  9:33:41, date: 08/10/18
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.634571] NET: Registered protocol family 16
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.645404] cpuidle: using governor ladder
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.657416] cpuidle: using governor menu
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.658599] PCCT header not found.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.659292] ACPI: bus type PCI registered
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.659983] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.661245] PCI: Using configuration type 1 for base access
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.674301] ACPI: Added _OSI(Module Device)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.675181] ACPI: Added _OSI(Processor Device)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.676002] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.677000] ACPI: Added _OSI(Processor Aggregator Device)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.680467] ACPI: Executed 2 blocks of module-level executable AML code
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.705448] ACPI: Interpreter enabled
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.706302] ACPI: (supports S0 S3 S4 S5)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.706931] ACPI: Using IOAPIC for interrupt routing
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.708038] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.739015] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.740389] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.741738] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.742866] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.745468] PCI host bridge to bus 0000:00
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.746202] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.747399] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.748412] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.749679] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.750916] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.752154] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.752618] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.773275] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.793475] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.795519] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.803341] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.808685] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.826852] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.835543] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.841609] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.859866] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.862868] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.865753] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.868915] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.872048] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.893445] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.895015] vgaarb: loaded
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.895661] SCSI subsystem initialized
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.896695] libata version 3.00 loaded.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.896724] ACPI: bus type USB registered
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.897399] usbcore: registered new interface driver usbfs
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.898269] usbcore: registered new interface driver hub
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.899272] usbcore: registered new device driver usb
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.900682] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.902505] dmi: Firmware registration failed.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.903401] PCI: Using ACPI for IRQ routing
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.904426] PCI: pci_cache_line_size set to 64 bytes
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.904534] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.904537] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.904676] NetLabel: Initializing
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.905414] NetLabel:  domain hash size = 128
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.906309] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.907073] NetLabel:  unlabeled traffic allowed by default
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.908384] amd_nb: Cannot enumerate AMD northbridges
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.909244] clocksource: Switched to clocksource kvm-clock
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.917985] pnp: PnP ACPI init
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.918666] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.918783] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.918829] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.918881] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.918925] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.918971] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.919012] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.919192] pnp: PnP ACPI: found 7 devices
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.927142] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.928507] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.928510] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.928512] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.928514] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.928552] NET: Registered protocol family 2
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.929565] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.931794] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.932955] TCP: Hash tables configured (established 131072 bind 65536)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.934195] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.935293] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.936567] NET: Registered protocol family 1
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.937335] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.938456] PCI: CLS 0 bytes, default 64
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    0.939270] Unpacking initramfs...
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.057459] Freeing initrd memory: 21432K
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.058358] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.059327] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.060824] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.062143] hw unit of domain pp0-core 2^-0 Joules
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.062930] hw unit of domain package 2^-0 Joules
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.063794] hw unit of domain dram 2^-0 Joules
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.064633] Scanning for low memory corruption every 60 seconds
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.066140] audit: initializing netlink subsys (disabled)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.067007] audit: type=2000 audit(1533893623.654:1): initialized
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.068151] Initialise system trusted keyring
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.069009] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.070151] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.072456] zbud: loaded
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.073451] VFS: Disk quotas dquot_6.6.0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.074324] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.075766] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.077145] fuse init (API version 7.23)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.078181] Key type big_key registered
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.079128] Allocating IMA MOK and blacklist keyrings.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.081262] Key type asymmetric registered
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.081999] Asymmetric key parser 'x509' registered
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.082869] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.084039] io scheduler noop registered
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.084659] io scheduler deadline registered (default)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.085677] io scheduler cfq registered
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.086500] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.087579] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.088790] intel_idle: does not run on family 6 model 45
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.088910] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.090005] ACPI: Power Button [PWRF]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.090665] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.092009] ACPI: Sleep Button [SLPF]
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.093111] GHES: HEST is not enabled!
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.096178] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.097382] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.103160] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.104313] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.111896] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.134858] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.158490] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.182263] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.205724] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.209518] Linux agpgart interface v0.103
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.213062] loop: module loaded
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.214039] libphy: Fixed MDIO Bus: probed
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.215082] tun: Universal TUN/TAP device driver, 1.6
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.216329] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.263772] PPP generic driver version 2.4.2
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.264926] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.266547] ehci-pci: EHCI PCI platform driver
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.267626] ehci-platform: EHCI generic platform driver
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.268708] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.270019] ohci-pci: OHCI PCI platform driver
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.270964] ohci-platform: OHCI generic platform driver
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.272003] uhci_hcd: USB Universal Host Controller Interface driver
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.273814] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.276377] i8042: Warning: Keylock active
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.278376] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.279329] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.280697] mousedev: PS/2 mouse device common for all mice
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.282246] rtc_cmos 00:00: RTC can wake from S4
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.283774] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.285181] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.286206] i2c /dev entries driver
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.287149] device-mapper: uevent: version 1.0.3
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.288399] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.290639] ledtrig-cpu: registered to indicate activity on CPUs
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.292478] NET: Registered protocol family 10
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.294103] NET: Registered protocol family 17
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.295249] Key type dns_resolver registered
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.296727] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.297899] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.299598] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.301094] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.302583] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.304838] registered taskstats version 1
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.305815] Loading compiled-in X.509 certificates
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.307477] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.309770] zswap: loaded using pool lzo/zbud
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.313044] Key type trusted registered
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.317893] Key type encrypted registered
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.318983] ima: No TPM chip found, activating TPM-bypass!
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.320561] evm: HMAC attrs: 0x1
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.321648]   Magic number: 14:744:575
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.322720]  workqueue: hash matches
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.323670] rtc_cmos 00:00: setting system clock to 2018-08-10 09:33:44 UTC (1533893624)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.325848] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.327348] EDD information not available.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.328286] PM: Hibernation image not present or could not be loaded.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.329690] Freeing unused kernel memory: 1496K
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.330553] Write protecting the kernel read-only data: 14336k
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.332282] Freeing unused kernel memory: 1956K
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.333678] Freeing unused kernel memory: 92K
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.347731] systemd-udevd[119]: starting version 204
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.396377] AVX version of gcm_enc/dec engaged.
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.397570] AES CTR mode by8 optimization enabled
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.423064] scsi host0: Virtio SCSI HBA
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.426991] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.452621] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.452628] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.452630] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.452891] sd 0:0:1:0: [sda] Write Protect is off
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.452893] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.452940] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.454428]  sda: sda1
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.455208] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    3.481548] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    4.061347] tsc: Refined TSC clocksource calibration: 2599.769 MHz
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    4.062711] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257961c8102, max_idle_ns: 440795256056 ns
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    4.314397] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    6.409443] floppy0: no floppy controllers found
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    7.561290] raid6: sse2x1   gen()  8984 MB/s
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    7.629293] raid6: sse2x1   xor()  6979 MB/s
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    7.697286] raid6: sse2x2   gen() 11344 MB/s
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    7.765299] raid6: sse2x2   xor()  7747 MB/s
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    7.833290] raid6: sse2x4   gen() 12741 MB/s
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    7.901273] raid6: sse2x4   xor()  8348 MB/s
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    7.902515] raid6: using algorithm sse2x4 gen() 12741 MB/s
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    7.903508] raid6: .... xor() 8348 MB/s, rmw enabled
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    7.904351] raid6: using ssse3x2 recovery algorithm
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    7.906588] xor: automatically using best checksumming function:
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    7.945282]    avx       : 21883.000 MB/sec
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    7.960898] Btrfs loaded
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    8.007541] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    8.009103] EXT4-fs (sda1): write access will be enabled during recovery
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    8.079092] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    8.085881] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    8.086867] EXT4-fs (sda1): recovery complete
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    8.092368] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    8.295471] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    8.399073] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    8.448234] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    8.624508] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    9.137927] random: cloud-init: uninitialized urandom read (32 bytes read, 44 bits of entropy available)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    9.262096] systemd-udevd[701]: starting version 204
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    9.365116] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    9.398241] intel_rapl: no valid rapl domains found in package 0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    9.426629] intel_rapl: no valid rapl domains found in package 0
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    9.466673] ppdev: user-space parallel port driver
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    9.548804] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    9.597681] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    9.661858] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [    9.828886] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [   10.069373] random: mktemp: uninitialized urandom read (12 bytes read, 59 bits of entropy available)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [   10.143332] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [   10.222460] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [   10.253536] EXT4-fs (sda1): resized filesystem to 7864064
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [   10.683721] init: failsafe main process (1094) killed by TERM signal
Aug 10 09:33:51 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO Running set_multiqueue.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO Set channels for eth0 to 4.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 google-accounts: INFO Starting Google Accounts daemon.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 google-accounts: INFO Creating a new user account for me.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 google-accounts: INFO Created user account me.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 google-accounts: INFO Created user account me.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 google-accounts: INFO Creating a new user account for bogdana.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [   11.430240] random: nonblocking pool is initialized
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 google-accounts: INFO Created user account bogdana.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 google-accounts: INFO Creating a new user account for aj.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 google-accounts: INFO Created user account aj.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 google-accounts: INFO Creating a new user account for asari.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 google-accounts: INFO Created user account asari.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 google-accounts: INFO Removing user packer.
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 cron[1434]: (CRON) INFO (pidfile fd = 3)
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 cron[1477]: (CRON) STARTUP (fork ok)
Aug 10 09:33:52 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 cron[1477]: (CRON) INFO (Running @reboot jobs)
Aug 10 09:33:53 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 acpid: starting up with netlink and the input layer
Aug 10 09:33:53 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 acpid: 1 rule loaded
Aug 10 09:33:53 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 acpid: waiting for events: event logging is off
Aug 10 09:33:53 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 haveged: haveged starting up
Aug 10 09:33:53 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 09:33:53 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 09:33:53 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [   11.921881] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 09:33:53 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [   11.935662] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 09:33:53 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [   12.050096] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 10 09:33:53 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [   12.052544] Bridge firewalling registered
Aug 10 09:33:53 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [   12.067889] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 10 09:33:53 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 google-clock-skew: INFO Synced system time with hardware clock.
Aug 10 09:33:53 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [   12.140306] Initializing XFRM netlink socket
Aug 10 09:33:53 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [   12.150632] Netfilter messages via NETLINK v0.30.
Aug 10 09:33:53 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [   12.153469] ctnetlink v0.93: registering with nfnetlink.
Aug 10 09:33:53 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [   12.417503] floppy0: no floppy controllers found
Aug 10 09:34:16 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpdate[1790]: adjust time server 169.254.169.254 offset 0.002327 sec
Aug 10 09:34:22 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpd[1825]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 10 09:34:22 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpd[1826]: proto: precision = 0.104 usec
Aug 10 09:34:22 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpd[1826]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 09:34:22 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpd[1826]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 09:34:22 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpd[1826]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 10 09:34:22 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpd[1826]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 10 09:34:22 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpd[1826]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 10 09:34:22 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpd[1826]: Listen normally on 3 eth0 10.20.255.107 UDP 123
Aug 10 09:34:22 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpd[1826]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 10 09:34:22 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpd[1826]: peers refreshed
Aug 10 09:34:22 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpd[1826]: Listening on routing socket on fd #21 for interface updates
Aug 10 09:34:22 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [   42.107198] init: plymouth-upstart-bridge main process ended, respawning
Aug 10 09:34:23 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 startup-script: INFO Found startup-script in metadata.
Aug 10 09:34:23 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 10 09:34:23 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 startup-script: INFO startup-script: job 1 at Fri Aug 10 12:44:00 2018
Aug 10 09:34:23 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 startup-script: INFO startup-script: Return code 0.
Aug 10 09:34:23 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 startup-script: INFO startup-script: Return code 0.
Aug 10 09:34:23 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 startup-script: INFO Finished running startup scripts.
Aug 10 09:34:23 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ec2: 
Aug 10 09:34:23 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ec2: #############################################################
Aug 10 09:34:23 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 10 09:34:23 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ec2: 1024 af:c1:69:50:38:b7:95:c1:b8:99:e1:06:1d:0e:03:d7  root@travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 (DSA)
Aug 10 09:34:23 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ec2: 256 93:66:0d:be:f4:d7:1a:12:66:23:ae:66:31:47:7d:4d  root@travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 (ECDSA)
Aug 10 09:34:23 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ec2: 256 41:c3:18:24:7e:33:9e:c4:07:b5:74:b1:66:d9:7d:bf  root@travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 (ED25519)
Aug 10 09:34:23 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ec2: 2048 6a:50:7f:b9:14:4e:35:04:6c:ae:2a:6c:62:84:51:b1  root@travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 (RSA)
Aug 10 09:34:23 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 10 09:34:23 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ec2: #############################################################
Aug 10 09:38:08 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [  267.830999] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 10 09:43:14 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [  573.877914] device veth278b576 entered promiscuous mode
Aug 10 09:43:14 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [  573.877994] docker0: port 1(veth278b576) entered forwarding state
Aug 10 09:43:14 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [  573.878006] docker0: port 1(veth278b576) entered forwarding state
Aug 10 09:43:14 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [  573.878405] docker0: port 1(veth278b576) entered disabled state
Aug 10 09:43:14 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [  573.987429] cgroup: docker-runc (4586) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 10 09:43:14 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [  573.987431] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 10 09:43:14 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [  574.059140] eth0: renamed from veth0e8df51
Aug 10 09:43:14 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [  574.102508] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 10 09:43:14 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [  574.104088] docker0: port 1(veth278b576) entered forwarding state
Aug 10 09:43:14 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [  574.104113] docker0: port 1(veth278b576) entered forwarding state
Aug 10 09:43:14 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [  574.104137] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 10 09:43:18 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpd[1826]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 10 09:43:18 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpd[1826]: Listen normally on 6 docker0 fe80::42:e9ff:feca:83e9 UDP 123
Aug 10 09:43:18 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpd[1826]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 10 09:43:18 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpd[1826]: peers refreshed
Aug 10 09:43:18 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 ntpd[1826]: new interface(s) found: waking up resolver
Aug 10 09:43:30 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [  589.134664] docker0: port 1(veth278b576) entered forwarding state
Aug 10 10:17:01 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 CRON[2462]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 10 10:43:43 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [ 4202.589150] traps: a[8996] trap invalid opcode ip:564b7b071bbb sp:7ffd97a76860 error:0 in a[564b7b06e000+6000]
Aug 10 10:44:03 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [ 4221.866379] traps: a[11871] trap invalid opcode ip:7fa410cbc5a1 sp:7ffef8a2d1c0 error:0 in libstd-41f43a30bc296e4f.so[7fa410c5f000+166000]
Aug 10 10:44:03 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [ 4221.894809] traps: a[11872] trap invalid opcode ip:7f5358d2d5a1 sp:7ffc348332c0 error:0 in libstd-41f43a30bc296e4f.so[7f5358cd0000+166000]
Aug 10 10:45:50 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [ 4328.953672] traps: a[26695] trap invalid opcode ip:55d9c507ae29 sp:7ffc0d34c8e0 error:0 in a[55d9c5078000+4000]
Aug 10 10:49:17 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [ 4536.361514] a[22697]: segfault at 0 ip 000055e6645983ef sp 00007fffb8d1bda0 error 6 in a[55e664595000+5000]
Aug 10 10:49:29 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [ 4548.286923] a[23458]: segfault at 1 ip 000055a21ff6cbed sp 00007ffca4215090 error 6 in a[55a21ff6a000+4000]
Aug 10 10:49:35 travis-job-4423a0a1-7415-4863-851d-5b61f34b5ce3 kernel: [ 4553.837332] traps: a[23887] trap invalid opcode ip:56262b52d4bc sp:7fffd3d81d10 error:0 in a[56262b52a000+7000]
---
travis_time:end:18491dc3:start=1533899623181972016,finish=1533899623190625936,duration=8653920
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:17e33b3c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0059c730
travis_time:start:0059c730
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0db60753
$ dmesg | grep -i kill
