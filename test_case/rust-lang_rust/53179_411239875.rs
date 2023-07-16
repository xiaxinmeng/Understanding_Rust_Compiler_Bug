plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/ae/dd/830ce78d3038e4a1d55d70a91823a6d4f563463be6515c3fda727a632aa2/awscli-1.15.73-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 33.8MB/s eta 0:00:01
    1% |▌                               | 20kB 1.4MB/s eta 0:00:01
    2% |▊                               | 30kB 1.7MB/s eta 0:00:01
    3% |█                               | 40kB 1.4MB/s eta 0:00:01
---
travis_time:start:tidy
tidy check
[00:03:57] * 554 error codes
[00:03:57] * highest error code: E0711
[00:03:58] tidy error: Found 1 features without a gate test.
[00:03:58] Expected a gate test for the feature 'wasm_target_feature'.
[00:03:58] Hint: create a failing test file named 'feature-gate-wasm_target_feature.rs'
[00:03:58]       in the 'ui' test suite, with its failures due to
[00:03:58]       missing usage of #![feature(wasm_target_feature)].
[00:03:58] Hint: If you already have such a test and don't want to rename it,
[00:03:58]       you can also add a // gate-test-wasm_target_feature line to the test file.
[00:03:58] some tidy checks failed
[00:03:58] 
[00:03:58] 
[00:03:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:58] 
[00:03:58] 
[00:03:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:58] Build completed unsuccessfully in 0:00:51
[00:03:58] Build completed unsuccessfully in 0:00:51
[00:03:58] Makefile:79: recipe for target 'tidy' failed
[00:03:58] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:068c2e10
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:11c1d858
$ sudo tail -n 500 /var/log/syslog
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] kvm-clock: using sched offset of 1929764064 cycles
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] Zone ranges:
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000]   Device   empty
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] Movable zone start for each node
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] Early memory node ranges
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] Policy zone: Normal
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] console [ttyS0] enabled
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.534443] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.538130] pid_max: default: 32768 minimum: 301
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.539843] ACPI: Core revision 20150930
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.548569] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.552346] Security Framework initialized
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.554289] Yama: becoming mindful.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.556808] AppArmor: AppArmor disabled by boot time parameter
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.561717] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.573665] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.580464] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.583087] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.586858] Initializing cgroup subsys io
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.588809] Initializing cgroup subsys memory
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.591199] Initializing cgroup subsys devices
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.593296] Initializing cgroup subsys freezer
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.595462] Initializing cgroup subsys net_cls
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.597948] Initializing cgroup subsys perf_event
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.600070] Initializing cgroup subsys net_prio
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.602492] Initializing cgroup subsys hugetlb
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.604952] Initializing cgroup subsys pids
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.606695] CPU: Physical Processor ID: 0
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.608141] CPU: Processor Core ID: 0
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.611144] mce: CPU supports 32 MCE banks
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.613850] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.615999] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.620312] Freeing SMP alternatives memory: 32K
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.630516] ftrace: allocating 32185 entries in 126 pages
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.684198] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.687009] smpboot: Max logical packages: 2
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.689131] x2apic enabled
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.692300] Switched APIC routing to physical x2apic.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.696861] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.804192] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.807772] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.813202] x86: Booting SMP configuration:
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.814957] .... node  #0, CPUs:      #1
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.817007] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.822658]  #2
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.823830] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.829918]  #3
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.830901] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.837019] x86: Booted up 1 node, 4 CPUs
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.838557] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.842531] devtmpfs: initialized
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.848337] evm: security.selinux
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.849731] evm: security.SMACK64
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.851130] evm: security.SMACK64EXEC
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.852259] evm: security.SMACK64TRANSMUTE
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.853646] evm: security.SMACK64MMAP
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.854882] evm: security.ima
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.855841] evm: security.capability
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.857565] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.861594] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.864320] pinctrl core: initialized pinctrl subsystem
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.866745] RTC time: 23:44:26, date: 08/07/18
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.869288] NET: Registered protocol family 16
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.880251] cpuidle: using governor ladder
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.892261] cpuidle: using governor menu
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.893841] PCCT header not found.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.895387] ACPI: bus type PCI registered
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.896615] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.899137] PCI: Using configuration type 1 for base access
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.913991] ACPI: Added _OSI(Module Device)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.915731] ACPI: Added _OSI(Processor Device)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.918105] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.920070] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.924448] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.951293] ACPI: Interpreter enabled
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.952644] ACPI: (supports S0 S3 S4 S5)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.954342] ACPI: Using IOAPIC for interrupt routing
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.955926] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.990161] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.993302] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.995849] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    0.998543] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.003700] PCI host bridge to bus 0000:00
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.005539] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.008057] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.010954] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.013563] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.016808] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.019197] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.019668] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.048943] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.077261] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.080423] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.092284] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.100882] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.126358] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.137005] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.145752] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.169704] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.174563] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.180124] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.184591] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.189160] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.212737] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.215523] vgaarb: loaded
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.217564] SCSI subsystem initialized
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.219153] libata version 3.00 loaded.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.219184] ACPI: bus type USB registered
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.220544] usbcore: registered new interface driver usbfs
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.222352] usbcore: registered new interface driver hub
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.224315] usbcore: registered new device driver usb
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.226524] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.229369] dmi: Firmware registration failed.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.231205] PCI: Using ACPI for IRQ routing
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.233034] PCI: pci_cache_line_size set to 64 bytes
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.233172] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.233175] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.233339] NetLabel: Initializing
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.235072] NetLabel:  domain hash size = 128
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.236548] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.238802] NetLabel:  unlabeled traffic allowed by default
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.241328] amd_nb: Cannot enumerate AMD northbridges
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.242960] clocksource: Switched to clocksource kvm-clock
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.253892] pnp: PnP ACPI init
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.254921] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.255008] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.255081] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.255136] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.255178] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.255217] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.255256] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.255442] pnp: PnP ACPI: found 7 devices
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.265233] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.268357] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.268361] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.268363] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.268364] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.268405] NET: Registered protocol family 2
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.270094] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.272701] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.275225] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.277495] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.279429] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.283017] NET: Registered protocol family 1
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.285189] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.287323] PCI: CLS 0 bytes, default 64
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    1.287405] Unpacking initramfs...
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.510079] Freeing initrd memory: 21432K
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.512401] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.516346] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.519699] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.522831] hw unit of domain pp0-core 2^-0 Joules
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.524674] hw unit of domain package 2^-0 Joules
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.526644] hw unit of domain dram 2^-0 Joules
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.528301] Scanning for low memory corruption every 60 seconds
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.531255] audit: initializing netlink subsys (disabled)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.533625] audit: type=2000 audit(1533685469.005:1): initialized
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.536280] Initialise system trusted keyring
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.539051] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.541678] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.544903] zbud: loaded
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.546332] VFS: Disk quotas dquot_6.6.0
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.548120] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.550594] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.553654] fuse init (API version 7.23)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.555468] Key type big_key registered
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.556883] Allocating IMA MOK and blacklist keyrings.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.562457] Key type asymmetric registered
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.565952] Asymmetric key parser 'x509' registered
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.568340] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.572391] io scheduler noop registered
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.574848] io scheduler deadline registered (default)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.577263] io scheduler cfq registered
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.579481] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.581627] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.584927] intel_idle: does not run on family 6 model 62
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.585041] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.589435] ACPI: Power Button [PWRF]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.591446] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.594617] ACPI: Sleep Button [SLPF]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.597161] GHES: HEST is not enabled!
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.601194] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.603913] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.612235] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.614349] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.624105] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.648023] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.673263] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.698180] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.723546] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.728722] Linux agpgart interface v0.103
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.732670] loop: module loaded
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.734323] libphy: Fixed MDIO Bus: probed
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.736034] tun: Universal TUN/TAP device driver, 1.6
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.737871] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.792766] PPP generic driver version 2.4.2
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.795011] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.798271] ehci-pci: EHCI PCI platform driver
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.799948] ehci-platform: EHCI generic platform driver
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.802121] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.805030] ohci-pci: OHCI PCI platform driver
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.807304] ohci-platform: OHCI generic platform driver
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.809320] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.811565] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.814785] i8042: Warning: Keylock active
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.817694] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.819424] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.821452] mousedev: PS/2 mouse device common for all mice
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.824439] rtc_cmos 00:00: RTC can wake from S4
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.827035] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.829726] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.832254] i2c /dev entries driver
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.833577] device-mapper: uevent: version 1.0.3
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.836763] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.840296] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.844033] NET: Registered protocol family 10
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.846540] NET: Registered protocol family 17
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.848719] Key type dns_resolver registered
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.850715] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.852634] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.854890] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.857189] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.859708] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.863076] registered taskstats version 1
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.864669] Loading compiled-in X.509 certificates
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.866873] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.870330] zswap: loaded using pool lzo/zbud
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.874561] Key type trusted registered
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.881763] Key type encrypted registered
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.883606] ima: No TPM chip found, activating TPM-bypass!
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.885545] evm: HMAC attrs: 0x1
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.887128]   Magic number: 14:662:756
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.888535] tty ttyS7: hash matches
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.889921] acpi LNXCPU:4e: hash matches
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.891502] rtc_cmos 00:00: setting system clock to 2018-08-07 23:44:29 UTC (1533685469)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.895486] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.897608] EDD information not available.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.899249] PM: Hibernation image not present or could not be loaded.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.901586] Freeing unused kernel memory: 1496K
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.903532] Write protecting the kernel read-only data: 14336k
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.906852] Freeing unused kernel memory: 1956K
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.909183] Freeing unused kernel memory: 92K
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.928243] systemd-udevd[118]: starting version 204
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    3.994426] scsi host0: Virtio SCSI HBA
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    4.001046] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    4.010666] AVX version of gcm_enc/dec engaged.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    4.012448] AES CTR mode by8 optimization enabled
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    4.028091] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    4.062716] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    4.062911] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    4.062912] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    4.063694] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    4.063696] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    4.063759] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    4.066060]  sda: sda1
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    4.067923] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    4.527114] tsc: Refined TSC clocksource calibration: 2499.799 MHz
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    4.529583] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24087c0d71c, max_idle_ns: 440795316476 ns
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    4.865105] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    6.983204] floppy0: no floppy controllers found
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.171016] raid6: sse2x1   gen()  8918 MB/s
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.239057] raid6: sse2x1   xor()  6682 MB/s
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.307016] raid6: sse2x2   gen() 10926 MB/s
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.375010] raid6: sse2x2   xor()  7606 MB/s
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.443012] raid6: sse2x4   gen() 12341 MB/s
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.511065] raid6: sse2x4   xor()  8650 MB/s
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.513389] raid6: using algorithm sse2x4 gen() 12341 MB/s
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.515628] raid6: .... xor() 8650 MB/s, rmw enabled
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.517354] raid6: using ssse3x2 recovery algorithm
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.521333] xor: automatically using best checksumming function:
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.562973]    avx       : 21530.000 MB/sec
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.579593] Btrfs loaded
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.633070] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.635381] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.699028] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.710908] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.711767] EXT4-fs (sda1): recovery complete
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.716935] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    8.929440] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    9.054669] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    9.108660] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    9.330887] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [    9.984547] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   10.151766] systemd-udevd[702]: starting version 204
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   10.270914] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   10.355590] ppdev: user-space parallel port driver
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   10.490178] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   10.552199] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   10.625569] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   10.804372] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   11.089096] random: mktemp: uninitialized urandom read (12 bytes read, 58 bits of entropy available)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   11.178895] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   11.274757] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   11.325159] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   11.608519] init: failsafe main process (1094) killed by TERM signal
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO Running set_multiqueue.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO Set channels for eth0 to 4.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 23:44:37 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Starting Google Accounts daemon.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Creating a new user account for me.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Created user account me.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Creating a new user account for henrik.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Created user account henrik.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Creating a new user account for emma.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Created user account emma.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Creating a new user account for igor.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   12.793739] random: nonblocking pool is initialized
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Created user account igor.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Created user account konstantinhaase.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Creating a new user account for aj.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Created user account aj.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Creating a new user account for solarce.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Created user account solarce.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Creating a new user account for asari.
Aug  7 23:44:38 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   13.069053] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   13.073742] Bridge firewalling registered
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Created user account asari.
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   13.088352] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Creating a new user account for bogdana.
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   13.127254] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Created user account bogdana.
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Creating a new user account for konstantin.
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   13.206027] Initializing XFRM netlink socket
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   13.215255] Netfilter messages via NETLINK v0.30.
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   13.218716] ctnetlink v0.93: registering with nfnetlink.
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Created user account konstantin.
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Creating a new user account for carmen.
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Created user account carmen.
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   13.327251] floppy0: no floppy controllers found
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Creating a new user account for maria.
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Created user account maria.
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 google-accounts: INFO Removing user packer.
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 cron[1706]: (CRON) INFO (pidfile fd = 3)
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 cron[1749]: (CRON) STARTUP (fork ok)
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 cron[1749]: (CRON) INFO (Running @reboot jobs)
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 acpid: starting up with netlink and the input layer
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 acpid: 1 rule loaded
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 acpid: waiting for events: event logging is off
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 haveged: haveged starting up
Aug  7 23:44:39 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   13.814164] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 23:45:02 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpdate[1870]: adjust time server 169.254.169.254 offset 0.005448 sec
Aug  7 23:45:09 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpd[1905]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 23:45:09 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpd[1907]: proto: precision = 0.130 usec
Aug  7 23:45:09 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpd[1907]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 23:45:09 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpd[1907]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 23:45:09 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpd[1907]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 23:45:09 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpd[1907]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  7 23:45:09 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpd[1907]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 23:45:09 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpd[1907]: Listen normally on 3 eth0 10.20.1.159 UDP 123
Aug  7 23:45:09 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpd[1907]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 23:45:09 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpd[1907]: peers refreshed
Aug  7 23:45:09 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpd[1907]: Listening on routing socket on fd #21 for interface updates
Aug  7 23:45:09 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   44.014689] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 23:45:10 travis-job-48015012-d83b-475d-ac31-28482dab9245 startup-script: INFO Found startup-script in metadata.
Aug  7 23:45:10 travis-job-48015012-d83b-475d-ac31-28482dab9245 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 23:45:10 travis-job-48015012-d83b-475d-ac31-28482dab9245 startup-script: INFO startup-script: job 1 at Wed Aug  8 02:55:00 2018
Aug  7 23:45:10 travis-job-48015012-d83b-475d-ac31-28482dab9245 startup-script: INFO startup-script: Return code 0.
Aug  7 23:45:10 travis-job-48015012-d83b-475d-ac31-28482dab9245 startup-script: INFO startup-script: Return code 0.
Aug  7 23:45:10 travis-job-48015012-d83b-475d-ac31-28482dab9245 startup-script: INFO Finished running startup scripts.
Aug  7 23:45:10 travis-job-48015012-d83b-475d-ac31-28482dab9245 ec2: 
Aug  7 23:45:10 travis-job-48015012-d83b-475d-ac31-28482dab9245 ec2: #############################################################
Aug  7 23:45:10 travis-job-48015012-d83b-475d-ac31-28482dab9245 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 23:45:10 travis-job-48015012-d83b-475d-ac31-28482dab9245 ec2: 1024 2a:32:1b:07:a4:63:36:36:47:5a:59:60:0c:6a:5e:61  root@travis-job-48015012-d83b-475d-ac31-28482dab9245 (DSA)
Aug  7 23:45:10 travis-job-48015012-d83b-475d-ac31-28482dab9245 ec2: 256 ad:a2:d9:53:8c:49:dd:bf:80:2f:e9:a1:04:9e:ca:d0  root@travis-job-48015012-d83b-475d-ac31-28482dab9245 (ECDSA)
Aug  7 23:45:10 travis-job-48015012-d83b-475d-ac31-28482dab9245 ec2: 256 85:20:4a:f0:2b:54:5f:b6:11:0d:d1:bb:c9:b7:a2:e7  root@travis-job-48015012-d83b-475d-ac31-28482dab9245 (ED25519)
Aug  7 23:45:10 travis-job-48015012-d83b-475d-ac31-28482dab9245 ec2: 2048 80:92:1f:d2:86:65:5b:6a:c8:d0:0e:35:24:02:cf:83  root@travis-job-48015012-d83b-475d-ac31-28482dab9245 (RSA)
Aug  7 23:45:10 travis-job-48015012-d83b-475d-ac31-28482dab9245 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 23:45:10 travis-job-48015012-d83b-475d-ac31-28482dab9245 ec2: #############################################################
Aug  7 23:45:42 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [   76.251293] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 23:46:47 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [  141.176452] device veth883c8d1 entered promiscuous mode
Aug  7 23:46:47 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [  141.176523] docker0: port 1(veth883c8d1) entered forwarding state
Aug  7 23:46:47 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [  141.176533] docker0: port 1(veth883c8d1) entered forwarding state
Aug  7 23:46:47 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [  141.176895] docker0: port 1(veth883c8d1) entered disabled state
Aug  7 23:46:47 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [  141.279977] cgroup: docker-runc (4891) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 23:46:47 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [  141.279981] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 23:46:47 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [  141.519377] eth0: renamed from vethc8647d7
Aug  7 23:46:47 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [  141.555185] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 23:46:47 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [  141.556438] docker0: port 1(veth883c8d1) entered forwarding state
Aug  7 23:46:47 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [  141.556452] docker0: port 1(veth883c8d1) entered forwarding state
Aug  7 23:46:47 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [  141.556473] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 23:46:50 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpd[1907]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  7 23:46:50 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpd[1907]: Listen normally on 6 docker0 fe80::42:b9ff:fe86:5307 UDP 123
Aug  7 23:46:50 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpd[1907]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 23:46:50 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpd[1907]: peers refreshed
Aug  7 23:46:50 travis-job-48015012-d83b-475d-ac31-28482dab9245 ntpd[1907]: new interface(s) found: waking up resolver
Aug  7 23:47:02 travis-job-48015012-d83b-475d-ac31-28482dab9245 kernel: [  156.599181] docker0: port 1(veth883c8d1) entered forwarding state
---
travis_time:end:021bb08c:start=1533685782686554548,finish=1533685782694417197,duration=7862649
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09cea5fe
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:078ca340
travis_time:start:078ca340
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:017c5e67
$ dmesg | grep -i kill
