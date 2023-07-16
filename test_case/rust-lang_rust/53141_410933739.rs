plain
############################                                              40.1%
######################################################################## 100.0%
[00:01:21] extracting /checkout/obj/build/cache/2018-08-01/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:21]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:43]     Updating git repository `https://github.com/racer-rust/racer`
[00:01:45]  Downloading filetime v0.2.1
[00:01:45]  Downloading libc v0.2.42
[00:01:45]  Downloading petgraph v0.4.12
[00:01:45]  Downloading cmake v0.1.31
---
tidy check
[00:04:04] * 557 error codes
[00:04:04] * highest error code: E0711
[00:04:04] * 213 features
[00:04:05] invalid source: "git+https://github.com/racer-rust/racer?rev=2af3ab88420e968500fcfc75418be7e852debfa0#2af3ab88420e968500fcfc75418be7e852debfa0"
[00:04:05] some tidy checks failed
[00:04:05] 
[00:04:05] 
[00:04:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:05] 
[00:04:05] 
[00:04:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:05] Build completed unsuccessfully in 0:00:50
[00:04:05] Build completed unsuccessfully in 0:00:50
[00:04:05] make: *** [tidy] Error 1
[00:04:05] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:32496e58
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:131f79a4
$ sudo tail -n 500 /var/log/syslog
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: RSDP 0x00000000000F27D0 000014 (v00 Google)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] kvm-clock: using sched offset of 1230100389 cycles
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] Zone ranges:
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000]   Device   empty
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] Movable zone start for each node
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] Early memory node ranges
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] Policy zone: Normal
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] console [ttyS0] enabled
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.300909] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.302095] pid_max: default: 32768 minimum: 301
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.302731] ACPI: Core revision 20150930
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.308889] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.309930] Security Framework initialized
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.310499] Yama: becoming mindful.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.310993] AppArmor: AppArmor disabled by boot time parameter
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.313484] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.322572] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.326718] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.327835] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.329125] Initializing cgroup subsys io
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.329686] Initializing cgroup subsys memory
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.330289] Initializing cgroup subsys devices
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.330898] Initializing cgroup subsys freezer
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.331584] Initializing cgroup subsys net_cls
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.332267] Initializing cgroup subsys perf_event
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.332924] Initializing cgroup subsys net_prio
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.333568] Initializing cgroup subsys hugetlb
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.334176] Initializing cgroup subsys pids
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.334821] CPU: Physical Processor ID: 0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.335390] CPU: Processor Core ID: 0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.335912] mce: CPU supports 32 MCE banks
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.336603] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.337350] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.339950] Freeing SMP alternatives memory: 32K
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.348031] ftrace: allocating 32185 entries in 126 pages
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.393692] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.394807] smpboot: Max logical packages: 2
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.395998] x2apic enabled
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.397764] Switched APIC routing to physical x2apic.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.401147] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.508245] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.509633] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.511707] x86: Booting SMP configuration:
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.512442] .... node  #0, CPUs:      #1
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.513211] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.516586]  #2
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.517070] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.520484]  #3
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.520904] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.524028] x86: Booted up 1 node, 4 CPUs
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.524611] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.526827] devtmpfs: initialized
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.531204] evm: security.selinux
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.531738] evm: security.SMACK64
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.532237] evm: security.SMACK64EXEC
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.532750] evm: security.SMACK64TRANSMUTE
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.533314] evm: security.SMACK64MMAP
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.533820] evm: security.ima
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.534256] evm: security.capability
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.535021] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.536433] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.537468] pinctrl core: initialized pinctrl subsystem
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.538387] RTC time:  3:49:08, date: 08/07/18
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.539922] NET: Registered protocol family 16
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.552268] cpuidle: using governor ladder
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.564275] cpuidle: using governor menu
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.564950] PCCT header not found.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.565513] ACPI: bus type PCI registered
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.566086] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.567307] PCI: Using configuration type 1 for base access
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.581253] ACPI: Added _OSI(Module Device)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.581956] ACPI: Added _OSI(Processor Device)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.582674] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.583336] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.586490] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.609394] ACPI: Interpreter enabled
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.609957] ACPI: (supports S0 S3 S4 S5)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.610510] ACPI: Using IOAPIC for interrupt routing
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.611203] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.640164] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.641110] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.642047] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.642945] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.645134] PCI host bridge to bus 0000:00
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.645701] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.646625] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.647533] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.648620] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.649699] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.650583] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.650975] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.663523] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.675152] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.676417] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.681697] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.685596] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.695597] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.700469] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.704634] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.716166] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.717963] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.720052] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.721924] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.723758] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.743991] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.744925] vgaarb: loaded
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.745630] SCSI subsystem initialized
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.746277] libata version 3.00 loaded.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.746304] ACPI: bus type USB registered
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.746877] usbcore: registered new interface driver usbfs
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.747646] usbcore: registered new interface driver hub
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.748431] usbcore: registered new device driver usb
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.749476] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.750512] dmi: Firmware registration failed.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.751406] PCI: Using ACPI for IRQ routing
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.752011] PCI: pci_cache_line_size set to 64 bytes
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.752109] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.752111] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.752221] NetLabel: Initializing
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.752723] NetLabel:  domain hash size = 128
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.753404] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.754248] NetLabel:  unlabeled traffic allowed by default
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.755148] amd_nb: Cannot enumerate AMD northbridges
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.755906] clocksource: Switched to clocksource kvm-clock
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.762891] pnp: PnP ACPI init
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.763438] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.763508] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.763553] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.763604] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.763646] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.763688] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.763730] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.763877] pnp: PnP ACPI: found 7 devices
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.771563] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.773000] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.773003] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.773004] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.773006] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.773042] NET: Registered protocol family 2
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.773862] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.775750] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.776793] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.777825] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.778724] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.780362] NET: Registered protocol family 1
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.780980] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.781849] PCI: CLS 0 bytes, default 64
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    0.781891] Unpacking initramfs...
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.715055] Freeing initrd memory: 21432K
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.715891] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.716805] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.718205] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.719421] hw unit of domain pp0-core 2^-0 Joules
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.720078] hw unit of domain package 2^-0 Joules
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.720721] hw unit of domain dram 2^-0 Joules
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.721417] Scanning for low memory corruption every 60 seconds
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.722861] audit: initializing netlink subsys (disabled)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.723648] audit: type=2000 audit(1533613750.504:1): initialized
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.724793] Initialise system trusted keyring
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.725614] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.726505] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.728550] zbud: loaded
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.729273] VFS: Disk quotas dquot_6.6.0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.729862] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.731008] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.732333] fuse init (API version 7.23)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.733072] Key type big_key registered
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.733631] Allocating IMA MOK and blacklist keyrings.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.735356] Key type asymmetric registered
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.736027] Asymmetric key parser 'x509' registered
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.736742] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.738047] io scheduler noop registered
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.738749] io scheduler deadline registered (default)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.739528] io scheduler cfq registered
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.740150] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.740915] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.741863] intel_idle: does not run on family 6 model 45
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.741946] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.743038] ACPI: Power Button [PWRF]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.743606] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.744720] ACPI: Sleep Button [SLPF]
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.745644] GHES: HEST is not enabled!
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.748927] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.749952] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.753791] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.754703] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.758828] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.781104] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.803512] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.825855] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.848886] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.852032] Linux agpgart interface v0.103
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.854872] loop: module loaded
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.855749] libphy: Fixed MDIO Bus: probed
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.856254] tun: Universal TUN/TAP device driver, 1.6
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.856799] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.883580] PPP generic driver version 2.4.2
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.884730] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.886201] ehci-pci: EHCI PCI platform driver
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.887201] ehci-platform: EHCI generic platform driver
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.888243] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.889759] ohci-pci: OHCI PCI platform driver
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.890602] ohci-platform: OHCI generic platform driver
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.891513] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.892322] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.894012] i8042: Warning: Keylock active
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.895778] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.897003] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.898097] mousedev: PS/2 mouse device common for all mice
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.899560] rtc_cmos 00:00: RTC can wake from S4
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.900571] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.901939] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.903551] i2c /dev entries driver
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.904211] device-mapper: uevent: version 1.0.3
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.905060] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.906339] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.907791] NET: Registered protocol family 10
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.909012] NET: Registered protocol family 17
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.909855] Key type dns_resolver registered
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.911208] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.911885] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.912529] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.913398] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.914895] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.916019] registered taskstats version 1
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.916655] Loading compiled-in X.509 certificates
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.918041] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.919605] zswap: loaded using pool lzo/zbud
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.922329] Key type trusted registered
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.926620] Key type encrypted registered
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.927337] ima: No TPM chip found, activating TPM-bypass!
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.928176] evm: HMAC attrs: 0x1
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.929285]   Magic number: 14:639:815
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.930089] rtc_cmos 00:00: setting system clock to 2018-08-07 03:49:10 UTC (1533613750)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.931363] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.932093] EDD information not available.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.932939] PM: Hibernation image not present or could not be loaded.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.934283] Freeing unused kernel memory: 1496K
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.935014] Write protecting the kernel read-only data: 14336k
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.937079] Freeing unused kernel memory: 1956K
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.937986] Freeing unused kernel memory: 92K
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    2.951145] systemd-udevd[118]: starting version 204
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    3.009330] scsi host0: Virtio SCSI HBA
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    3.012837] AVX version of gcm_enc/dec engaged.
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    3.013522] AES CTR mode by8 optimization enabled
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    3.015800] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    3.045806] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    3.045812] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    3.047903] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    3.049076] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    3.049873] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    3.049996] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    3.052486]  sda: sda1
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    3.053784] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    3.096436] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    3.719990] tsc: Refined TSC clocksource calibration: 2600.001 MHz
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    3.721096] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3ce1c4c, max_idle_ns: 440795206275 ns
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    3.933417] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    6.008076] floppy0: no floppy controllers found
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.155966] raid6: sse2x1   gen()  8996 MB/s
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.224156] raid6: sse2x1   xor()  6734 MB/s
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.291950] raid6: sse2x2   gen() 11204 MB/s
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.359949] raid6: sse2x2   xor()  7339 MB/s
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.427932] raid6: sse2x4   gen() 12951 MB/s
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.495934] raid6: sse2x4   xor()  9060 MB/s
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.496603] raid6: using algorithm sse2x4 gen() 12951 MB/s
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.497497] raid6: .... xor() 9060 MB/s, rmw enabled
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.498172] raid6: using ssse3x2 recovery algorithm
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.500280] xor: automatically using best checksumming function:
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.539953]    avx       : 27984.000 MB/sec
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.553177] Btrfs loaded
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.590289] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.591396] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.653661] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.659369] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.660210] EXT4-fs (sda1): recovery complete
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.664386] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.846147] random: init: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.944571] random: mountall: uninitialized urandom read (12 bytes read, 32 bits of entropy available)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    7.989741] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    8.165037] random: cloud-init: uninitialized urandom read (32 bytes read, 40 bits of entropy available)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    8.666993] random: cloud-init: uninitialized urandom read (32 bytes read, 48 bits of entropy available)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    8.790987] systemd-udevd[701]: starting version 204
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    8.890536] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    8.941605] intel_rapl: no valid rapl domains found in package 0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    8.981116] intel_rapl: no valid rapl domains found in package 0
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    9.033522] ppdev: user-space parallel port driver
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    9.103745] random: mktemp: uninitialized urandom read (6 bytes read, 60 bits of entropy available)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    9.146399] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    9.204705] random: cloud-init: uninitialized urandom read (32 bytes read, 61 bits of entropy available)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    9.358135] random: cloud-init: uninitialized urandom read (32 bytes read, 61 bits of entropy available)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    9.631749] random: mktemp: uninitialized urandom read (12 bytes read, 64 bits of entropy available)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    9.694791] random: mktemp: uninitialized urandom read (6 bytes read, 65 bits of entropy available)
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    9.755961] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [    9.789615] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 03:49:17 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [   10.198529] init: failsafe main process (1092) killed by TERM signal
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO Running set_multiqueue.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO Set channels for eth0 to 4.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [   10.818631] random: nonblocking pool is initialized
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Starting Google Accounts daemon.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Creating a new user account for me.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Created user account me.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Created user account me.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Creating a new user account for henrik.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Created user account henrik.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Creating a new user account for emma.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Created user account emma.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Creating a new user account for igor.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e cron[1414]: (CRON) INFO (pidfile fd = 3)
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e cron[1467]: (CRON) STARTUP (fork ok)
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e cron[1467]: (CRON) INFO (Running @reboot jobs)
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Created user account igor.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e acpid: starting up with netlink and the input layer
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e acpid: 1 rule loaded
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e acpid: waiting for events: event logging is off
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Created user account konstantinhaase.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Creating a new user account for aj.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e haveged: haveged starting up
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Created user account aj.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Creating a new user account for solarce.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [   11.370470] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [   11.383732] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Created user account solarce.
Aug  7 03:49:18 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Creating a new user account for asari.
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Created user account asari.
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Creating a new user account for bogdana.
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Created user account bogdana.
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Creating a new user account for konstantin.
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Created user account konstantin.
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [   11.545901] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [   11.551618] Bridge firewalling registered
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Creating a new user account for carmen.
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [   11.563487] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Created user account carmen.
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Creating a new user account for maria.
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [   11.624113] Initializing XFRM netlink socket
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [   11.630687] Netfilter messages via NETLINK v0.30.
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Created user account maria.
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [   11.636264] ctnetlink v0.93: registering with nfnetlink.
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e google-accounts: INFO Removing user packer.
Aug  7 03:49:19 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [   11.968075] floppy0: no floppy controllers found
Aug  7 03:49:25 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ntpdate[978]: adjust time server 169.254.169.254 offset 0.007766 sec
Aug  7 03:49:42 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ntpdate[1868]: adjust time server 169.254.169.254 offset 0.003544 sec
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ntpd[1904]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ntpd[1905]: proto: precision = 0.115 usec
---
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ntpd[1905]: Listen normally on 3 eth0 10.20.0.129 UDP 123
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ntpd[1905]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ntpd[1905]: peers refreshed
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ntpd[1905]: Listening on routing socket on fd #21 for interface updates
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [   41.545502] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e startup-script: INFO Found startup-script in metadata.
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e startup-script: INFO startup-script: job 1 at Tue Aug  7 06:59:00 2018
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e startup-script: INFO startup-script: Return code 0.
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e startup-script: INFO startup-script: Return code 0.
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e startup-script: INFO Finished running startup scripts.
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ec2: 
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ec2: #############################################################
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ec2: 1024 4b:8d:19:58:d6:12:2f:3a:67:af:66:94:61:9d:07:eb  root@travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e (DSA)
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ec2: 256 5c:8b:1e:21:78:bf:ae:a9:1f:45:39:19:f3:27:7e:e1  root@travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e (ECDSA)
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ec2: 256 15:f7:39:49:05:15:40:58:6a:99:aa:a8:31:25:6e:15  root@travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e (ED25519)
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ec2: 2048 ac:e6:1e:18:14:14:98:bc:4f:59:bd:27:d2:ef:bd:54  root@travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e (RSA)
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 03:49:49 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ec2: #############################################################
Aug  7 03:50:57 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [  109.050708] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 03:52:05 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [  177.604013] device veth9305cfd entered promiscuous mode
Aug  7 03:52:05 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [  177.681246] cgroup: docker-runc (4889) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 03:52:05 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [  177.681250] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 03:52:05 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [  177.744823] eth0: renamed from veth5d97b9e
Aug  7 03:52:05 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [  177.778303] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 03:52:05 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [  177.779115] docker0: port 1(veth9305cfd) entered forwarding state
Aug  7 03:52:05 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [  177.779130] docker0: port 1(veth9305cfd) entered forwarding state
Aug  7 03:52:05 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [  177.779154] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 03:52:08 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ntpd[1905]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  7 03:52:08 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ntpd[1905]: Listen normally on 6 docker0 fe80::42:a6ff:fe50:f8bd UDP 123
Aug  7 03:52:08 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ntpd[1905]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 03:52:08 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ntpd[1905]: peers refreshed
Aug  7 03:52:08 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e ntpd[1905]: new interface(s) found: waking up resolver
Aug  7 03:52:20 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [  192.780736] docker0: port 1(veth9305cfd) entered forwarding state
Aug  7 03:55:03 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [  355.305157] veth5d97b9e: renamed from eth0
Aug  7 03:55:03 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [  355.318496] docker0: port 1(veth9305cfd) entered disabled state
Aug  7 03:55:03 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [  355.357914] docker0: port 1(veth9305cfd) entered disabled state
Aug  7 03:55:03 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [  355.359604] device veth9305cfd left promiscuous mode
Aug  7 03:55:03 travis-job-d3d6215f-472f-43d3-9bd6-6b28052c6e3e kernel: [  355.359607] docker0: port 1(veth9305cfd) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:04da7cb3
---
travis_time:end:0cdea368:start=1533614104045629080,finish=1533614104050731762,duration=5102682
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14890d44
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01aac7dc
travis_time:start:01aac7dc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:07e7b09c
$ dmesg | grep -i kill
