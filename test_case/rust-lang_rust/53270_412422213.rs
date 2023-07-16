plain
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0175f66b
$ sudo tail -n 500 /var/log/syslog
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] Early memory node ranges
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] Policy zone: Normal
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] console [ttyS0] enabled
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.579587] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.585800] pid_max: default: 32768 minimum: 301
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.589388] ACPI: Core revision 20150930
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.597518] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.601198] Security Framework initialized
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.603922] Yama: becoming mindful.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.606080] AppArmor: AppArmor disabled by boot time parameter
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.610517] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.623646] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.630889] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.634507] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.638332] Initializing cgroup subsys io
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.640649] Initializing cgroup subsys memory
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.643312] Initializing cgroup subsys devices
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.646387] Initializing cgroup subsys freezer
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.648594] Initializing cgroup subsys net_cls
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.651220] Initializing cgroup subsys perf_event
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.654314] Initializing cgroup subsys net_prio
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.656523] Initializing cgroup subsys hugetlb
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.658775] Initializing cgroup subsys pids
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.660878] CPU: Physical Processor ID: 0
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.662995] CPU: Processor Core ID: 0
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.665036] mce: CPU supports 32 MCE banks
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.667215] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.670706] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.676133] Freeing SMP alternatives memory: 32K
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.687780] ftrace: allocating 32185 entries in 126 pages
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.743558] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.747460] smpboot: Max logical packages: 2
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.750633] x2apic enabled
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.753188] Switched APIC routing to physical x2apic.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.759122] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.868372] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.873368] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.879208] x86: Booting SMP configuration:
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.881629] .... node  #0, CPUs:      #1
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.883778] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.889631]  #2
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.890807] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.896518]  #3
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.897482] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.903189] x86: Booted up 1 node, 4 CPUs
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.905410] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.910321] devtmpfs: initialized
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.916044] evm: security.selinux
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.918125] evm: security.SMACK64
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.919639] evm: security.SMACK64EXEC
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.921430] evm: security.SMACK64TRANSMUTE
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.923573] evm: security.SMACK64MMAP
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.925320] evm: security.ima
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.926646] evm: security.capability
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.929052] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.934344] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.937810] pinctrl core: initialized pinctrl subsystem
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.940712] RTC time:  5:22:32, date: 08/13/18
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.944356] NET: Registered protocol family 16
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.956474] cpuidle: using governor ladder
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.968428] cpuidle: using governor menu
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.970502] PCCT header not found.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.972650] ACPI: bus type PCI registered
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.975184] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.979999] PCI: Using configuration type 1 for base access
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    0.998099] ACPI: Added _OSI(Module Device)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.000872] ACPI: Added _OSI(Processor Device)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.003241] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.006079] ACPI: Added _OSI(Processor Aggregator Device)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.011425] ACPI: Executed 2 blocks of module-level executable AML code
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.038033] ACPI: Interpreter enabled
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.039977] ACPI: (supports S0 S3 S4 S5)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.042055] ACPI: Using IOAPIC for interrupt routing
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.044898] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.077861] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.081478] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.085350] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.088772] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.095187] PCI host bridge to bus 0000:00
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.097349] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.101110] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.104775] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.109311] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.113655] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.116869] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.117331] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.140900] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.163949] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.168180] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.177761] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.185535] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.204607] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.213839] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.220798] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.240684] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.245753] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.251669] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.256882] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.261901] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.284685] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.288712] vgaarb: loaded
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.290479] SCSI subsystem initialized
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.292677] libata version 3.00 loaded.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.292703] ACPI: bus type USB registered
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.294707] usbcore: registered new interface driver usbfs
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.298079] usbcore: registered new interface driver hub
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.301055] usbcore: registered new device driver usb
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.304373] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.307911] dmi: Firmware registration failed.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.311201] PCI: Using ACPI for IRQ routing
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.313690] PCI: pci_cache_line_size set to 64 bytes
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.313789] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.313791] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.313927] NetLabel: Initializing
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.316230] NetLabel:  domain hash size = 128
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.318431] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.321122] NetLabel:  unlabeled traffic allowed by default
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.323983] amd_nb: Cannot enumerate AMD northbridges
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.326919] clocksource: Switched to clocksource kvm-clock
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.337099] pnp: PnP ACPI init
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.338800] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.338875] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.338934] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.338988] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.339031] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.339072] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.339113] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.339300] pnp: PnP ACPI: found 7 devices
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.349484] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.354691] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.354693] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.354695] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.354697] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.354734] NET: Registered protocol family 2
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.357086] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.361377] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.364564] TCP: Hash tables configured (established 131072 bind 65536)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.368025] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.371439] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.374716] NET: Registered protocol family 1
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.376746] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.380340] PCI: CLS 0 bytes, default 64
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    1.381108] Unpacking initramfs...
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.432078] Freeing initrd memory: 21432K
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.434225] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.437972] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.442945] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.446853] hw unit of domain pp0-core 2^-0 Joules
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.448938] hw unit of domain package 2^-0 Joules
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.451891] hw unit of domain dram 2^-0 Joules
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.454303] Scanning for low memory corruption every 60 seconds
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.458077] audit: initializing netlink subsys (disabled)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.460915] audit: type=2000 audit(1534137754.898:1): initialized
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.464229] Initialise system trusted keyring
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.467652] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.470833] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.474931] zbud: loaded
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.476819] VFS: Disk quotas dquot_6.6.0
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.478658] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.482591] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.486207] fuse init (API version 7.23)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.488742] Key type big_key registered
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.490605] Allocating IMA MOK and blacklist keyrings.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.497557] Key type asymmetric registered
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.499602] Asymmetric key parser 'x509' registered
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.502414] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.506499] io scheduler noop registered
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.508467] io scheduler deadline registered (default)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.511250] io scheduler cfq registered
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.513622] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.516891] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.520680] intel_idle: does not run on family 6 model 45
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.520782] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.524894] ACPI: Power Button [PWRF]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.526769] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.530510] ACPI: Sleep Button [SLPF]
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.533074] GHES: HEST is not enabled!
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.537954] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.541306] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.550663] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.553509] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.564695] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.589596] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.615548] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.641550] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.667585] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.674443] Linux agpgart interface v0.103
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.682024] loop: module loaded
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.683990] libphy: Fixed MDIO Bus: probed
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.685837] tun: Universal TUN/TAP device driver, 1.6
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.688769] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.734475] PPP generic driver version 2.4.2
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.737387] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.741336] ehci-pci: EHCI PCI platform driver
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.743258] ehci-platform: EHCI generic platform driver
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.746369] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.749419] ohci-pci: OHCI PCI platform driver
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.752014] ohci-platform: OHCI generic platform driver
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.754572] uhci_hcd: USB Universal Host Controller Interface driver
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.758500] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.763541] i8042: Warning: Keylock active
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.767499] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.769943] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.773556] mousedev: PS/2 mouse device common for all mice
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.777776] rtc_cmos 00:00: RTC can wake from S4
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.780742] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.784434] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.787207] i2c /dev entries driver
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.788931] device-mapper: uevent: version 1.0.3
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.791782] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.796328] ledtrig-cpu: registered to indicate activity on CPUs
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.801197] NET: Registered protocol family 10
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.804936] NET: Registered protocol family 17
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.807230] Key type dns_resolver registered
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.810249] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.813201] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.816412] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.820624] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.824196] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.829360] registered taskstats version 1
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.831353] Loading compiled-in X.509 certificates
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.834762] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.840389] zswap: loaded using pool lzo/zbud
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.844997] Key type trusted registered
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.851984] Key type encrypted registered
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.854079] ima: No TPM chip found, activating TPM-bypass!
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.856948] evm: HMAC attrs: 0x1
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.858999]   Magic number: 14:339:365
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.861080] rtc_cmos 00:00: setting system clock to 2018-08-13 05:22:35 UTC (1534137755)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.865821] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.869172] EDD information not available.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.871636] PM: Hibernation image not present or could not be loaded.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.873304] Freeing unused kernel memory: 1496K
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.875817] Write protecting the kernel read-only data: 14336k
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.880230] Freeing unused kernel memory: 1956K
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.883206] Freeing unused kernel memory: 92K
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.902652] systemd-udevd[118]: starting version 204
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.960006] scsi host0: Virtio SCSI HBA
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.967539] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.975424] AVX version of gcm_enc/dec engaged.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.975655] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    3.983488] AES CTR mode by8 optimization enabled
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    4.047312] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    4.051083] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    4.055760] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    4.059553] sd 0:0:1:0: [sda] Write Protect is off
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    4.062002] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    4.062298] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    4.071375]  sda: sda1
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    4.076034] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    4.451154] tsc: Refined TSC clocksource calibration: 2599.995 MHz
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    4.455327] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a376535a, max_idle_ns: 440795296166 ns
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    4.809433] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    6.955126] floppy0: no floppy controllers found
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.122938] raid6: sse2x1   gen()  8656 MB/s
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.190937] raid6: sse2x1   xor()  6481 MB/s
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.258936] raid6: sse2x2   gen() 10574 MB/s
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.326932] raid6: sse2x2   xor()  7183 MB/s
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.394929] raid6: sse2x4   gen() 12549 MB/s
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.462931] raid6: sse2x4   xor()  8789 MB/s
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.465591] raid6: using algorithm sse2x4 gen() 12549 MB/s
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.468426] raid6: .... xor() 8789 MB/s, rmw enabled
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.471215] raid6: using ssse3x2 recovery algorithm
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.476900] xor: automatically using best checksumming function:
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.518930]    avx       : 27035.000 MB/sec
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.536582] Btrfs loaded
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.599645] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.603644] EXT4-fs (sda1): write access will be enabled during recovery
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.683190] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.693580] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.696157] EXT4-fs (sda1): recovery complete
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.706154] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    8.967257] random: init: uninitialized urandom read (12 bytes read, 22 bits of entropy available)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    9.105802] random: mountall: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    9.162586] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [    9.387307] random: cloud-init: uninitialized urandom read (32 bytes read, 32 bits of entropy available)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   10.004384] random: cloud-init: uninitialized urandom read (32 bytes read, 39 bits of entropy available)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   10.151885] systemd-udevd[702]: starting version 204
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   10.277712] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   10.318572] intel_rapl: no valid rapl domains found in package 0
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   10.370229] ppdev: user-space parallel port driver
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   10.455168] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   10.498901] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   10.562304] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   10.730648] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   11.011044] random: mktemp: uninitialized urandom read (12 bytes read, 55 bits of entropy available)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   11.092886] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   11.174738] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   11.227231] EXT4-fs (sda1): resized filesystem to 7864064
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   11.958116] init: failsafe main process (1094) killed by TERM signal
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO Running set_multiqueue.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO Set channels for eth0 to 4.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 13 05:22:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 13 05:22:44 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 05:22:44 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 05:22:44 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Starting Google Accounts daemon.
Aug 13 05:22:44 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Creating a new user account for me.
Aug 13 05:22:44 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 13 05:22:44 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Created user account me.
Aug 13 05:22:44 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Creating a new user account for henrik.
Aug 13 05:22:44 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Created user account henrik.
Aug 13 05:22:44 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Creating a new user account for emma.
Aug 13 05:22:44 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Created user account emma.
Aug 13 05:22:44 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Creating a new user account for igor.
Aug 13 05:22:44 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Created user account igor.
Aug 13 05:22:44 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 13 05:22:44 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Created user account konstantinhaase.
Aug 13 05:22:44 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Creating a new user account for aj.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-clock-skew: INFO Synced system time with hardware clock.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Created user account aj.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Creating a new user account for solarce.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Created user account solarce.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Creating a new user account for asari.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Created user account asari.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Creating a new user account for bogdana.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Created user account bogdana.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Creating a new user account for konstantin.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   13.331120] floppy0: no floppy controllers found
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   13.334888] random: nonblocking pool is initialized
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Created user account konstantin.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Creating a new user account for carmen.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   13.399420] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   13.403597] Bridge firewalling registered
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   13.416842] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Created user account carmen.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   13.460770] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Creating a new user account for maria.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Created user account maria.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   13.525351] Initializing XFRM netlink socket
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 google-accounts: INFO Removing user packer.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   13.534989] Netfilter messages via NETLINK v0.30.
Aug 13 05:22:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   13.538423] ctnetlink v0.93: registering with nfnetlink.
Aug 13 05:22:47 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 cron[1709]: (CRON) INFO (pidfile fd = 3)
Aug 13 05:22:47 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 cron[1739]: (CRON) STARTUP (fork ok)
Aug 13 05:22:47 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 05:22:47 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 cron[1739]: (CRON) INFO (Running @reboot jobs)
Aug 13 05:22:47 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 05:22:47 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 acpid: starting up with netlink and the input layer
Aug 13 05:22:47 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 acpid: 1 rule loaded
Aug 13 05:22:47 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 acpid: waiting for events: event logging is off
Aug 13 05:22:47 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 haveged: haveged starting up
Aug 13 05:22:47 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   15.829931] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 05:22:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1853]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 13 05:22:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1854]: proto: precision = 0.102 usec
Aug 13 05:22:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1854]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 13 05:22:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1854]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 05:22:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1854]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 05:22:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1854]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 13 05:22:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1854]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 13 05:22:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1854]: Listen normally on 3 eth0 10.20.0.165 UDP 123
Aug 13 05:22:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1854]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 13 05:22:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1854]: peers refreshed
Aug 13 05:22:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1854]: Listening on routing socket on fd #21 for interface updates
Aug 13 05:22:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   21.010821] init: plymouth-upstart-bridge main process (510) killed by TERM signal
Aug 13 05:22:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 startup-script: INFO Found startup-script in metadata.
Aug 13 05:22:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 13 05:22:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 startup-script: INFO startup-script: job 1 at Mon Aug 13 08:32:00 2018
Aug 13 05:22:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 startup-script: INFO startup-script: Return code 0.
Aug 13 05:22:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 startup-script: INFO startup-script: Return code 0.
Aug 13 05:22:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 startup-script: INFO Finished running startup scripts.
Aug 13 05:22:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ec2: 
Aug 13 05:22:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ec2: #############################################################
Aug 13 05:22:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 13 05:22:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ec2: 1024 6b:14:23:64:aa:4c:b7:ff:75:fc:0e:b3:8d:8c:bc:5a  root@travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 (DSA)
Aug 13 05:22:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ec2: 256 a4:fd:4e:b1:b9:b6:4d:ee:32:62:35:7b:36:c4:5e:32  root@travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 (ECDSA)
Aug 13 05:22:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ec2: 256 9c:90:a6:6c:e5:4d:06:c4:4f:a6:6f:55:c9:cc:92:ae  root@travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 (ED25519)
Aug 13 05:22:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ec2: 2048 a1:e0:5a:c6:55:28:67:b5:54:30:e5:15:81:0d:1c:a7  root@travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 (RSA)
Aug 13 05:22:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 13 05:22:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ec2: #############################################################
Aug 13 05:23:00 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpdate[2257]: the NTP socket is in use, exiting
Aug 13 05:23:38 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [   66.194814] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 13 05:27:28 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [  296.956774] device vethc74b6a5 entered promiscuous mode
Aug 13 05:27:28 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [  296.956852] docker0: port 1(vethc74b6a5) entered forwarding state
Aug 13 05:27:28 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [  296.956861] docker0: port 1(vethc74b6a5) entered forwarding state
Aug 13 05:27:28 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [  296.957279] docker0: port 1(vethc74b6a5) entered disabled state
Aug 13 05:27:29 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [  297.054766] cgroup: docker-runc (5009) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 13 05:27:29 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [  297.054769] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 13 05:27:29 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [  297.150231] eth0: renamed from vethb89179d
Aug 13 05:27:29 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [  297.192234] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 13 05:27:29 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [  297.193551] docker0: port 1(vethc74b6a5) entered forwarding state
Aug 13 05:27:29 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [  297.193572] docker0: port 1(vethc74b6a5) entered forwarding state
Aug 13 05:27:29 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [  297.193595] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 13 05:27:32 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1854]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 13 05:27:32 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1854]: Listen normally on 6 docker0 fe80::42:deff:fe70:363 UDP 123
Aug 13 05:27:32 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1854]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 13 05:27:32 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1854]: peers refreshed
Aug 13 05:27:32 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1854]: new interface(s) found: waking up resolver
Aug 13 05:27:44 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [  312.221618] docker0: port 1(vethc74b6a5) entered forwarding state
Aug 13 06:17:01 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 CRON[17364]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 13 06:25:01 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 CRON[20634]: (root) CMD (test -x /usr/sbin/anacron || ( cd / && run-parts --report /etc/cron.daily ))
Aug 13 06:34:17 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 upstart-socket-bridge[839]: Disconnected from Upstart
Aug 13 06:34:17 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 upstart-udev-bridge[697]: Disconnected from Upstart
Aug 13 06:34:17 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [ 4305.383998] init: upstart-udev-bridge main process (697) terminated with status 1
Aug 13 06:34:17 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [ 4305.384011] init: upstart-udev-bridge main process ended, respawning
Aug 13 06:34:17 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [ 4305.384119] init: upstart-socket-bridge main process (839) terminated with status 1
Aug 13 06:34:17 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [ 4305.384127] init: upstart-socket-bridge main process ended, respawning
Aug 13 06:34:17 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [ 4305.384245] init: upstart-file-bridge main process (1203) terminated with status 1
Aug 13 06:34:17 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [ 4305.384258] init: upstart-file-bridge main process ended, respawning
Aug 13 06:34:31 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 dbus[1154]: [system] Reloaded configuration
Aug 13 06:34:31 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 dbus[1154]: message repeated 9 times: [ [system] Reloaded configuration]
Aug 13 06:34:36 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[1854]: ntpd exiting on signal 15
Aug 13 06:34:43 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [ 4331.614064] init: apport post-stop process (3614) terminated with status 1
Aug 13 06:34:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 dbus[1154]: [system] Reloaded configuration
Aug 13 06:34:45 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 dbus[1154]: message repeated 4 times: [ [system] Reloaded configuration]
Aug 13 06:34:51 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 kernel: [ 4339.477134] systemd-udevd[5657]: starting version 204
Aug 13 06:34:51 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 dbus[1154]: [system] Reloaded configuration
Aug 13 06:34:51 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 dbus[1154]: message repeated 3 times: [ [system] Reloaded configuration]
Aug 13 06:34:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[5840]: ntpd 4.2.6p5@1.2349-o Fri Jul  6 20:19:54 UTC 2018 (1)
Aug 13 06:34:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[5841]: ntp_io: estimated max descriptors: 72000, initial socket boundary: 16
Aug 13 06:34:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[5841]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 06:34:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[5841]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 13 06:34:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[5841]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 13 06:34:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[5841]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 13 06:34:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[5841]: Listen normally on 3 eth0 10.20.0.165 UDP 123
Aug 13 06:34:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[5841]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 13 06:34:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[5841]: peers refreshed
Aug 13 06:34:52 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 ntpd[5841]: Listening on routing socket on fd #21 for interface updates
Aug 13 06:34:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 06:34:53 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 06:34:55 travis-job-b8dfded4-93b6-4d05-9e3e-6ed6fa6e81b5 dbus[1154]: [system] Reloaded configuration
---
travis_time:end:0d3771e6:start=1534142516445446459,finish=1534142516454843904,duration=9397445
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04061838
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0007d5ce
travis_time:start:0007d5ce
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:1ea7cf10
$ dmesg | grep -i kill
$ dmesg | grep -i kill
[   11.958116] init: failsafe main process (1094) killed by TERM signal
[   21.010821] init: plymouth-upstart-bridge main process (510) killed by TERM signal
travis_fold:end:after_failure.7

Done. Your build exited with 1.
