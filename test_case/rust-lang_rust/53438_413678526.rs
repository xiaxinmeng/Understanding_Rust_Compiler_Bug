plain
tidy check
[00:04:52] * 556 error codes
[00:04:52] * highest error code: E0712
[00:04:53] * 220 features
[00:04:54] Stray file with UI testing output: "/checkout/src/test/ui/issue-27282-move-match-input-into-guard.stderr"
[00:04:54] Stray file with UI testing output: "/checkout/src/test/ui/issue-27282-mutate-before-diverging-arm-1.stderr"
[00:04:54] Stray file with UI testing output: "/checkout/src/test/ui/issue-27282-mutate-before-diverging-arm-2.stderr"
[00:04:54] some tidy checks failed
[00:04:54] 
[00:04:54] 
[00:04:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:54] 
[00:04:54] 
[00:04:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:54] Build completed unsuccessfully in 0:00:49
[00:04:54] Build completed unsuccessfully in 0:00:49
[00:04:54] make: *** [tidy] Error 1
[00:04:54] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f00721a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:1fc8a652
$ sudo tail -n 500 /var/log/syslog
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] kvm-clock: using sched offset of 1549433429 cycles
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] Zone ranges:
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000]   Device   empty
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] Movable zone start for each node
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] Early memory node ranges
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] Policy zone: Normal
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] console [ttyS0] enabled
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.323792] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.325238] pid_max: default: 32768 minimum: 301
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.326067] ACPI: Core revision 20150930
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.332851] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.333891] Security Framework initialized
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.334571] Yama: becoming mindful.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.335346] AppArmor: AppArmor disabled by boot time parameter
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.338210] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.348270] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.352933] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.354341] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.355773] Initializing cgroup subsys io
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.356585] Initializing cgroup subsys memory
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.357338] Initializing cgroup subsys devices
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.358113] Initializing cgroup subsys freezer
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.358886] Initializing cgroup subsys net_cls
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.359801] Initializing cgroup subsys perf_event
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.360665] Initializing cgroup subsys net_prio
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.361669] Initializing cgroup subsys hugetlb
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.362338] Initializing cgroup subsys pids
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.363454] CPU: Physical Processor ID: 0
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.364280] CPU: Processor Core ID: 0
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.365992] mce: CPU supports 32 MCE banks
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.367164] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.368122] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.371602] Freeing SMP alternatives memory: 32K
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.381943] ftrace: allocating 32185 entries in 126 pages
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.439306] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.440396] smpboot: Max logical packages: 2
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.441662] x2apic enabled
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.443899] Switched APIC routing to physical x2apic.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.447618] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.554841] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.556749] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.560145] x86: Booting SMP configuration:
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.560910] .... node  #0, CPUs:      #1
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.561868] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.566497]  #2
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.566943] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.571485]  #3
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.571979] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.576572] x86: Booted up 1 node, 4 CPUs
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.577396] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.580506] devtmpfs: initialized
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.585017] evm: security.selinux
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.585729] evm: security.SMACK64
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.586306] evm: security.SMACK64EXEC
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.587005] evm: security.SMACK64TRANSMUTE
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.587697] evm: security.SMACK64MMAP
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.588228] evm: security.ima
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.588683] evm: security.capability
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.589549] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.591164] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.592360] pinctrl core: initialized pinctrl subsystem
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.593394] RTC time: 20:29:32, date: 08/16/18
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.595256] NET: Registered protocol family 16
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.606904] cpuidle: using governor ladder
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.618880] cpuidle: using governor menu
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.619656] PCCT header not found.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.620260] ACPI: bus type PCI registered
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.620835] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.622011] PCI: Using configuration type 1 for base access
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.635916] ACPI: Added _OSI(Module Device)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.636655] ACPI: Added _OSI(Processor Device)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.637319] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.638244] ACPI: Added _OSI(Processor Aggregator Device)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.641854] ACPI: Executed 2 blocks of module-level executable AML code
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.666682] ACPI: Interpreter enabled
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.667402] ACPI: (supports S0 S3 S4 S5)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.667997] ACPI: Using IOAPIC for interrupt routing
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.668737] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.699611] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.700829] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.701905] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.703131] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.705667] PCI host bridge to bus 0000:00
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.706301] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.707292] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.708535] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.709821] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.710894] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.712223] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.712665] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.728744] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.744461] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.746203] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.752126] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.756789] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.772121] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.777506] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.781678] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.796536] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.798994] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.801454] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.803593] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.805731] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.826648] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.828008] vgaarb: loaded
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.828693] SCSI subsystem initialized
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.829540] libata version 3.00 loaded.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.829565] ACPI: bus type USB registered
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.830254] usbcore: registered new interface driver usbfs
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.831329] usbcore: registered new interface driver hub
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.832668] usbcore: registered new device driver usb
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.834010] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.835077] dmi: Firmware registration failed.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.836146] PCI: Using ACPI for IRQ routing
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.836806] PCI: pci_cache_line_size set to 64 bytes
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.836954] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.836956] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.837096] NetLabel: Initializing
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.837737] NetLabel:  domain hash size = 128
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.838569] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.839390] NetLabel:  unlabeled traffic allowed by default
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.840421] amd_nb: Cannot enumerate AMD northbridges
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.841305] clocksource: Switched to clocksource kvm-clock
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.849577] pnp: PnP ACPI init
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.850277] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.850349] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.850391] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.850439] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.850479] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.850519] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.850557] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.850721] pnp: PnP ACPI: found 7 devices
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.858465] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.859772] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.859774] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.859776] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.859777] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.859816] NET: Registered protocol family 2
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.860643] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.861938] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.863118] TCP: Hash tables configured (established 131072 bind 65536)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.864128] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.865026] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.866723] NET: Registered protocol family 1
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.867535] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.868467] PCI: CLS 0 bytes, default 64
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    0.868530] Unpacking initramfs...
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    2.983181] Freeing initrd memory: 21432K
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    2.984075] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    2.985180] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    2.986768] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    2.988188] hw unit of domain pp0-core 2^-0 Joules
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    2.988913] hw unit of domain package 2^-0 Joules
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    2.989660] hw unit of domain dram 2^-0 Joules
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    2.990426] Scanning for low memory corruption every 60 seconds
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    2.991964] audit: initializing netlink subsys (disabled)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    2.992830] audit: type=2000 audit(1534451374.579:1): initialized
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    2.994683] Initialise system trusted keyring
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    2.995824] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    2.996922] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    2.998901] zbud: loaded
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    2.999774] VFS: Disk quotas dquot_6.6.0
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.000621] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.001833] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.003274] fuse init (API version 7.23)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.004222] Key type big_key registered
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.004855] Allocating IMA MOK and blacklist keyrings.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.007008] Key type asymmetric registered
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.007676] Asymmetric key parser 'x509' registered
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.008620] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.009776] io scheduler noop registered
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.010356] io scheduler deadline registered (default)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.011139] io scheduler cfq registered
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.012047] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.012896] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.014119] intel_idle: does not run on family 6 model 62
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.014219] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.015622] ACPI: Power Button [PWRF]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.016777] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.018225] ACPI: Sleep Button [SLPF]
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.019401] GHES: HEST is not enabled!
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.022128] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.023182] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.027691] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.028883] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.033827] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.056753] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.080847] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.105860] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.130271] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.134400] Linux agpgart interface v0.103
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.138364] loop: module loaded
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.139243] libphy: Fixed MDIO Bus: probed
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.140164] tun: Universal TUN/TAP device driver, 1.6
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.141242] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.175784] PPP generic driver version 2.4.2
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.177150] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.178557] ehci-pci: EHCI PCI platform driver
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.179801] ehci-platform: EHCI generic platform driver
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.181040] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.182463] ohci-pci: OHCI PCI platform driver
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.183565] ohci-platform: OHCI generic platform driver
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.184712] uhci_hcd: USB Universal Host Controller Interface driver
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.186191] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.188201] i8042: Warning: Keylock active
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.190346] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.191308] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.192666] mousedev: PS/2 mouse device common for all mice
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.194327] rtc_cmos 00:00: RTC can wake from S4
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.195726] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.197360] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.198618] i2c /dev entries driver
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.199489] device-mapper: uevent: version 1.0.3
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.200603] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.202652] ledtrig-cpu: registered to indicate activity on CPUs
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.204632] NET: Registered protocol family 10
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.206168] NET: Registered protocol family 17
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.207240] Key type dns_resolver registered
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.208656] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.210716] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.212023] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.213326] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.214802] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.217119] registered taskstats version 1
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.218135] Loading compiled-in X.509 certificates
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.219826] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.222397] zswap: loaded using pool lzo/zbud
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.225606] Key type trusted registered
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.229896] Key type encrypted registered
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.230706] ima: No TPM chip found, activating TPM-bypass!
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.232201] evm: HMAC attrs: 0x1
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.233482]   Magic number: 14:263:498
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.234842] rtc_cmos 00:00: setting system clock to 2018-08-16 20:29:35 UTC (1534451375)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.237850] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.239701] EDD information not available.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.240799] PM: Hibernation image not present or could not be loaded.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.242237] Freeing unused kernel memory: 1496K
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.242928] Write protecting the kernel read-only data: 14336k
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.244729] Freeing unused kernel memory: 1956K
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.245634] Freeing unused kernel memory: 92K
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.259652] systemd-udevd[118]: starting version 204
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.318440] scsi host0: Virtio SCSI HBA
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.322427] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.326497] AVX version of gcm_enc/dec engaged.
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.327556] AES CTR mode by8 optimization enabled
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.358835] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.359839] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.361187] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.362646] sd 0:0:1:0: [sda] Write Protect is off
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.363490] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.363649] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.366958]  sda: sda1
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.368437] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.393978] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.989425] tsc: Refined TSC clocksource calibration: 2499.783 MHz
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    3.990632] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24086cd64eb, max_idle_ns: 440795280575 ns
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    4.231118] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    6.337555] floppy0: no floppy controllers found
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    7.509322] raid6: sse2x1   gen()  9084 MB/s
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    7.577323] raid6: sse2x1   xor()  6754 MB/s
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    7.645318] raid6: sse2x2   gen() 11256 MB/s
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    7.713321] raid6: sse2x2   xor()  7761 MB/s
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    7.781317] raid6: sse2x4   gen() 12546 MB/s
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    7.849315] raid6: sse2x4   xor()  8431 MB/s
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    7.850368] raid6: using algorithm sse2x4 gen() 12546 MB/s
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    7.851501] raid6: .... xor() 8431 MB/s, rmw enabled
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    7.852205] raid6: using ssse3x2 recovery algorithm
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    7.854604] xor: automatically using best checksumming function:
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    7.893318]    avx       : 22040.000 MB/sec
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    7.908451] Btrfs loaded
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    7.951453] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    7.952543] EXT4-fs (sda1): write access will be enabled during recovery
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    8.000871] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    8.012390] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    8.013836] EXT4-fs (sda1): recovery complete
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    8.019023] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    8.208932] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    8.315047] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    8.359080] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    8.540116] random: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    9.044248] random: cloud-init: uninitialized urandom read (32 bytes read, 45 bits of entropy available)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    9.171483] systemd-udevd[701]: starting version 204
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    9.262044] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    9.358094] ppdev: user-space parallel port driver
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    9.467323] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    9.516746] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    9.580666] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [    9.739800] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   10.004668] random: mktemp: uninitialized urandom read (12 bytes read, 60 bits of entropy available)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   10.070939] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   10.155375] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   10.193993] EXT4-fs (sda1): resized filesystem to 7864064
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   10.482801] init: failsafe main process (1092) killed by TERM signal
Aug 16 20:29:42 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO Running set_multiqueue.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO Set channels for eth0 to 4.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   11.185653] random: nonblocking pool is initialized
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Starting Google Accounts daemon.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Creating a new user account for me.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Created user account me.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Creating a new user account for henrik.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Created user account henrik.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Creating a new user account for emma.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Created user account emma.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Creating a new user account for igor.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Created user account igor.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 cron[1424]: (CRON) INFO (pidfile fd = 3)
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 cron[1471]: (CRON) STARTUP (fork ok)
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 cron[1471]: (CRON) INFO (Running @reboot jobs)
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Created user account konstantinhaase.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 acpid: starting up with netlink and the input layer
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 acpid: 1 rule loaded
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 acpid: waiting for events: event logging is off
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Creating a new user account for aj.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Created user account aj.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 haveged: haveged starting up
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Creating a new user account for solarce.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   11.705493] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Created user account solarce.
Aug 16 20:29:43 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   11.715166] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Creating a new user account for asari.
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Created user account asari.
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Creating a new user account for bogdana.
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Created user account bogdana.
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   11.832346] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   11.836525] Bridge firewalling registered
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Creating a new user account for konstantin.
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   11.849738] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Created user account konstantin.
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Creating a new user account for carmen.
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   11.914617] Initializing XFRM netlink socket
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   11.921145] Netfilter messages via NETLINK v0.30.
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   11.924330] ctnetlink v0.93: registering with nfnetlink.
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Created user account carmen.
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Creating a new user account for maria.
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Created user account maria.
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-accounts: INFO Removing user packer.
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 google-clock-skew: INFO Synced system time with hardware clock.
Aug 16 20:29:44 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   12.345491] floppy0: no floppy controllers found
Aug 16 20:30:06 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpdate[1841]: adjust time server 169.254.169.254 offset 0.012859 sec
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpd[1877]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpd[1878]: proto: precision = 0.101 usec
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpd[1878]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpd[1878]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpd[1878]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpd[1878]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpd[1878]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpd[1878]: Listen normally on 3 eth0 10.20.0.69 UDP 123
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpd[1878]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpd[1878]: peers refreshed
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpd[1878]: Listening on routing socket on fd #21 for interface updates
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   41.905467] init: plymouth-upstart-bridge main process ended, respawning
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 startup-script: INFO Found startup-script in metadata.
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 startup-script: INFO startup-script: job 1 at Thu Aug 16 23:40:00 2018
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 startup-script: INFO startup-script: Return code 0.
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 startup-script: INFO startup-script: Return code 0.
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 startup-script: INFO Finished running startup scripts.
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ec2: 
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ec2: #############################################################
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ec2: 1024 d6:76:8c:56:31:dc:69:7a:e1:05:6f:70:02:7d:ba:a5  root@travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 (DSA)
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ec2: 256 bc:76:d1:eb:fb:f7:b3:69:8b:bb:ed:ea:ac:cb:4b:46  root@travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 (ECDSA)
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ec2: 256 71:3d:be:4f:e6:25:ef:01:bc:15:bf:72:a4:29:99:65  root@travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 (ED25519)
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ec2: 2048 e8:db:3b:0d:0f:e2:48:f5:c2:39:bc:23:57:08:bc:be  root@travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 (RSA)
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 16 20:30:13 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ec2: #############################################################
Aug 16 20:30:46 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [   74.438030] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 16 20:32:30 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [  178.797901] device veth5a04153 entered promiscuous mode
Aug 16 20:32:30 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [  178.797963] docker0: port 1(veth5a04153) entered forwarding state
Aug 16 20:32:30 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [  178.797970] docker0: port 1(veth5a04153) entered forwarding state
Aug 16 20:32:30 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [  178.798617] docker0: port 1(veth5a04153) entered disabled state
Aug 16 20:32:30 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [  178.882977] cgroup: docker-runc (4962) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 16 20:32:30 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [  178.882980] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 16 20:32:30 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [  178.942536] eth0: renamed from vetha105d17
Aug 16 20:32:30 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [  178.973162] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 16 20:32:30 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [  178.974503] docker0: port 1(veth5a04153) entered forwarding state
Aug 16 20:32:30 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [  178.974529] docker0: port 1(veth5a04153) entered forwarding state
Aug 16 20:32:30 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [  178.974555] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 16 20:32:34 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpd[1878]: Listen normally on 5 docker0 fe80::42:98ff:feef:7694 UDP 123
Aug 16 20:32:34 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpd[1878]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 16 20:32:34 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpd[1878]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 16 20:32:34 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpd[1878]: peers refreshed
Aug 16 20:32:34 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 ntpd[1878]: new interface(s) found: waking up resolver
Aug 16 20:32:45 travis-job-2cbdd25a-45be-4527-967b-ad397a357c10 kernel: [  194.025753] docker0: port 1(veth5a04153) entered forwarding state
---
travis_time:end:0252f948:start=1534451742023722866,finish=1534451742032807437,duration=9084571
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:213e48a0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:341ade4b
travis_time:start:341ade4b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:138e805b
$ dmesg | grep -i kill
