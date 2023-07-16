plain
[00:35:13]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:35:52] [RUSTC-TIMING] rustc_traits test:false 38.716
[00:37:07] [RUSTC-TIMING] rustc_typeck test:false 166.982

Broadcast message from root@travis-job-2ec8d179-f671-477e-b985-5d538b56687a
 (unknown) at 20:05 ...
The system is going down for power off NOW!
[00:37:29] 
[00:37:29] Session terminated, terminating shell... ...terminated.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:01b56acc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:1b1adea0
$ sudo tail -n 500 /var/log/syslog
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] kvm-clock: using sched offset of 2026282153 cycles
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] Zone ranges:
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000]   Device   empty
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] Movable zone start for each node
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] Early memory node ranges
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] Policy zone: Normal
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] Hierarchical RCU implementation.
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] console [ttyS0] enabled
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.602528] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.605711] pid_max: default: 32768 minimum: 301
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.608249] ACPI: Core revision 20150930
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.617378] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.621270] Security Framework initialized
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.622989] Yama: becoming mindful.
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.624493] AppArmor: AppArmor disabled by boot time parameter
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.629299] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.645450] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.655339] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.659786] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.664449] Initializing cgroup subsys io
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.666962] Initializing cgroup subsys memory
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.670061] Initializing cgroup subsys devices
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.672582] Initializing cgroup subsys freezer
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.674884] Initializing cgroup subsys net_cls
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.676736] Initializing cgroup subsys perf_event
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.678988] Initializing cgroup subsys net_prio
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.681234] Initializing cgroup subsys hugetlb
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.684224] Initializing cgroup subsys pids
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.686182] CPU: Physical Processor ID: 0
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.687958] CPU: Processor Core ID: 0
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.689855] mce: CPU supports 32 MCE banks
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.692086] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.694255] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.699889] Freeing SMP alternatives memory: 32K
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.712639] ftrace: allocating 32185 entries in 126 pages
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.773597] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.776874] smpboot: Max logical packages: 2
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.780144] x2apic enabled
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.783261] Switched APIC routing to physical x2apic.
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.788928] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.898447] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.905863] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.913146] x86: Booting SMP configuration:
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.915744] .... node  #0, CPUs:      #1
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.918532] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.923953]  #2
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.925009] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.931667]  #3
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.932627] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.937533] x86: Booted up 1 node, 4 CPUs
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.939711] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.945366] devtmpfs: initialized
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.952056] evm: security.selinux
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.953814] evm: security.SMACK64
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.956196] evm: security.SMACK64EXEC
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.957740] evm: security.SMACK64TRANSMUTE
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.959415] evm: security.SMACK64MMAP
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.961506] evm: security.ima
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.963129] evm: security.capability
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.965834] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.971125] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.975602] pinctrl core: initialized pinctrl subsystem
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.979730] RTC time: 19:26:28, date: 08/13/18
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.982311] NET: Registered protocol family 16
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    0.994544] cpuidle: using governor ladder
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.006558] cpuidle: using governor menu
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.009039] PCCT header not found.
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.010830] ACPI: bus type PCI registered
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.012549] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.015563] PCI: Using configuration type 1 for base access
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.032205] ACPI: Added _OSI(Module Device)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.034320] ACPI: Added _OSI(Processor Device)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.036414] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.038644] ACPI: Added _OSI(Processor Aggregator Device)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.043915] ACPI: Executed 2 blocks of module-level executable AML code
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.073825] ACPI: Interpreter enabled
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.077147] ACPI: (supports S0 S3 S4 S5)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.081172] ACPI: Using IOAPIC for interrupt routing
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.089132] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.126422] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.130888] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.133490] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.137240] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.145824] PCI host bridge to bus 0000:00
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.148927] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.152959] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.157419] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.163775] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.167378] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.169588] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.170065] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.201838] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.235117] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.239010] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.252581] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.262575] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.292635] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.309594] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.326537] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.366972] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.374191] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.381771] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.388520] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.395754] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.419600] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.422443] vgaarb: loaded
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.424089] SCSI subsystem initialized
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.426148] libata version 3.00 loaded.
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.426179] ACPI: bus type USB registered
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.427775] usbcore: registered new interface driver usbfs
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.430821] usbcore: registered new interface driver hub
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.433386] usbcore: registered new device driver usb
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.435701] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.440789] dmi: Firmware registration failed.
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.443021] PCI: Using ACPI for IRQ routing
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.445640] PCI: pci_cache_line_size set to 64 bytes
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.445761] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.445764] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.445920] NetLabel: Initializing
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.448821] NetLabel:  domain hash size = 128
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.451539] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.454306] NetLabel:  unlabeled traffic allowed by default
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.456724] amd_nb: Cannot enumerate AMD northbridges
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.459285] clocksource: Switched to clocksource kvm-clock
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.469883] pnp: PnP ACPI init
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.471937] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.472016] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.472090] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.472146] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.472237] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.472280] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.472323] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.472504] pnp: PnP ACPI: found 7 devices
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.482731] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.486535] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.486538] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.486540] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.486542] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.486590] NET: Registered protocol family 2
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.489961] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.494443] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.497859] TCP: Hash tables configured (established 131072 bind 65536)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.501324] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.505387] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.509026] NET: Registered protocol family 1
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.511771] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.515116] PCI: CLS 0 bytes, default 64
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    1.515205] Unpacking initramfs...
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.706028] Freeing initrd memory: 21432K
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.707034] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.708184] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.709891] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.711592] hw unit of domain pp0-core 2^-0 Joules
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.712365] hw unit of domain package 2^-0 Joules
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.713381] hw unit of domain dram 2^-0 Joules
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.714368] Scanning for low memory corruption every 60 seconds
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.715864] audit: initializing netlink subsys (disabled)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.717015] audit: type=2000 audit(1534188390.873:1): initialized
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.718388] Initialise system trusted keyring
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.719505] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.720771] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.723209] zbud: loaded
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.724071] VFS: Disk quotas dquot_6.6.0
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.724802] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.726473] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.728101] fuse init (API version 7.23)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.728961] Key type big_key registered
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.729806] Allocating IMA MOK and blacklist keyrings.
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.732375] Key type asymmetric registered
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.733330] Asymmetric key parser 'x509' registered
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.734208] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.735390] io scheduler noop registered
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.736208] io scheduler deadline registered (default)
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.737266] io scheduler cfq registered
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.738145] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.739099] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.740296] intel_idle: does not run on family 6 model 45
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.740404] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.741755] ACPI: Power Button [PWRF]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.742441] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.743734] ACPI: Sleep Button [SLPF]
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.744796] GHES: HEST is not enabled!
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.747947] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.749258] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.755416] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.756571] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.763223] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.785794] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.809431] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.833177] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.857531] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.861581] Linux agpgart interface v0.103
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.866050] loop: module loaded
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.867326] libphy: Fixed MDIO Bus: probed
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.868505] tun: Universal TUN/TAP device driver, 1.6
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.870249] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.924732] PPP generic driver version 2.4.2
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.926218] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.927765] ehci-pci: EHCI PCI platform driver
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.929182] ehci-platform: EHCI generic platform driver
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.930640] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.932562] ohci-pci: OHCI PCI platform driver
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.934148] ohci-platform: OHCI generic platform driver
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.935642] uhci_hcd: USB Universal Host Controller Interface driver
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.937515] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.940468] i8042: Warning: Keylock active
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.942749] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.944679] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.946540] mousedev: PS/2 mouse device common for all mice
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.948811] rtc_cmos 00:00: RTC can wake from S4
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.950633] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.952796] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.955661] i2c /dev entries driver
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.956630] device-mapper: uevent: version 1.0.3
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.958575] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.961507] ledtrig-cpu: registered to indicate activity on CPUs
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.964372] NET: Registered protocol family 10
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.965960] NET: Registered protocol family 17
Aug 13 19:26:39 travis-job-2ec8d179-f671-477e-b985-5d538b56687a kernel: [    3.967404] Key type dns_resolver registered
