plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/ae/dd/830ce78d3038e4a1d55d70a91823a6d4f563463be6515c3fda727a632aa2/awscli-1.15.73-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 14.4MB/s eta 0:00:01
    1% |▌                               | 20kB 1.2MB/s eta 0:00:02
    2% |▊                               | 30kB 1.3MB/s eta 0:00:01
    3% |█                               | 40kB 1.2MB/s eta 0:00:02
---

[00:04:06] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:07] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:62: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:90: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:98: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:121: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:209: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:244: TODO is deprecated; use FIXME
[00:04:08] some tidy checks failed
[00:04:08] 
[00:04:08] 
[00:04:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:08] 
[00:04:08] 
[00:04:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:08] Build completed unsuccessfully in 0:00:51
[00:04:08] Build completed unsuccessfully in 0:00:51
[00:04:08] Makefile:79: recipe for target 'tidy' failed
[00:04:08] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1888eaa0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:039099cc
$ sudo tail -n 500 /var/log/syslog
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] kvm-clock: using sched offset of 1648979483 cycles
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] Zone ranges:
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000]   Device   empty
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] Movable zone start for each node
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] Early memory node ranges
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] Policy zone: Normal
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] console [ttyS0] enabled
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.363560] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.365361] pid_max: default: 32768 minimum: 301
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.366653] ACPI: Core revision 20150930
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.373584] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.375313] Security Framework initialized
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.375990] Yama: becoming mindful.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.376706] AppArmor: AppArmor disabled by boot time parameter
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.380164] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.391530] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.398159] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.400117] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.402119] Initializing cgroup subsys io
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.403215] Initializing cgroup subsys memory
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.404599] Initializing cgroup subsys devices
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.405274] Initializing cgroup subsys freezer
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.406030] Initializing cgroup subsys net_cls
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.406893] Initializing cgroup subsys perf_event
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.407797] Initializing cgroup subsys net_prio
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.408956] Initializing cgroup subsys hugetlb
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.409790] Initializing cgroup subsys pids
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.410992] CPU: Physical Processor ID: 0
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.411795] CPU: Processor Core ID: 0
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.413991] mce: CPU supports 32 MCE banks
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.414845] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.415800] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.418880] Freeing SMP alternatives memory: 32K
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.427943] ftrace: allocating 32185 entries in 126 pages
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.476593] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.478621] smpboot: Max logical packages: 2
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.480455] x2apic enabled
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.482215] Switched APIC routing to physical x2apic.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.486579] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.594343] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.596525] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.600065] x86: Booting SMP configuration:
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.601045] .... node  #0, CPUs:      #1
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.602469] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.607942]  #2
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.608600] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.614248]  #3
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.614799] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.620052] x86: Booted up 1 node, 4 CPUs
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.620733] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.623807] devtmpfs: initialized
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.628510] evm: security.selinux
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.629304] evm: security.SMACK64
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.629838] evm: security.SMACK64EXEC
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.630500] evm: security.SMACK64TRANSMUTE
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.631234] evm: security.SMACK64MMAP
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.631989] evm: security.ima
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.632541] evm: security.capability
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.633707] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.636237] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.637640] pinctrl core: initialized pinctrl subsystem
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.638907] RTC time: 23:17:54, date: 08/07/18
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.640398] NET: Registered protocol family 16
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.646174] cpuidle: using governor ladder
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.658173] cpuidle: using governor menu
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.659064] PCCT header not found.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.660305] ACPI: bus type PCI registered
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.661044] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.662352] PCI: Using configuration type 1 for base access
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.675179] ACPI: Added _OSI(Module Device)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.676289] ACPI: Added _OSI(Processor Device)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.677009] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.678006] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.681992] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.706266] ACPI: Interpreter enabled
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.707074] ACPI: (supports S0 S3 S4 S5)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.708146] ACPI: Using IOAPIC for interrupt routing
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.709211] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.740092] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.741379] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.743352] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.744758] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.748041] PCI host bridge to bus 0000:00
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.749261] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.750557] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.752223] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.753698] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.755367] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.756585] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.757032] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.778274] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.803899] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.806033] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.815084] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.822885] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.842475] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.849967] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.856305] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.877707] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.881417] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.885013] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.888813] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.892343] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.914071] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.915377] vgaarb: loaded
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.916143] SCSI subsystem initialized
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.916843] libata version 3.00 loaded.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.916877] ACPI: bus type USB registered
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.917541] usbcore: registered new interface driver usbfs
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.918510] usbcore: registered new interface driver hub
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.919508] usbcore: registered new device driver usb
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.920774] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.923033] dmi: Firmware registration failed.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.924329] PCI: Using ACPI for IRQ routing
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.925474] PCI: pci_cache_line_size set to 64 bytes
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.925588] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.925590] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.925744] NetLabel: Initializing
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.926731] NetLabel:  domain hash size = 128
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.927831] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.929005] NetLabel:  unlabeled traffic allowed by default
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.931551] amd_nb: Cannot enumerate AMD northbridges
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.932820] clocksource: Switched to clocksource kvm-clock
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.941854] pnp: PnP ACPI init
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.942896] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.942963] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.943005] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.943054] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.943093] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.943134] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.943172] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.943337] pnp: PnP ACPI: found 7 devices
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.951357] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.954361] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.954364] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.954366] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.954368] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.954411] NET: Registered protocol family 2
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.955850] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.958452] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.960264] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.961829] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.963480] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.966144] NET: Registered protocol family 1
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.967128] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.968510] PCI: CLS 0 bytes, default 64
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    0.968578] Unpacking initramfs...
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.095825] Freeing initrd memory: 21432K
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.096755] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.098043] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.100091] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.101813] hw unit of domain pp0-core 2^-0 Joules
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.102893] hw unit of domain package 2^-0 Joules
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.103859] hw unit of domain dram 2^-0 Joules
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.104629] Scanning for low memory corruption every 60 seconds
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.106796] audit: initializing netlink subsys (disabled)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.107810] audit: type=2000 audit(1533683876.697:1): initialized
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.109294] Initialise system trusted keyring
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.110250] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.111361] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.113871] zbud: loaded
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.114632] VFS: Disk quotas dquot_6.6.0
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.115354] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.116941] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.118412] fuse init (API version 7.23)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.119533] Key type big_key registered
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.120191] Allocating IMA MOK and blacklist keyrings.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.122750] Key type asymmetric registered
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.123647] Asymmetric key parser 'x509' registered
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.124671] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.126254] io scheduler noop registered
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.127072] io scheduler deadline registered (default)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.128176] io scheduler cfq registered
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.129149] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.130890] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.132161] intel_idle: does not run on family 6 model 62
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.132259] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.134057] ACPI: Power Button [PWRF]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.135024] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.136335] ACPI: Sleep Button [SLPF]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.137868] GHES: HEST is not enabled!
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.140521] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.142182] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.148274] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.149588] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.155895] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.178773] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.202681] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.226871] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.250546] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.254780] Linux agpgart interface v0.103
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.259945] loop: module loaded
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.261184] libphy: Fixed MDIO Bus: probed
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.262417] tun: Universal TUN/TAP device driver, 1.6
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.264031] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.318257] PPP generic driver version 2.4.2
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.319468] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.321486] ehci-pci: EHCI PCI platform driver
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.322973] ehci-platform: EHCI generic platform driver
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.324428] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.326290] ohci-pci: OHCI PCI platform driver
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.327558] ohci-platform: OHCI generic platform driver
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.329023] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.330895] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.334191] i8042: Warning: Keylock active
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.336322] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.337948] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.339698] mousedev: PS/2 mouse device common for all mice
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.341701] rtc_cmos 00:00: RTC can wake from S4
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.343493] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.345822] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.347798] i2c /dev entries driver
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.348967] device-mapper: uevent: version 1.0.3
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.350279] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.352962] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.355843] NET: Registered protocol family 10
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.357438] NET: Registered protocol family 17
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.358450] Key type dns_resolver registered
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.360134] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.361763] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.363357] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.365280] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.366882] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.369549] registered taskstats version 1
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.370725] Loading compiled-in X.509 certificates
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.372712] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.375803] zswap: loaded using pool lzo/zbud
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.379312] Key type trusted registered
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.383968] Key type encrypted registered
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.385183] ima: No TPM chip found, activating TPM-bypass!
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.387030] evm: HMAC attrs: 0x1
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.388225]   Magic number: 14:697:301
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.389352] tty tty30: hash matches
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.390471] rtc_cmos 00:00: setting system clock to 2018-08-07 23:17:57 UTC (1533683877)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.392840] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.394640] EDD information not available.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.396078] PM: Hibernation image not present or could not be loaded.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.397669] Freeing unused kernel memory: 1496K
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.398540] Write protecting the kernel read-only data: 14336k
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.400753] Freeing unused kernel memory: 1956K
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.401961] Freeing unused kernel memory: 92K
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.416880] systemd-udevd[120]: starting version 204
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.487112] AVX version of gcm_enc/dec engaged.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.488312] AES CTR mode by8 optimization enabled
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.498645] scsi host0: Virtio SCSI HBA
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.503107] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.542401] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.542408] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.542409] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.542758] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.542761] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.542870] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.544668]  sda: sda1
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.545758] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    3.550972] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    4.100965] tsc: Refined TSC clocksource calibration: 2499.796 MHz
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    4.102436] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x240879331a3, max_idle_ns: 440795300520 ns
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    4.387017] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    6.513000] floppy0: no floppy controllers found
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    7.692858] raid6: sse2x1   gen()  9169 MB/s
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    7.760863] raid6: sse2x1   xor()  7049 MB/s
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    7.828882] raid6: sse2x2   gen() 11386 MB/s
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    7.896878] raid6: sse2x2   xor()  7864 MB/s
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    7.964878] raid6: sse2x4   gen() 12503 MB/s
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.032863] raid6: sse2x4   xor()  8609 MB/s
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.034013] raid6: using algorithm sse2x4 gen() 12503 MB/s
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.035328] raid6: .... xor() 8609 MB/s, rmw enabled
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.036539] raid6: using ssse3x2 recovery algorithm
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.039577] xor: automatically using best checksumming function:
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.080832]    avx       : 22082.000 MB/sec
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.097510] Btrfs loaded
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.149591] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.150857] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.226919] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.239554] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.240674] EXT4-fs (sda1): recovery complete
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.246626] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.450831] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.569777] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.623344] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    8.809168] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    9.366472] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    9.509975] systemd-udevd[704]: starting version 204
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    9.616404] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    9.719071] ppdev: user-space parallel port driver
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    9.811170] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    9.871047] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [    9.947256] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [   10.112335] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [   10.387968] random: mktemp: uninitialized urandom read (12 bytes read, 57 bits of entropy available)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [   10.464349] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [   10.540763] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [   10.571595] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [   10.975182] init: failsafe main process (1098) killed by TERM signal
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO Running set_multiqueue.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO Set channels for eth0 to 4.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Starting Google Accounts daemon.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Creating a new user account for me.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Created user account me.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Creating a new user account for henrik.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [   11.821894] random: nonblocking pool is initialized
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Created user account henrik.
Aug  7 23:18:05 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Creating a new user account for emma.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Created user account emma.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Creating a new user account for igor.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Created user account igor.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Created user account konstantinhaase.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Creating a new user account for aj.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Created user account aj.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Creating a new user account for solarce.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Created user account solarce.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Creating a new user account for asari.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Created user account asari.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Creating a new user account for bogdana.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe cron[1467]: (CRON) INFO (pidfile fd = 3)
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe cron[1515]: (CRON) STARTUP (fork ok)
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe cron[1515]: (CRON) INFO (Running @reboot jobs)
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Created user account bogdana.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Creating a new user account for konstantin.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe acpid: starting up with netlink and the input layer
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe acpid: 1 rule loaded
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe acpid: waiting for events: event logging is off
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Created user account konstantin.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [   12.350093] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Creating a new user account for carmen.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [   12.353852] Bridge firewalling registered
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe haveged: haveged starting up
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [   12.369169] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Created user account carmen.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [   12.401225] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Creating a new user account for maria.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [   12.472949] Initializing XFRM netlink socket
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Created user account maria.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [   12.483810] Netfilter messages via NETLINK v0.30.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [   12.487392] ctnetlink v0.93: registering with nfnetlink.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe google-accounts: INFO Removing user packer.
Aug  7 23:18:06 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [   12.681041] floppy0: no floppy controllers found
Aug  7 23:18:29 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ntpdate[1861]: adjust time server 169.254.169.254 offset 0.002766 sec
Aug  7 23:18:36 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ntpd[1896]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 23:18:36 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ntpd[1897]: proto: precision = 0.119 usec
Aug  7 23:18:36 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ntpd[1897]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
---
Aug  7 23:18:36 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 23:18:36 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe startup-script: INFO startup-script: job 1 at Wed Aug  8 02:28:00 2018
Aug  7 23:18:36 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe startup-script: INFO startup-script: Return code 0.
Aug  7 23:18:36 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe startup-script: INFO Finished running startup scripts.
Aug  7 23:18:36 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ec2: 
Aug  7 23:18:36 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ec2: #############################################################
Aug  7 23:18:36 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 23:18:36 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ec2: 1024 17:bc:a5:52:8f:47:d4:ba:13:32:11:90:5a:91:87:8e  root@travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe (DSA)
Aug  7 23:18:36 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ec2: 256 c3:02:99:e8:71:6f:e8:4b:ea:90:58:26:22:76:ad:b3  root@travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe (ECDSA)
Aug  7 23:18:36 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ec2: 256 b7:87:8e:7b:23:7e:54:1e:e1:3c:dc:0e:b8:eb:87:5c  root@travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe (ED25519)
Aug  7 23:18:36 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ec2: 2048 49:2a:5d:63:2f:af:a7:b1:c4:2b:3a:71:fd:cf:1b:9c  root@travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe (RSA)
Aug  7 23:18:36 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 23:18:36 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ec2: #############################################################
Aug  7 23:20:10 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  136.993498] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 23:21:08 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  194.379883] device veth9997098 entered promiscuous mode
Aug  7 23:21:08 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  194.379933] docker0: port 1(veth9997098) entered forwarding state
Aug  7 23:21:08 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  194.379948] docker0: port 1(veth9997098) entered forwarding state
Aug  7 23:21:08 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  194.380439] docker0: port 1(veth9997098) entered disabled state
Aug  7 23:21:08 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  194.498583] cgroup: docker-runc (4893) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 23:21:08 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  194.498586] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 23:21:08 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  194.577989] eth0: renamed from veth0cd17bc
Aug  7 23:21:08 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  194.616623] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 23:21:08 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  194.618415] docker0: port 1(veth9997098) entered forwarding state
Aug  7 23:21:08 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  194.618438] docker0: port 1(veth9997098) entered forwarding state
Aug  7 23:21:08 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  194.618454] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 23:21:12 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ntpd[1897]: Listen normally on 5 docker0 fe80::42:5ff:fe9f:5fae UDP 123
Aug  7 23:21:12 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ntpd[1897]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  7 23:21:12 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ntpd[1897]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 23:21:12 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ntpd[1897]: peers refreshed
Aug  7 23:21:12 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe ntpd[1897]: new interface(s) found: waking up resolver
Aug  7 23:21:23 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  209.645730] docker0: port 1(veth9997098) entered forwarding state
Aug  7 23:24:20 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  386.737648] veth0cd17bc: renamed from eth0
Aug  7 23:24:20 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  386.758560] docker0: port 1(veth9997098) entered disabled state
Aug  7 23:24:20 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  386.807880] docker0: port 1(veth9997098) entered disabled state
Aug  7 23:24:20 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  386.809592] device veth9997098 left promiscuous mode
Aug  7 23:24:20 travis-job-a2caac7b-95cb-46f7-863e-79bbb88951fe kernel: [  386.809594] docker0: port 1(veth9997098) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:0dd2b7d6
---
travis_time:end:17f4418c:start=1533684261169309278,finish=1533684261177664663,duration=8355385
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07c3c790
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0262ab0e
travis_time:start:0262ab0e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:12c45ba0
$ dmesg | grep -i kill
