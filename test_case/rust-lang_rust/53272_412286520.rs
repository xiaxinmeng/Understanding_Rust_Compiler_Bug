plain
[00:16:34]    Compiling rustc_passes v0.0.0 (file:///checkout/src/librustc_passes)
[00:16:34]    Compiling rustc_codegen_utils v0.0.0 (file:///checkout/src/librustc_codegen_utils)
[00:16:35]    Compiling rustc_borrowck v0.0.0 (file:///checkout/src/librustc_borrowck)
[00:16:35]    Compiling rustc_lint v0.0.0 (file:///checkout/src/librustc_lint)
[00:16:36] error: no rules expected the token `,`
[00:16:36]    --> librustc_lint/builtin.rs:631:9
[00:16:36] 631 |     Warn,
[00:16:36]     |         ^
[00:16:36] 
[00:16:36] error: aborting due to previous error
---
[00:16:36] warning: build failed, waiting for other jobs to finish...
[00:16:52] error: build failed
[00:16:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:52] expected success, got: exit code: 101
[00:16:52] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:16:52] travis_fold:end:stage0-rustc

[00:16:52] travis_time:end:stage0-rustc:start=1534004675580293667,finish=1534005391954047121,duration=716373753454


[00:16:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:52] Build completed unsuccessfully in 0:12:52
[00:16:52] make: *** [all] Error 1
[00:16:52] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09538611
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:062fa2c2
$ sudo tail -n 500 /var/log/syslog
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] kvm-clock: using sched offset of 1651685527 cycles
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] Zone ranges:
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000]   Device   empty
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] Movable zone start for each node
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] Early memory node ranges
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] Policy zone: Normal
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] console [ttyS0] enabled
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.513092] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.516025] pid_max: default: 32768 minimum: 301
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.517622] ACPI: Core revision 20150930
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.525351] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.528648] Security Framework initialized
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.530170] Yama: becoming mindful.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.532042] AppArmor: AppArmor disabled by boot time parameter
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.536249] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.549142] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.556045] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.558240] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.562452] Initializing cgroup subsys io
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.563926] Initializing cgroup subsys memory
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.565645] Initializing cgroup subsys devices
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.567240] Initializing cgroup subsys freezer
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.568516] Initializing cgroup subsys net_cls
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.570135] Initializing cgroup subsys perf_event
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.571610] Initializing cgroup subsys net_prio
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.572930] Initializing cgroup subsys hugetlb
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.574330] Initializing cgroup subsys pids
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.576232] CPU: Physical Processor ID: 0
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.577831] CPU: Processor Core ID: 0
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.581387] mce: CPU supports 32 MCE banks
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.583569] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.585492] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.590780] Freeing SMP alternatives memory: 32K
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.603928] ftrace: allocating 32185 entries in 126 pages
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.669935] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.672591] smpboot: Max logical packages: 2
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.674698] x2apic enabled
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.677106] Switched APIC routing to physical x2apic.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.682554] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.790801] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.794825] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.800534] x86: Booting SMP configuration:
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.802217] .... node  #0, CPUs:      #1
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.803991] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.809595]  #2
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.810721] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.816268]  #3
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.817185] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.823243] x86: Booted up 1 node, 4 CPUs
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.824654] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.828726] devtmpfs: initialized
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.833920] evm: security.selinux
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.835497] evm: security.SMACK64
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.837063] evm: security.SMACK64EXEC
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.838225] evm: security.SMACK64TRANSMUTE
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.839743] evm: security.SMACK64MMAP
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.840795] evm: security.ima
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.841963] evm: security.capability
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.844194] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.847957] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.851102] pinctrl core: initialized pinctrl subsystem
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.852795] RTC time: 16:18:26, date: 08/11/18
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.855269] NET: Registered protocol family 16
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.866958] cpuidle: using governor ladder
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.878925] cpuidle: using governor menu
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.881019] PCCT header not found.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.882289] ACPI: bus type PCI registered
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.883771] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.887007] PCI: Using configuration type 1 for base access
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.900490] ACPI: Added _OSI(Module Device)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.902157] ACPI: Added _OSI(Processor Device)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.903625] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.905012] ACPI: Added _OSI(Processor Aggregator Device)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.909509] ACPI: Executed 2 blocks of module-level executable AML code
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.935245] ACPI: Interpreter enabled
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.936749] ACPI: (supports S0 S3 S4 S5)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.938366] ACPI: Using IOAPIC for interrupt routing
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.939983] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.973941] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.975943] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.978083] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.980852] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.986023] PCI host bridge to bus 0000:00
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.987575] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.989637] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.992111] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.994642] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    0.997752] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.000303] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.001086] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.023344] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.047733] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.050807] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.060279] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.067345] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.087708] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.097753] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.105064] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.125671] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.130546] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.135200] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.139709] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.144385] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.168199] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.170899] vgaarb: loaded
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.172112] SCSI subsystem initialized
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.173749] libata version 3.00 loaded.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.173780] ACPI: bus type USB registered
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.175107] usbcore: registered new interface driver usbfs
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.176915] usbcore: registered new interface driver hub
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.178542] usbcore: registered new device driver usb
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.180634] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.182716] dmi: Firmware registration failed.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.184408] PCI: Using ACPI for IRQ routing
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.185713] PCI: pci_cache_line_size set to 64 bytes
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.185823] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.185825] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.185980] NetLabel: Initializing
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.186957] NetLabel:  domain hash size = 128
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.188627] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.190157] NetLabel:  unlabeled traffic allowed by default
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.192243] amd_nb: Cannot enumerate AMD northbridges
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.193782] clocksource: Switched to clocksource kvm-clock
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.203350] pnp: PnP ACPI init
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.205891] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.205962] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.206006] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.206071] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.206113] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.206154] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.206195] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.206367] pnp: PnP ACPI: found 7 devices
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.214702] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.218351] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.218353] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.218355] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.218356] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.218394] NET: Registered protocol family 2
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.219993] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.222566] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.225400] TCP: Hash tables configured (established 131072 bind 65536)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.227630] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.229368] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.232832] NET: Registered protocol family 1
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.234400] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.236908] PCI: CLS 0 bytes, default 64
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    1.236975] Unpacking initramfs...
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.421185] Freeing initrd memory: 21432K
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.423230] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.425821] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.430290] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.433134] hw unit of domain pp0-core 2^-0 Joules
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.434861] hw unit of domain package 2^-0 Joules
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.437562] hw unit of domain dram 2^-0 Joules
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.439913] Scanning for low memory corruption every 60 seconds
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.443897] audit: initializing netlink subsys (disabled)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.446214] audit: type=2000 audit(1534004308.568:1): initialized
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.448768] Initialise system trusted keyring
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.451304] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.454201] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.458131] zbud: loaded
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.459361] VFS: Disk quotas dquot_6.6.0
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.460543] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.463248] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.466060] fuse init (API version 7.23)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.467541] Key type big_key registered
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.468931] Allocating IMA MOK and blacklist keyrings.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.472941] Key type asymmetric registered
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.474668] Asymmetric key parser 'x509' registered
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.476273] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.478733] io scheduler noop registered
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.480038] io scheduler deadline registered (default)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.482156] io scheduler cfq registered
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.483328] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.485005] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.487605] intel_idle: does not run on family 6 model 62
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.487755] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.490134] ACPI: Power Button [PWRF]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.491432] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.493879] ACPI: Sleep Button [SLPF]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.495591] GHES: HEST is not enabled!
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.499279] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.501056] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.508005] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.509886] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.517301] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.540994] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.566056] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.590069] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.614554] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.619134] Linux agpgart interface v0.103
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.624345] loop: module loaded
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.626076] libphy: Fixed MDIO Bus: probed
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.627657] tun: Universal TUN/TAP device driver, 1.6
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.629562] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.669337] PPP generic driver version 2.4.2
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.671632] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.674256] ehci-pci: EHCI PCI platform driver
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.675713] ehci-platform: EHCI generic platform driver
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.677464] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.679747] ohci-pci: OHCI PCI platform driver
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.681244] ohci-platform: OHCI generic platform driver
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.683042] uhci_hcd: USB Universal Host Controller Interface driver
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.685403] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.688063] i8042: Warning: Keylock active
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.690847] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.692915] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.694915] mousedev: PS/2 mouse device common for all mice
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.697011] rtc_cmos 00:00: RTC can wake from S4
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.698839] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.701161] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.703478] i2c /dev entries driver
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.704751] device-mapper: uevent: version 1.0.3
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.706403] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.709260] ledtrig-cpu: registered to indicate activity on CPUs
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.712359] NET: Registered protocol family 10
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.714856] NET: Registered protocol family 17
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.716695] Key type dns_resolver registered
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.719430] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.722527] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.724448] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.726576] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.728746] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.733607] registered taskstats version 1
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.735586] Loading compiled-in X.509 certificates
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.739028] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.743436] zswap: loaded using pool lzo/zbud
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.748501] Key type trusted registered
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.754726] Key type encrypted registered
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.756910] ima: No TPM chip found, activating TPM-bypass!
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.759094] evm: HMAC attrs: 0x1
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.761351]   Magic number: 14:805:337
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.762606] tty tty58: hash matches
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.763912] rtc_cmos 00:00: setting system clock to 2018-08-11 16:18:29 UTC (1534004309)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.766717] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.768452] EDD information not available.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.770157] PM: Hibernation image not present or could not be loaded.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.771878] Freeing unused kernel memory: 1496K
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.773295] Write protecting the kernel read-only data: 14336k
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.776017] Freeing unused kernel memory: 1956K
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.778005] Freeing unused kernel memory: 92K
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.796288] systemd-udevd[119]: starting version 204
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.855814] scsi host0: Virtio SCSI HBA
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.866065] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.872236] AVX version of gcm_enc/dec engaged.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.873584] AES CTR mode by8 optimization enabled
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.894244] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.928528] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.931329] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.934202] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.937072] sd 0:0:1:0: [sda] Write Protect is off
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.938972] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.939235] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.945945]  sda: sda1
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    3.949053] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    4.437955] tsc: Refined TSC clocksource calibration: 2499.814 MHz
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    4.441180] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24088a0007e, max_idle_ns: 440795263290 ns
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    4.732309] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    6.850014] floppy0: no floppy controllers found
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.033805] raid6: sse2x1   gen()  8779 MB/s
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.101796] raid6: sse2x1   xor()  6861 MB/s
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.169804] raid6: sse2x2   gen() 11433 MB/s
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.237801] raid6: sse2x2   xor()  7867 MB/s
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.305799] raid6: sse2x4   gen() 12528 MB/s
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.373800] raid6: sse2x4   xor()  8729 MB/s
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.376046] raid6: using algorithm sse2x4 gen() 12528 MB/s
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.377835] raid6: .... xor() 8729 MB/s, rmw enabled
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.379370] raid6: using ssse3x2 recovery algorithm
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.383100] xor: automatically using best checksumming function:
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.421798]    avx       : 21869.000 MB/sec
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.440425] Btrfs loaded
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.493287] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.495909] EXT4-fs (sda1): write access will be enabled during recovery
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.575154] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.588703] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.591255] EXT4-fs (sda1): recovery complete
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.598341] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.811313] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.945342] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    8.999368] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    9.199052] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    9.804709] random: cloud-init: uninitialized urandom read (32 bytes read, 44 bits of entropy available)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [    9.955419] systemd-udevd[701]: starting version 204
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   10.061163] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   10.155322] ppdev: user-space parallel port driver
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   10.284952] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   10.344323] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   10.410116] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   10.585768] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   10.857474] random: mktemp: uninitialized urandom read (12 bytes read, 58 bits of entropy available)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   10.934609] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   11.020848] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   11.061931] EXT4-fs (sda1): resized filesystem to 7864064
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   11.429423] init: failsafe main process (1092) killed by TERM signal
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO Running set_multiqueue.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO Set channels for eth0 to 4.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Starting Google Accounts daemon.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Creating a new user account for me.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Created user account me.
Aug 11 16:18:37 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Creating a new user account for henrik.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Created user account henrik.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Creating a new user account for emma.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   12.343616] random: nonblocking pool is initialized
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Created user account emma.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Creating a new user account for igor.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-clock-skew: INFO Synced system time with hardware clock.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Created user account igor.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Created user account konstantinhaase.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Creating a new user account for aj.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Created user account aj.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Creating a new user account for solarce.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Created user account solarce.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Creating a new user account for asari.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Created user account asari.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Creating a new user account for bogdana.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 cron[1465]: (CRON) INFO (pidfile fd = 3)
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 cron[1511]: (CRON) STARTUP (fork ok)
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 cron[1511]: (CRON) INFO (Running @reboot jobs)
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Created user account bogdana.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 acpid: starting up with netlink and the input layer
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 acpid: 1 rule loaded
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 acpid: waiting for events: event logging is off
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Creating a new user account for konstantin.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 haveged: haveged starting up
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   12.887449] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   12.893462] Bridge firewalling registered
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   12.901131] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Created user account konstantin.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   12.915274] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   12.925037] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Creating a new user account for carmen.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Created user account carmen.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Creating a new user account for maria.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Created user account maria.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 google-accounts: INFO Removing user packer.
Aug 11 16:18:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   13.145993] floppy0: no floppy controllers found
Aug 11 16:18:39 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   13.963023] Initializing XFRM netlink socket
Aug 11 16:18:39 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   13.969951] Netfilter messages via NETLINK v0.30.
Aug 11 16:18:39 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   13.972176] ctnetlink v0.93: registering with nfnetlink.
Aug 11 16:19:01 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpdate[1854]: adjust time server 169.254.169.254 offset 0.005178 sec
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpd[1890]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpd[1891]: proto: precision = 0.108 usec
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpd[1891]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpd[1891]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpd[1891]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpd[1891]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpd[1891]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpd[1891]: Listen normally on 3 eth0 10.20.255.78 UDP 123
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpd[1891]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpd[1891]: peers refreshed
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpd[1891]: Listening on routing socket on fd #21 for interface updates
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   43.114586] init: plymouth-upstart-bridge main process ended, respawning
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 startup-script: INFO Found startup-script in metadata.
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 startup-script: INFO startup-script: job 1 at Sat Aug 11 19:29:00 2018
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 startup-script: INFO startup-script: Return code 0.
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 startup-script: INFO startup-script: Return code 0.
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 startup-script: INFO Finished running startup scripts.
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ec2: 
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ec2: #############################################################
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ec2: 1024 0e:d0:92:09:f7:d1:0f:55:3f:c1:ab:5c:98:bd:62:a9  root@travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 (DSA)
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ec2: 256 3f:e1:b6:d2:f0:5b:15:c5:2c:33:b0:85:14:df:e7:4d  root@travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 (ECDSA)
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ec2: 256 d3:4e:7f:b1:73:6b:56:a3:28:9b:53:70:e6:22:62:f0  root@travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 (ED25519)
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ec2: 2048 75:60:ac:6c:de:25:88:d7:fb:9f:01:b9:a5:96:89:64  root@travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 (RSA)
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 11 16:19:08 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ec2: #############################################################
Aug 11 16:19:38 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [   73.332263] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 11 16:20:43 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [  138.234821] device veth350474d entered promiscuous mode
Aug 11 16:20:43 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [  138.311330] cgroup: docker-runc (4872) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 11 16:20:43 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [  138.311333] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 11 16:20:43 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [  138.375145] eth0: renamed from veth901f761
Aug 11 16:20:44 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [  138.423047] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 11 16:20:44 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [  138.424113] docker0: port 1(veth350474d) entered forwarding state
Aug 11 16:20:44 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [  138.424130] docker0: port 1(veth350474d) entered forwarding state
Aug 11 16:20:44 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [  138.424149] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 11 16:20:47 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpd[1891]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 11 16:20:47 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpd[1891]: Listen normally on 6 docker0 fe80::42:ddff:fec8:88a1 UDP 123
Aug 11 16:20:47 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpd[1891]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 11 16:20:47 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpd[1891]: peers refreshed
Aug 11 16:20:47 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 ntpd[1891]: new interface(s) found: waking up resolver
Aug 11 16:20:59 travis-job-1245356d-8e9a-4cda-aa2a-9ef407e77712 kernel: [  153.458982] docker0: port 1(veth350474d) entered forwarding state
---
travis_time:end:0606533d:start=1534005392572322133,finish=1534005392578683327,duration=6361194
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c7cd300
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08054040
travis_time:start:08054040
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:020fb737
$ dmesg | grep -i kill
