plain
    100% |████████████████████████████████| 61kB 4.5MB/s 
Collecting botocore==1.10.75 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/91/cd/629d59d751a6bac5e02963e61e5b449619373c6c0e54e8d5af7f7215e081/botocore-1.10.75-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 42.9MB/s eta 0:00:01
    0% |▏                               | 20kB 14.5MB/s eta 0:00:01
    0% |▏                               | 30kB 18.8MB/s eta 0:00:01
    0% |▎                               | 40kB 6.8MB/s eta 0:00:01
---

[00:06:21] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:06:21] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1267: line longer than 100 chars
[00:06:21] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1587: line longer than 100 chars
[00:06:23] some tidy checks failed
[00:06:23] 
[00:06:23] 
[00:06:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:06:23] 
[00:06:23] 
[00:06:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:06:23] Build completed unsuccessfully in 0:00:50
[00:06:23] Build completed unsuccessfully in 0:00:50
[00:06:23] Makefile:79: recipe for target 'tidy' failed
[00:06:23] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:021a4584
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0af728cc
$ sudo tail -n 500 /var/log/syslog
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] kvm-clock: using sched offset of 1648230008 cycles
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] Zone ranges:
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000]   Device   empty
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] Movable zone start for each node
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] Early memory node ranges
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] Policy zone: Normal
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] Hierarchical RCU implementation.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] console [ttyS0] enabled
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.556555] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.560051] pid_max: default: 32768 minimum: 301
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.562680] ACPI: Core revision 20150930
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.571691] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.574613] Security Framework initialized
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.575978] Yama: becoming mindful.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.577905] AppArmor: AppArmor disabled by boot time parameter
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.583175] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.595690] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.602213] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.606161] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.609996] Initializing cgroup subsys io
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.611901] Initializing cgroup subsys memory
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.614182] Initializing cgroup subsys devices
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.616471] Initializing cgroup subsys freezer
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.618466] Initializing cgroup subsys net_cls
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.620361] Initializing cgroup subsys perf_event
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.623223] Initializing cgroup subsys net_prio
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.625024] Initializing cgroup subsys hugetlb
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.627760] Initializing cgroup subsys pids
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.629768] CPU: Physical Processor ID: 0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.632309] CPU: Processor Core ID: 0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.636050] mce: CPU supports 32 MCE banks
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.638569] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.641473] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.648710] Freeing SMP alternatives memory: 32K
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.661240] ftrace: allocating 32185 entries in 126 pages
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.722806] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.726151] smpboot: Max logical packages: 2
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.728821] x2apic enabled
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.731940] Switched APIC routing to physical x2apic.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.738648] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.846788] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.855065] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.862818] x86: Booting SMP configuration:
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.865843] .... node  #0, CPUs:      #1
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.867790] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.874947]  #2
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.876189] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.884850]  #3
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.886196] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.892982] x86: Booted up 1 node, 4 CPUs
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.896387] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.900921] devtmpfs: initialized
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.906713] evm: security.selinux
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.908145] evm: security.SMACK64
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.910185] evm: security.SMACK64EXEC
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.911901] evm: security.SMACK64TRANSMUTE
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.914161] evm: security.SMACK64MMAP
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.915646] evm: security.ima
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.917047] evm: security.capability
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.919670] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.924812] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.927478] pinctrl core: initialized pinctrl subsystem
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.930165] RTC time: 22:09:18, date: 08/10/18
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.935062] NET: Registered protocol family 16
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.946897] cpuidle: using governor ladder
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.962902] cpuidle: using governor menu
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.964404] PCCT header not found.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.965546] ACPI: bus type PCI registered
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.967392] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.971058] PCI: Using configuration type 1 for base access
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.988641] ACPI: Added _OSI(Module Device)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.990089] ACPI: Added _OSI(Processor Device)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.992424] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.994223] ACPI: Added _OSI(Processor Aggregator Device)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    0.999817] ACPI: Executed 2 blocks of module-level executable AML code
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.026415] ACPI: Interpreter enabled
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.028211] ACPI: (supports S0 S3 S4 S5)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.030273] ACPI: Using IOAPIC for interrupt routing
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.032114] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.065164] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.068992] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.072990] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.076013] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.082144] PCI host bridge to bus 0000:00
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.084256] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.087351] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.090863] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.095053] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.097942] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.100182] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.100691] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.124295] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.152491] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.156472] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.167353] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.175508] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.197617] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.207699] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.215340] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.236368] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.241192] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.245288] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.250102] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.254717] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.277291] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.280371] vgaarb: loaded
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.281831] SCSI subsystem initialized
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.284591] libata version 3.00 loaded.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.284621] ACPI: bus type USB registered
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.286594] usbcore: registered new interface driver usbfs
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.288954] usbcore: registered new interface driver hub
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.291052] usbcore: registered new device driver usb
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.294236] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.296692] dmi: Firmware registration failed.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.298736] PCI: Using ACPI for IRQ routing
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.300296] PCI: pci_cache_line_size set to 64 bytes
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.300409] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.300411] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.300531] NetLabel: Initializing
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.301596] NetLabel:  domain hash size = 128
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.302998] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.305700] NetLabel:  unlabeled traffic allowed by default
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.308653] amd_nb: Cannot enumerate AMD northbridges
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.311259] clocksource: Switched to clocksource kvm-clock
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.320730] pnp: PnP ACPI init
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.322502] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.322599] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.322643] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.322695] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.322735] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.322773] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.322812] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.322978] pnp: PnP ACPI: found 7 devices
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.332790] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.337079] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.337082] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.337084] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.337085] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.337124] NET: Registered protocol family 2
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.339869] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.343645] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.346600] TCP: Hash tables configured (established 131072 bind 65536)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.350276] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.352936] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.356997] NET: Registered protocol family 1
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.359443] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.362166] PCI: CLS 0 bytes, default 64
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    1.362229] Unpacking initramfs...
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.545870] Freeing initrd memory: 21432K
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.547957] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.550651] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.554483] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.559592] hw unit of domain pp0-core 2^-0 Joules
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.561781] hw unit of domain package 2^-0 Joules
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.563584] hw unit of domain dram 2^-16 Joules
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.565510] Scanning for low memory corruption every 60 seconds
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.568461] audit: initializing netlink subsys (disabled)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.571111] audit: type=2000 audit(1533938960.983:1): initialized
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.575076] Initialise system trusted keyring
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.577418] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.582484] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.586033] zbud: loaded
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.587759] VFS: Disk quotas dquot_6.6.0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.589556] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.593472] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.597254] fuse init (API version 7.23)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.599546] Key type big_key registered
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.601451] Allocating IMA MOK and blacklist keyrings.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.608647] Key type asymmetric registered
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.610413] Asymmetric key parser 'x509' registered
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.612772] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.616002] io scheduler noop registered
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.618045] io scheduler deadline registered (default)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.620606] io scheduler cfq registered
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.622042] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.624838] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.628263] intel_idle: does not run on family 6 model 63
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.628398] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.631796] ACPI: Power Button [PWRF]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.633321] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.636316] ACPI: Sleep Button [SLPF]
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.638640] GHES: HEST is not enabled!
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.643585] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.646827] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.656646] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.660094] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.671757] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.697030] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.721949] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.747491] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.772953] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.778331] Linux agpgart interface v0.103
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.782960] loop: module loaded
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.784721] libphy: Fixed MDIO Bus: probed
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.786478] tun: Universal TUN/TAP device driver, 1.6
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.789276] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.844911] PPP generic driver version 2.4.2
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.846900] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.850224] ehci-pci: EHCI PCI platform driver
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.852072] ehci-platform: EHCI generic platform driver
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.855247] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.857591] ohci-pci: OHCI PCI platform driver
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.859955] ohci-platform: OHCI generic platform driver
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.862093] uhci_hcd: USB Universal Host Controller Interface driver
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.865170] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.869747] i8042: Warning: Keylock active
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.872723] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.874526] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.877182] mousedev: PS/2 mouse device common for all mice
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.880077] rtc_cmos 00:00: RTC can wake from S4
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.882085] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.884936] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.888003] i2c /dev entries driver
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.889956] device-mapper: uevent: version 1.0.3
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.892285] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.895545] ledtrig-cpu: registered to indicate activity on CPUs
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.899241] NET: Registered protocol family 10
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.901943] NET: Registered protocol family 17
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.904866] Key type dns_resolver registered
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.906904] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.909505] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.911527] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.913549] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.916650] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.920884] registered taskstats version 1
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.922447] Loading compiled-in X.509 certificates
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.925021] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.929167] zswap: loaded using pool lzo/zbud
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.934063] Key type trusted registered
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.940790] Key type encrypted registered
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.942808] ima: No TPM chip found, activating TPM-bypass!
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.945155] evm: HMAC attrs: 0x1
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.947073]   Magic number: 14:713:198
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.949176] rtc_cmos 00:00: setting system clock to 2018-08-10 22:09:21 UTC (1533938961)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.952824] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.955050] EDD information not available.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.956792] PM: Hibernation image not present or could not be loaded.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.958669] Freeing unused kernel memory: 1496K
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.960760] Write protecting the kernel read-only data: 14336k
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.964401] Freeing unused kernel memory: 1956K
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.966724] Freeing unused kernel memory: 92K
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    3.986696] systemd-udevd[118]: starting version 204
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    4.058817] scsi host0: Virtio SCSI HBA
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    4.064628] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    4.073098] AVX2 version of gcm_enc/dec engaged.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    4.074790] AES CTR mode by8 optimization enabled
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    4.083703] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    4.123375] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    4.123481] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    4.123483] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    4.123883] sd 0:0:1:0: [sda] Write Protect is off
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    4.123885] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    4.123948] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    4.125806]  sda: sda1
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    4.126747] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    4.563433] tsc: Refined TSC clocksource calibration: 2299.805 MHz
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    4.567009] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x21267d10f42, max_idle_ns: 440795275523 ns
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    4.920539] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    7.031428] floppy0: no floppy controllers found
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.191321] raid6: sse2x1   gen()  9002 MB/s
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.259320] raid6: sse2x1   xor()  6677 MB/s
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.327314] raid6: sse2x2   gen() 11203 MB/s
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.395329] raid6: sse2x2   xor()  7763 MB/s
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.463372] raid6: sse2x4   gen() 12489 MB/s
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.531339] raid6: sse2x4   xor()  8793 MB/s
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.599374] raid6: avx2x1   gen() 16581 MB/s
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.667313] raid6: avx2x2   gen() 20311 MB/s
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.735310] raid6: avx2x4   gen() 20915 MB/s
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.736750] raid6: using algorithm avx2x4 gen() 20915 MB/s
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.738615] raid6: using avx2x2 recovery algorithm
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.742162] xor: automatically using best checksumming function:
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.783324]    avx       : 21969.000 MB/sec
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.799487] Btrfs loaded
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.858964] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.861159] EXT4-fs (sda1): write access will be enabled during recovery
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.978636] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.988224] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.990290] EXT4-fs (sda1): recovery complete
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    8.996993] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    9.248650] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    9.400633] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    9.456127] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [    9.688991] random: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   10.354167] random: cloud-init: uninitialized urandom read (32 bytes read, 45 bits of entropy available)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   10.502414] systemd-udevd[701]: starting version 204
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   10.632740] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   10.698755] intel_rapl: no valid rapl domains found in package 0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   10.760371] intel_rapl: no valid rapl domains found in package 0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   10.768894] ppdev: user-space parallel port driver
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   10.819039] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   10.879018] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   10.950668] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   11.126513] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   11.377572] random: mktemp: uninitialized urandom read (12 bytes read, 60 bits of entropy available)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   11.452454] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   11.529122] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   11.570314] EXT4-fs (sda1): resized filesystem to 7864064
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   11.926828] init: failsafe main process (1092) killed by TERM signal
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO Running set_multiqueue.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO Set channels for eth0 to 4.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 10 22:09:29 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   12.666581] random: nonblocking pool is initialized
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Starting Google Accounts daemon.
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Creating a new user account for me.
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Created user account me.
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Creating a new user account for henrik.
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Created user account henrik.
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Creating a new user account for emma.
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd cron[1405]: (CRON) INFO (pidfile fd = 3)
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd cron[1455]: (CRON) STARTUP (fork ok)
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd cron[1455]: (CRON) INFO (Running @reboot jobs)
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Created user account emma.
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Creating a new user account for igor.
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd acpid: starting up with netlink and the input layer
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd acpid: 1 rule loaded
Aug 10 22:09:30 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd acpid: waiting for events: event logging is off
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-clock-skew: INFO Synced system time with hardware clock.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Created user account igor.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd haveged: haveged starting up
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Created user account konstantinhaase.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Creating a new user account for aj.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   13.218849] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   13.231369] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Created user account aj.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Creating a new user account for solarce.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Created user account solarce.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   13.336253] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   13.341431] Bridge firewalling registered
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Creating a new user account for asari.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   13.356470] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Created user account asari.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Creating a new user account for bogdana.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   13.449948] Initializing XFRM netlink socket
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Created user account bogdana.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   13.462980] Netfilter messages via NETLINK v0.30.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   13.465919] ctnetlink v0.93: registering with nfnetlink.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Creating a new user account for konstantin.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Created user account konstantin.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Creating a new user account for carmen.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Created user account carmen.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Creating a new user account for maria.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Created user account maria.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd google-accounts: INFO Removing user packer.
Aug 10 22:09:31 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   13.727397] floppy0: no floppy controllers found
Aug 10 22:09:54 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpdate[1868]: adjust time server 169.254.169.254 offset 0.002845 sec
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpd[1904]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpd[1905]: proto: precision = 0.103 usec
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpd[1905]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpd[1905]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpd[1905]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpd[1905]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpd[1905]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpd[1905]: Listen normally on 3 eth0 10.20.0.2 UDP 123
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpd[1905]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpd[1905]: peers refreshed
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpd[1905]: Listening on routing socket on fd #21 for interface updates
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   43.437465] init: plymouth-upstart-bridge main process ended, respawning
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd startup-script: INFO Found startup-script in metadata.
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd startup-script: INFO startup-script: job 1 at Sat Aug 11 01:20:00 2018
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd startup-script: INFO startup-script: Return code 0.
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd startup-script: INFO startup-script: Return code 0.
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd startup-script: INFO Finished running startup scripts.
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ec2: 
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ec2: #############################################################
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ec2: 1024 27:e9:c5:bb:3a:a2:86:52:3a:17:87:a5:27:10:7a:a4  root@travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd (DSA)
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ec2: 256 9a:af:58:88:c9:82:01:af:14:76:b9:c6:59:bf:a8:a9  root@travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd (ECDSA)
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ec2: 256 f0:39:dc:f6:9d:97:c0:21:cf:19:36:ac:d7:8f:f6:f3  root@travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd (ED25519)
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ec2: 2048 fa:ea:db:e2:18:19:76:5a:0e:2d:c3:28:5c:d7:b2:38  root@travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd (RSA)
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 10 22:10:01 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ec2: #############################################################
Aug 10 22:10:33 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [   75.536641] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 10 22:13:10 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [  232.410779] device vethd0d66eb entered promiscuous mode
Aug 10 22:13:10 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [  232.410838] docker0: port 1(vethd0d66eb) entered forwarding state
Aug 10 22:13:10 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [  232.410845] docker0: port 1(vethd0d66eb) entered forwarding state
Aug 10 22:13:10 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [  232.411453] docker0: port 1(vethd0d66eb) entered disabled state
Aug 10 22:13:10 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [  232.516534] cgroup: docker-runc (4898) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 10 22:13:10 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [  232.516537] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 10 22:13:10 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [  232.588388] eth0: renamed from vethe042de4
Aug 10 22:13:10 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [  232.625303] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 10 22:13:10 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [  232.626570] docker0: port 1(vethd0d66eb) entered forwarding state
Aug 10 22:13:10 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [  232.626590] docker0: port 1(vethd0d66eb) entered forwarding state
Aug 10 22:13:10 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [  232.626609] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 10 22:13:13 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpd[1905]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 10 22:13:13 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpd[1905]: Listen normally on 6 docker0 fe80::42:61ff:fe11:6497 UDP 123
Aug 10 22:13:13 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpd[1905]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 10 22:13:13 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpd[1905]: peers refreshed
Aug 10 22:13:13 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd ntpd[1905]: new interface(s) found: waking up resolver
Aug 10 22:13:25 travis-job-79eed215-e8b8-44d2-9f47-c024f17ca3bd kernel: [  247.635829] docker0: port 1(vethd0d66eb) entered forwarding state
---
travis_time:end:0e1af2ca:start=1533939418016882533,finish=1533939418024996641,duration=8114108
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0018b1e0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a045ef5
travis_time:start:1a045ef5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0c23dac0
$ dmesg | grep -i kill
