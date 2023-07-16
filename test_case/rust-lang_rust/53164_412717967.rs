plain
[01:59:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[01:59:46] expected success, got: exit code: 101
[01:59:46] 
[01:59:46] 
[01:59:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:59:46] Build completed unsuccessfully in 0:52:33
[01:59:46] make: *** [check-aux] Error 1
[01:59:46] Makefile:60: recipe for target 'check-aux' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ccd705b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:086471b4
$ sudo tail -n 500 /var/log/syslog
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] kvm-clock: using sched offset of 1917154266 cycles
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] Zone ranges:
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000]   Device   empty
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] Movable zone start for each node
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] Early memory node ranges
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] Policy zone: Normal
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] Hierarchical RCU implementation.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] console [ttyS0] enabled
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.743277] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.749199] pid_max: default: 32768 minimum: 301
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.752096] ACPI: Core revision 20150930
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.760000] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.764688] Security Framework initialized
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.770288] Yama: becoming mindful.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.772753] AppArmor: AppArmor disabled by boot time parameter
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.777723] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.792090] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.800154] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.804662] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.809473] Initializing cgroup subsys io
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.812302] Initializing cgroup subsys memory
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.815559] Initializing cgroup subsys devices
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.818112] Initializing cgroup subsys freezer
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.821802] Initializing cgroup subsys net_cls
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.826054] Initializing cgroup subsys perf_event
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.828954] Initializing cgroup subsys net_prio
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.832019] Initializing cgroup subsys hugetlb
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.836679] Initializing cgroup subsys pids
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.840049] CPU: Physical Processor ID: 0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.843285] CPU: Processor Core ID: 0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.845935] mce: CPU supports 32 MCE banks
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.849140] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.853348] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.858839] Freeing SMP alternatives memory: 32K
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.872430] ftrace: allocating 32185 entries in 126 pages
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.929019] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.935433] smpboot: Max logical packages: 2
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.939513] x2apic enabled
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.943510] Switched APIC routing to physical x2apic.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    0.951141] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.060344] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.067398] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.074734] x86: Booting SMP configuration:
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.078101] .... node  #0, CPUs:      #1
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.080842] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.088920]  #2
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.090235] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.097027]  #3
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.098804] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.105510] x86: Booted up 1 node, 4 CPUs
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.107625] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.114298] devtmpfs: initialized
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.120949] evm: security.selinux
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.124130] evm: security.SMACK64
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.127106] evm: security.SMACK64EXEC
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.131064] evm: security.SMACK64TRANSMUTE
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.136752] evm: security.SMACK64MMAP
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.141827] evm: security.ima
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.145219] evm: security.capability
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.150026] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.156856] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.161221] pinctrl core: initialized pinctrl subsystem
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.165484] RTC time: 23:01:08, date: 08/13/18
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.169043] NET: Registered protocol family 16
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.180446] cpuidle: using governor ladder
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.192497] cpuidle: using governor menu
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.195851] PCCT header not found.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.198548] ACPI: bus type PCI registered
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.200834] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.205247] PCI: Using configuration type 1 for base access
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.222097] ACPI: Added _OSI(Module Device)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.224712] ACPI: Added _OSI(Processor Device)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.227779] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.232219] ACPI: Added _OSI(Processor Aggregator Device)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.238750] ACPI: Executed 2 blocks of module-level executable AML code
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.266430] ACPI: Interpreter enabled
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.268651] ACPI: (supports S0 S3 S4 S5)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.271144] ACPI: Using IOAPIC for interrupt routing
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.274436] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.310403] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.314859] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.320718] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.326439] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.337695] PCI host bridge to bus 0000:00
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.342587] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.349243] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.355414] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.363239] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.369881] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.375032] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.375476] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.416998] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.457063] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.463176] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.476130] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.487351] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.520646] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.532278] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.543121] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.572739] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.580384] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.589888] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.597776] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.605174] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.629278] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.632649] vgaarb: loaded
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.635039] SCSI subsystem initialized
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.637618] libata version 3.00 loaded.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.637662] ACPI: bus type USB registered
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.639996] usbcore: registered new interface driver usbfs
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.642837] usbcore: registered new interface driver hub
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.645906] usbcore: registered new device driver usb
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.649946] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.656018] dmi: Firmware registration failed.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.659479] PCI: Using ACPI for IRQ routing
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.662536] PCI: pci_cache_line_size set to 64 bytes
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.662639] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.662641] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.662779] NetLabel: Initializing
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.665201] NetLabel:  domain hash size = 128
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.669217] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.673582] NetLabel:  unlabeled traffic allowed by default
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.677158] amd_nb: Cannot enumerate AMD northbridges
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.681548] clocksource: Switched to clocksource kvm-clock
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.691906] pnp: PnP ACPI init
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.694678] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.694756] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.694802] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.694854] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.694896] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.694988] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.695037] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.695205] pnp: PnP ACPI: found 7 devices
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.706186] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.710890] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.710892] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.710894] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.710895] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.710933] NET: Registered protocol family 2
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.713116] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.717175] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.720785] TCP: Hash tables configured (established 131072 bind 65536)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.724317] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.727364] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.731124] NET: Registered protocol family 1
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.732907] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.735235] PCI: CLS 0 bytes, default 64
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    1.735292] Unpacking initramfs...
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.789722] Freeing initrd memory: 21432K
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.792529] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.795941] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.801737] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.807048] hw unit of domain pp0-core 2^-0 Joules
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.810520] hw unit of domain package 2^-0 Joules
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.813408] hw unit of domain dram 2^-0 Joules
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.815949] Scanning for low memory corruption every 60 seconds
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.820128] audit: initializing netlink subsys (disabled)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.823670] audit: type=2000 audit(1534201270.698:1): initialized
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.828602] Initialise system trusted keyring
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.832394] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.836912] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.842150] zbud: loaded
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.844643] VFS: Disk quotas dquot_6.6.0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.846880] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.851830] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.856210] fuse init (API version 7.23)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.858425] Key type big_key registered
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.860585] Allocating IMA MOK and blacklist keyrings.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.868766] Key type asymmetric registered
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.871491] Asymmetric key parser 'x509' registered
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.874467] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.879124] io scheduler noop registered
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.881051] io scheduler deadline registered (default)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.884462] io scheduler cfq registered
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.888081] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.891221] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.894905] intel_idle: does not run on family 6 model 45
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.895027] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.898751] ACPI: Power Button [PWRF]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.900379] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.904365] ACPI: Sleep Button [SLPF]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.907858] GHES: HEST is not enabled!
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.913675] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.918383] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.931651] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.934953] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.947134] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    3.972345] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.001346] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.028211] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.059803] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.067624] Linux agpgart interface v0.103
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.075280] loop: module loaded
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.079062] libphy: Fixed MDIO Bus: probed
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.081802] tun: Universal TUN/TAP device driver, 1.6
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.084338] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.140185] PPP generic driver version 2.4.2
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.144020] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.149067] ehci-pci: EHCI PCI platform driver
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.152417] ehci-platform: EHCI generic platform driver
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.155498] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.158767] ohci-pci: OHCI PCI platform driver
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.161502] ohci-platform: OHCI generic platform driver
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.164595] uhci_hcd: USB Universal Host Controller Interface driver
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.168855] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.173461] i8042: Warning: Keylock active
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.177091] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.181048] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.185047] mousedev: PS/2 mouse device common for all mice
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.189918] rtc_cmos 00:00: RTC can wake from S4
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.193370] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.200114] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.203866] i2c /dev entries driver
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.206657] device-mapper: uevent: version 1.0.3
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.210435] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.216534] ledtrig-cpu: registered to indicate activity on CPUs
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.223422] NET: Registered protocol family 10
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.231710] NET: Registered protocol family 17
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.235949] Key type dns_resolver registered
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.240145] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.246091] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.252199] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.258350] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.262912] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.269789] registered taskstats version 1
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.272448] Loading compiled-in X.509 certificates
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.276161] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.282658] zswap: loaded using pool lzo/zbud
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.288194] Key type trusted registered
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.296125] Key type encrypted registered
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.300125] ima: No TPM chip found, activating TPM-bypass!
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.303999] evm: HMAC attrs: 0x1
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.307022]   Magic number: 14:391:49
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.310115] memory memory81: hash matches
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.313175] rtc_cmos 00:00: setting system clock to 2018-08-13 23:01:11 UTC (1534201271)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.320194] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.325538] EDD information not available.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.329603] PM: Hibernation image not present or could not be loaded.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.331359] Freeing unused kernel memory: 1496K
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.335567] Write protecting the kernel read-only data: 14336k
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.342562] Freeing unused kernel memory: 1956K
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.349244] Freeing unused kernel memory: 92K
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.371906] systemd-udevd[118]: starting version 204
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.386015] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.454760] scsi host0: Virtio SCSI HBA
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.467002] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.482907] AVX version of gcm_enc/dec engaged.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.488834] AES CTR mode by8 optimization enabled
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.583149] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.586700] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.591620] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.595930] sd 0:0:1:0: [sda] Write Protect is off
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.599482] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.599988] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.610603]  sda: sda1
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.613725] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.813744] tsc: Refined TSC clocksource calibration: 2600.001 MHz
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    4.817082] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3ce1c4c, max_idle_ns: 440795206275 ns
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    5.304071] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    7.462014] floppy0: no floppy controllers found
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    8.629571] raid6: sse2x1   gen()  8850 MB/s
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    8.697562] raid6: sse2x1   xor()  6726 MB/s
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    8.765564] raid6: sse2x2   gen() 11021 MB/s
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    8.833567] raid6: sse2x2   xor()  7369 MB/s
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    8.901568] raid6: sse2x4   gen() 12940 MB/s
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    8.969563] raid6: sse2x4   xor()  9068 MB/s
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    8.970341] raid6: using algorithm sse2x4 gen() 12940 MB/s
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    8.971256] raid6: .... xor() 9068 MB/s, rmw enabled
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    8.971945] raid6: using ssse3x2 recovery algorithm
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    8.973945] xor: automatically using best checksumming function:
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    9.013566]    avx       : 27636.000 MB/sec
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    9.027415] Btrfs loaded
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    9.095160] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    9.096314] EXT4-fs (sda1): write access will be enabled during recovery
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    9.178057] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    9.186546] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    9.187572] EXT4-fs (sda1): recovery complete
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    9.192715] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    9.453713] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    9.598221] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    9.651208] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [    9.901438] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   10.589784] random: cloud-init: uninitialized urandom read (32 bytes read, 47 bits of entropy available)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   10.730974] systemd-udevd[701]: starting version 204
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   10.867391] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   10.926478] intel_rapl: no valid rapl domains found in package 0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   10.982697] ppdev: user-space parallel port driver
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   11.106112] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   11.162566] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   11.226380] random: cloud-init: uninitialized urandom read (32 bytes read, 60 bits of entropy available)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   11.391427] random: cloud-init: uninitialized urandom read (32 bytes read, 60 bits of entropy available)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   11.673579] random: mktemp: uninitialized urandom read (12 bytes read, 63 bits of entropy available)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   11.753932] random: mktemp: uninitialized urandom read (6 bytes read, 63 bits of entropy available)
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   11.834069] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   11.892806] EXT4-fs (sda1): resized filesystem to 7864064
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   12.234791] init: failsafe main process (1093) killed by TERM signal
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO Running set_multiqueue.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO Set channels for eth0 to 4.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 13 23:01:19 travis-job-307aefab-c807-437b-b743-94575f09442b instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 13 23:01:20 travis-job-307aefab-c807-437b-b743-94575f09442b google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug 13 23:01:20 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Starting Google Accounts daemon.
Aug 13 23:01:20 travis-job-307aefab-c807-437b-b743-94575f09442b google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 23:01:20 travis-job-307aefab-c807-437b-b743-94575f09442b google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 13 23:01:20 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Creating a new user account for me.
Aug 13 23:01:20 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Created user account me.
Aug 13 23:01:20 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Creating a new user account for henrik.
Aug 13 23:01:20 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Created user account henrik.
Aug 13 23:01:20 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Creating a new user account for emma.
Aug 13 23:01:20 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Created user account emma.
Aug 13 23:01:20 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Creating a new user account for igor.
Aug 13 23:01:20 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Created user account igor.
Aug 13 23:01:20 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-clock-skew: INFO Synced system time with hardware clock.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Created user account konstantinhaase.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Creating a new user account for aj.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Created user account aj.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Creating a new user account for solarce.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Created user account solarce.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Creating a new user account for asari.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Created user account asari.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Creating a new user account for bogdana.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   13.803830] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   13.808323] Bridge firewalling registered
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Created user account bogdana.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   13.845243] random: nonblocking pool is initialized
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Creating a new user account for konstantin.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Created user account konstantin.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   13.935902] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   13.950334] floppy0: no floppy controllers found
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   13.950515] work still pending
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Creating a new user account for carmen.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   13.990310] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Created user account carmen.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Creating a new user account for maria.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   14.082962] Initializing XFRM netlink socket
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   14.092163] Netfilter messages via NETLINK v0.30.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   14.096040] ctnetlink v0.93: registering with nfnetlink.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Created user account maria.
Aug 13 23:01:21 travis-job-307aefab-c807-437b-b743-94575f09442b google-accounts: INFO Removing user packer.
Aug 13 23:01:25 travis-job-307aefab-c807-437b-b743-94575f09442b pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 23:01:25 travis-job-307aefab-c807-437b-b743-94575f09442b pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 23:01:25 travis-job-307aefab-c807-437b-b743-94575f09442b cron[1710]: (CRON) INFO (pidfile fd = 3)
Aug 13 23:01:25 travis-job-307aefab-c807-437b-b743-94575f09442b cron[1747]: (CRON) STARTUP (fork ok)
Aug 13 23:01:25 travis-job-307aefab-c807-437b-b743-94575f09442b cron[1747]: (CRON) INFO (Running @reboot jobs)
Aug 13 23:01:25 travis-job-307aefab-c807-437b-b743-94575f09442b acpid: starting up with netlink and the input layer
Aug 13 23:01:25 travis-job-307aefab-c807-437b-b743-94575f09442b acpid: 1 rule loaded
Aug 13 23:01:25 travis-job-307aefab-c807-437b-b743-94575f09442b acpid: waiting for events: event logging is off
Aug 13 23:01:25 travis-job-307aefab-c807-437b-b743-94575f09442b haveged: haveged starting up
Aug 13 23:01:25 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   18.190243] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 23:01:30 travis-job-307aefab-c807-437b-b743-94575f09442b ntpd[1846]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 13 23:01:30 travis-job-307aefab-c807-437b-b743-94575f09442b ntpd[1847]: proto: precision = 0.107 usec
Aug 13 23:01:30 travis-job-307aefab-c807-437b-b743-94575f09442b ntpd[1847]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 13 23:01:30 travis-job-307aefab-c807-437b-b743-94575f09442b ntpd[1847]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 23:01:30 travis-job-307aefab-c807-437b-b743-94575f09442b ntpd[1847]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 23:01:30 travis-job-307aefab-c807-437b-b743-94575f09442b ntpd[1847]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 13 23:01:30 travis-job-307aefab-c807-437b-b743-94575f09442b ntpd[1847]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 13 23:01:30 travis-job-307aefab-c807-437b-b743-94575f09442b ntpd[1847]: Listen normally on 3 eth0 10.20.1.155 UDP 123
Aug 13 23:01:30 travis-job-307aefab-c807-437b-b743-94575f09442b ntpd[1847]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 13 23:01:30 travis-job-307aefab-c807-437b-b743-94575f09442b ntpd[1847]: peers refreshed
Aug 13 23:01:30 travis-job-307aefab-c807-437b-b743-94575f09442b ntpd[1847]: Listening on routing socket on fd #21 for interface updates
Aug 13 23:01:30 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   23.413720] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 23:01:31 travis-job-307aefab-c807-437b-b743-94575f09442b startup-script: INFO Found startup-script in metadata.
Aug 13 23:01:31 travis-job-307aefab-c807-437b-b743-94575f09442b startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 13 23:01:31 travis-job-307aefab-c807-437b-b743-94575f09442b startup-script: INFO startup-script: job 1 at Tue Aug 14 02:11:00 2018
Aug 13 23:01:31 travis-job-307aefab-c807-437b-b743-94575f09442b startup-script: INFO startup-script: Return code 0.
Aug 13 23:01:31 travis-job-307aefab-c807-437b-b743-94575f09442b startup-script: INFO startup-script: Return code 0.
Aug 13 23:01:31 travis-job-307aefab-c807-437b-b743-94575f09442b startup-script: INFO Finished running startup scripts.
Aug 13 23:01:31 travis-job-307aefab-c807-437b-b743-94575f09442b ec2: 
Aug 13 23:01:31 travis-job-307aefab-c807-437b-b743-94575f09442b ec2: #############################################################
Aug 13 23:01:31 travis-job-307aefab-c807-437b-b743-94575f09442b ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 13 23:01:31 travis-job-307aefab-c807-437b-b743-94575f09442b ec2: 1024 87:eb:ee:c1:fa:6a:4b:db:77:90:57:dc:6e:ec:a7:e3  root@travis-job-307aefab-c807-437b-b743-94575f09442b (DSA)
Aug 13 23:01:31 travis-job-307aefab-c807-437b-b743-94575f09442b ec2: 256 69:c4:8d:d7:13:cb:c3:30:58:32:88:6a:be:21:78:bc  root@travis-job-307aefab-c807-437b-b743-94575f09442b (ECDSA)
Aug 13 23:01:31 travis-job-307aefab-c807-437b-b743-94575f09442b ec2: 256 d4:70:32:e5:f7:67:d6:8c:2c:2e:ea:af:d7:2d:9e:1e  root@travis-job-307aefab-c807-437b-b743-94575f09442b (ED25519)
Aug 13 23:01:31 travis-job-307aefab-c807-437b-b743-94575f09442b ec2: 2048 25:71:84:d7:dc:f7:0e:c9:17:34:8f:c9:63:28:5c:8d  root@travis-job-307aefab-c807-437b-b743-94575f09442b (RSA)
Aug 13 23:01:31 travis-job-307aefab-c807-437b-b743-94575f09442b ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 13 23:01:31 travis-job-307aefab-c807-437b-b743-94575f09442b ec2: #############################################################
Aug 13 23:01:36 travis-job-307aefab-c807-437b-b743-94575f09442b ntpdate[2052]: the NTP socket is in use, exiting
Aug 13 23:02:14 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [   66.492579] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 13 23:03:16 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [  129.195465] device veth786f310 entered promiscuous mode
Aug 13 23:03:16 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [  129.195536] docker0: port 1(veth786f310) entered forwarding state
Aug 13 23:03:16 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [  129.195551] docker0: port 1(veth786f310) entered forwarding state
Aug 13 23:03:16 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [  129.196050] docker0: port 1(veth786f310) entered disabled state
Aug 13 23:03:16 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [  129.321487] cgroup: docker-runc (4823) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 13 23:03:16 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [  129.321492] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 13 23:03:17 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [  129.404640] eth0: renamed from veth157da95
Aug 13 23:03:17 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [  129.442940] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 13 23:03:17 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [  129.444292] docker0: port 1(veth786f310) entered forwarding state
Aug 13 23:03:17 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [  129.444313] docker0: port 1(veth786f310) entered forwarding state
Aug 13 23:03:17 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [  129.444337] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 13 23:03:20 travis-job-307aefab-c807-437b-b743-94575f09442b ntpd[1847]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 13 23:03:20 travis-job-307aefab-c807-437b-b743-94575f09442b ntpd[1847]: Listen normally on 6 docker0 fe80::42:63ff:fe30:126c UDP 123
Aug 13 23:03:20 travis-job-307aefab-c807-437b-b743-94575f09442b ntpd[1847]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 13 23:03:20 travis-job-307aefab-c807-437b-b743-94575f09442b ntpd[1847]: peers refreshed
Aug 13 23:03:20 travis-job-307aefab-c807-437b-b743-94575f09442b ntpd[1847]: new interface(s) found: waking up resolver
Aug 13 23:03:32 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [  144.468263] docker0: port 1(veth786f310) entered forwarding state
Aug 13 23:17:01 travis-job-307aefab-c807-437b-b743-94575f09442b CRON[12540]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 14 00:10:43 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [ 4176.010237] traps: a[9243] trap invalid opcode ip:56147b310b0b sp:7ffc9e818120 error:0 in a[56147b30d000+6000]
Aug 14 00:11:05 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [ 4198.247860] traps: a[12078] trap invalid opcode ip:7f7a17ae0ad1 sp:7ffd2a294a40 error:0 in libstd-41f43a30bc296e4f.so[7f7a17a82000+167000]
Aug 14 00:11:05 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [ 4198.287228] traps: a[12082] trap invalid opcode ip:7f46c69b0ad1 sp:7ffc3ff3cc70 error:0 in libstd-41f43a30bc296e4f.so[7f46c6952000+167000]
Aug 14 00:13:11 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [ 4323.842076] traps: a[26918] trap invalid opcode ip:55f424009e29 sp:7ffdc3e73df0 error:0 in a[55f424007000+4000]
Aug 14 00:17:01 travis-job-307aefab-c807-437b-b743-94575f09442b CRON[21623]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 14 00:17:12 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [ 4564.635142] a[22921]: segfault at 0 ip 000055a84acca3ef sp 00007ffdf205a180 error 6 in a[55a84acc7000+5000]
Aug 14 00:17:26 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [ 4578.627181] a[23674]: segfault at 1 ip 00005574c9ed9bed sp 00007fffda4d0760 error 6 in a[5574c9ed7000+4000]
Aug 14 00:17:32 travis-job-307aefab-c807-437b-b743-94575f09442b kernel: [ 4585.210182] traps: a[24103] trap invalid opcode ip:55df86ac44bc sp:7fffabfcaed0 error:0 in a[55df86ac1000+7000]
---
travis_time:end:019c065c:start=1534208540331784404,finish=1534208540343079415,duration=11295011
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00ed729b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05e911e0
travis_time:start:05e911e0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0184303e
$ dmesg | grep -i kill
