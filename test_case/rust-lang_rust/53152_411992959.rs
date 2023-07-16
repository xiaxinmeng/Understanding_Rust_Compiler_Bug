plain
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:14655276
$ sudo tail -n 500 /var/log/syslog
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] Policy zone: Normal
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] console [ttyS0] enabled
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.337594] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.338972] pid_max: default: 32768 minimum: 301
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.339764] ACPI: Core revision 20150930
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.346499] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.347639] Security Framework initialized
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.348454] Yama: becoming mindful.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.349183] AppArmor: AppArmor disabled by boot time parameter
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.352176] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.361959] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.366808] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.367831] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.369440] Initializing cgroup subsys io
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.370076] Initializing cgroup subsys memory
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.370730] Initializing cgroup subsys devices
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.371394] Initializing cgroup subsys freezer
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.372059] Initializing cgroup subsys net_cls
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.372920] Initializing cgroup subsys perf_event
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.373756] Initializing cgroup subsys net_prio
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.374529] Initializing cgroup subsys hugetlb
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.375247] Initializing cgroup subsys pids
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.376155] CPU: Physical Processor ID: 0
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.376944] CPU: Processor Core ID: 0
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.377662] mce: CPU supports 32 MCE banks
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.378776] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.379583] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.382598] Freeing SMP alternatives memory: 32K
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.391826] ftrace: allocating 32185 entries in 126 pages
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.441180] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.442455] smpboot: Max logical packages: 2
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.443719] x2apic enabled
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.445292] Switched APIC routing to physical x2apic.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.448837] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.556785] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.558288] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.561659] x86: Booting SMP configuration:
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.562433] .... node  #0, CPUs:      #1
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.563462] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.566818]  #2
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.567327] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.570931]  #3
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.571391] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.574904] x86: Booted up 1 node, 4 CPUs
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.575547] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.578233] devtmpfs: initialized
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.583086] evm: security.selinux
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.583858] evm: security.SMACK64
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.584509] evm: security.SMACK64EXEC
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.585244] evm: security.SMACK64TRANSMUTE
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.585842] evm: security.SMACK64MMAP
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.586397] evm: security.ima
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.586866] evm: security.capability
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.587972] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.590121] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.592302] pinctrl core: initialized pinctrl subsystem
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.593738] RTC time:  5:40:33, date: 08/10/18
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.595668] NET: Registered protocol family 16
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.606895] cpuidle: using governor ladder
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.616863] cpuidle: using governor menu
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.617795] PCCT header not found.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.618773] ACPI: bus type PCI registered
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.619465] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.621331] PCI: Using configuration type 1 for base access
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.633931] ACPI: Added _OSI(Module Device)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.634786] ACPI: Added _OSI(Processor Device)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.635915] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.636757] ACPI: Added _OSI(Processor Aggregator Device)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.640393] ACPI: Executed 2 blocks of module-level executable AML code
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.665894] ACPI: Interpreter enabled
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.666857] ACPI: (supports S0 S3 S4 S5)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.667499] ACPI: Using IOAPIC for interrupt routing
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.668477] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.699811] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.701043] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.702052] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.703239] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.706191] PCI host bridge to bus 0000:00
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.706865] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.708214] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.709156] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.710312] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.711625] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.712752] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.713246] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.729229] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.744165] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.746177] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.753170] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.760729] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.775906] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.782425] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.787872] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.803317] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.805834] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.808753] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.811978] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.814300] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.835732] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.837049] vgaarb: loaded
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.837840] SCSI subsystem initialized
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.838637] libata version 3.00 loaded.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.838662] ACPI: bus type USB registered
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.839397] usbcore: registered new interface driver usbfs
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.840606] usbcore: registered new interface driver hub
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.841733] usbcore: registered new device driver usb
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.842955] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.844035] dmi: Firmware registration failed.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.844927] PCI: Using ACPI for IRQ routing
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.845975] PCI: pci_cache_line_size set to 64 bytes
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.846079] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.846081] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.846211] NetLabel: Initializing
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.846903] NetLabel:  domain hash size = 128
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.847577] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.848434] NetLabel:  unlabeled traffic allowed by default
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.849698] amd_nb: Cannot enumerate AMD northbridges
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.850710] clocksource: Switched to clocksource kvm-clock
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.858810] pnp: PnP ACPI init
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.859757] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.859827] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.859871] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.859924] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.859965] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.860041] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.860083] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.860251] pnp: PnP ACPI: found 7 devices
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.868259] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.869781] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.869784] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.869786] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.869788] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.869824] NET: Registered protocol family 2
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.870958] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.872545] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.873874] TCP: Hash tables configured (established 131072 bind 65536)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.875046] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.876595] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.877629] NET: Registered protocol family 1
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.878799] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.879685] PCI: CLS 0 bytes, default 64
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    0.880536] Unpacking initramfs...
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.021220] Freeing initrd memory: 21432K
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.022695] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.024189] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.027024] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.028935] hw unit of domain pp0-core 2^-0 Joules
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.029606] hw unit of domain package 2^-0 Joules
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.030447] hw unit of domain dram 2^-0 Joules
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.031545] Scanning for low memory corruption every 60 seconds
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.033810] audit: initializing netlink subsys (disabled)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.034625] audit: type=2000 audit(1533879635.991:1): initialized
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.036446] Initialise system trusted keyring
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.037558] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.038595] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.040914] zbud: loaded
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.041892] VFS: Disk quotas dquot_6.6.0
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.042933] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.044783] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.046733] fuse init (API version 7.23)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.047523] Key type big_key registered
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.048322] Allocating IMA MOK and blacklist keyrings.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.050738] Key type asymmetric registered
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.051779] Asymmetric key parser 'x509' registered
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.052563] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.054148] io scheduler noop registered
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.055129] io scheduler deadline registered (default)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.056682] io scheduler cfq registered
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.057798] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.059716] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.061709] intel_idle: does not run on family 6 model 45
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.061857] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.063159] ACPI: Power Button [PWRF]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.064811] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.066409] ACPI: Sleep Button [SLPF]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.067549] GHES: HEST is not enabled!
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.070057] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.071612] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.077447] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.082241] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.094622] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.117576] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.142447] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.167873] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.191325] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.194963] Linux agpgart interface v0.103
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.198540] loop: module loaded
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.199634] libphy: Fixed MDIO Bus: probed
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.200754] tun: Universal TUN/TAP device driver, 1.6
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.202272] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.294507] PPP generic driver version 2.4.2
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.295778] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.297324] ehci-pci: EHCI PCI platform driver
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.298313] ehci-platform: EHCI generic platform driver
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.299599] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.301022] ohci-pci: OHCI PCI platform driver
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.301884] ohci-platform: OHCI generic platform driver
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.303272] uhci_hcd: USB Universal Host Controller Interface driver
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.305014] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.308038] i8042: Warning: Keylock active
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.309855] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.311162] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.312431] mousedev: PS/2 mouse device common for all mice
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.313946] rtc_cmos 00:00: RTC can wake from S4
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.315283] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.316629] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.317685] i2c /dev entries driver
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.318422] device-mapper: uevent: version 1.0.3
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.319700] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.321536] ledtrig-cpu: registered to indicate activity on CPUs
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.323520] NET: Registered protocol family 10
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.324851] NET: Registered protocol family 17
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.325882] Key type dns_resolver registered
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.327157] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.328767] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.329958] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.331243] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.332761] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.335088] registered taskstats version 1
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.336181] Loading compiled-in X.509 certificates
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.337877] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.340701] zswap: loaded using pool lzo/zbud
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.343705] Key type trusted registered
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.348219] Key type encrypted registered
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.349151] ima: No TPM chip found, activating TPM-bypass!
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.350515] evm: HMAC attrs: 0x1
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.351648]   Magic number: 14:423:668
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.352662] acpi LNXCPU:17: hash matches
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.353787] rtc_cmos 00:00: setting system clock to 2018-08-10 05:40:36 UTC (1533879636)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.355930] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.357237] EDD information not available.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.358164] PM: Hibernation image not present or could not be loaded.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.359597] Freeing unused kernel memory: 1496K
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.360388] Write protecting the kernel read-only data: 14336k
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.362192] Freeing unused kernel memory: 1956K
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.363314] Freeing unused kernel memory: 92K
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.377884] systemd-udevd[119]: starting version 204
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.465113] scsi host0: Virtio SCSI HBA
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.472838] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.477718] AVX version of gcm_enc/dec engaged.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.478640] AES CTR mode by8 optimization enabled
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.511280] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.517139] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.517142] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.519689] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.520928] sd 0:0:1:0: [sda] Write Protect is off
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.521775] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.521976] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.526364]  sda: sda1
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    3.527689] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    4.030900] tsc: Refined TSC clocksource calibration: 2599.786 MHz
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    4.031930] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257971e5419, max_idle_ns: 440795308604 ns
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    4.348623] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    6.426979] floppy0: no floppy controllers found
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    7.586747] raid6: sse2x1   gen()  9217 MB/s
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    7.654825] raid6: sse2x1   xor()  7052 MB/s
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    7.722762] raid6: sse2x2   gen() 11299 MB/s
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    7.790748] raid6: sse2x2   xor()  7715 MB/s
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    7.858763] raid6: sse2x4   gen() 12377 MB/s
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    7.926749] raid6: sse2x4   xor()  8742 MB/s
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    7.927879] raid6: using algorithm sse2x4 gen() 12377 MB/s
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    7.928955] raid6: .... xor() 8742 MB/s, rmw enabled
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    7.929834] raid6: using ssse3x2 recovery algorithm
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    7.932877] xor: automatically using best checksumming function:
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    7.970783]    avx       : 22483.000 MB/sec
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    7.986292] Btrfs loaded
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    8.073919] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    8.075627] EXT4-fs (sda1): write access will be enabled during recovery
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    8.154046] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    8.162661] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    8.164068] EXT4-fs (sda1): recovery complete
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    8.169898] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    8.440270] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    8.595236] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    8.646526] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    8.895207] random: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    9.548119] random: cloud-init: uninitialized urandom read (32 bytes read, 46 bits of entropy available)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    9.685665] systemd-udevd[702]: starting version 204
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    9.825543] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    9.872948] intel_rapl: no valid rapl domains found in package 0
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    9.915485] intel_rapl: no valid rapl domains found in package 0
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [    9.931210] ppdev: user-space parallel port driver
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   10.057136] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   10.114861] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   10.180125] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   10.342813] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   10.614213] random: mktemp: uninitialized urandom read (12 bytes read, 62 bits of entropy available)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   10.701914] random: mktemp: uninitialized urandom read (6 bytes read, 63 bits of entropy available)
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   10.791756] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   10.851891] EXT4-fs (sda1): resized filesystem to 7864064
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   11.270871] init: failsafe main process (1094) killed by TERM signal
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO Running set_multiqueue.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO Set channels for eth0 to 4.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 10 05:40:44 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   12.027819] random: nonblocking pool is initialized
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Starting Google Accounts daemon.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Creating a new user account for me.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Created user account me.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Creating a new user account for henrik.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Created user account henrik.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Creating a new user account for emma.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 cron[1413]: (CRON) INFO (pidfile fd = 3)
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 cron[1452]: (CRON) STARTUP (fork ok)
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 cron[1452]: (CRON) INFO (Running @reboot jobs)
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Created user account emma.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 acpid: starting up with netlink and the input layer
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 acpid: 1 rule loaded
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 acpid: waiting for events: event logging is off
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Creating a new user account for igor.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Created user account igor.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 haveged: haveged starting up
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Created user account konstantinhaase.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Creating a new user account for aj.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   12.481068] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   12.491488] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Created user account aj.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Creating a new user account for solarce.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Created user account solarce.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Creating a new user account for asari.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Created user account asari.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Creating a new user account for bogdana.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Created user account bogdana.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Creating a new user account for konstantin.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   12.752454] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   12.757122] Bridge firewalling registered
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   12.770243] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 10 05:40:45 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Created user account konstantin.
Aug 10 05:40:46 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-clock-skew: INFO Synced system time with hardware clock.
Aug 10 05:40:46 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Creating a new user account for carmen.
Aug 10 05:40:46 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Created user account carmen.
Aug 10 05:40:46 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   12.848297] Initializing XFRM netlink socket
Aug 10 05:40:46 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   12.855734] Netfilter messages via NETLINK v0.30.
Aug 10 05:40:46 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   12.859870] ctnetlink v0.93: registering with nfnetlink.
Aug 10 05:40:46 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Creating a new user account for maria.
Aug 10 05:40:46 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   12.883500] floppy0: no floppy controllers found
Aug 10 05:40:46 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Created user account maria.
Aug 10 05:40:46 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 google-accounts: INFO Removing user packer.
Aug 10 05:41:08 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpdate[1864]: adjust time server 169.254.169.254 offset 0.005057 sec
Aug 10 05:41:15 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1900]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 10 05:41:15 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1901]: proto: precision = 0.100 usec
Aug 10 05:41:15 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1901]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 05:41:15 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1901]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 05:41:15 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1901]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 10 05:41:15 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1901]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 10 05:41:15 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1901]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 10 05:41:15 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1901]: Listen normally on 3 eth0 10.20.0.144 UDP 123
Aug 10 05:41:15 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1901]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 10 05:41:15 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1901]: peers refreshed
Aug 10 05:41:15 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1901]: Listening on routing socket on fd #21 for interface updates
Aug 10 05:41:15 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   42.684212] init: plymouth-upstart-bridge main process ended, respawning
Aug 10 05:41:16 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 startup-script: INFO Found startup-script in metadata.
Aug 10 05:41:16 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 10 05:41:16 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 startup-script: INFO startup-script: job 1 at Fri Aug 10 08:51:00 2018
Aug 10 05:41:16 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 startup-script: INFO startup-script: Return code 0.
Aug 10 05:41:16 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 startup-script: INFO startup-script: Return code 0.
Aug 10 05:41:16 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 startup-script: INFO Finished running startup scripts.
Aug 10 05:41:16 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ec2: 
Aug 10 05:41:16 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ec2: #############################################################
Aug 10 05:41:16 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 10 05:41:16 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ec2: 1024 a4:95:aa:18:d9:cc:72:f9:f9:4d:9f:1d:8b:88:5f:3d  root@travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 (DSA)
Aug 10 05:41:16 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ec2: 256 9e:ba:8d:24:a3:5f:ab:ff:84:ab:95:cd:9e:50:d6:5d  root@travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 (ECDSA)
Aug 10 05:41:16 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ec2: 256 9d:0a:b2:dc:ec:ae:42:0a:95:a1:d3:c5:e1:e9:cd:a9  root@travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 (ED25519)
Aug 10 05:41:16 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ec2: 2048 49:25:98:8b:0d:ae:99:27:d5:af:94:a5:6d:d6:0f:07  root@travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 (RSA)
Aug 10 05:41:16 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 10 05:41:16 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ec2: #############################################################
Aug 10 05:41:53 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [   80.312710] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 10 05:44:51 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [  258.121325] device vethb77859b entered promiscuous mode
Aug 10 05:44:51 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [  258.121417] docker0: port 1(vethb77859b) entered forwarding state
Aug 10 05:44:51 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [  258.121429] docker0: port 1(vethb77859b) entered forwarding state
Aug 10 05:44:51 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [  258.121729] docker0: port 1(vethb77859b) entered disabled state
Aug 10 05:44:51 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [  258.213408] cgroup: docker-runc (5035) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 10 05:44:51 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [  258.213411] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 10 05:44:51 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [  258.285268] eth0: renamed from veth67ab06d
Aug 10 05:44:51 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [  258.324360] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 10 05:44:51 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [  258.325472] docker0: port 1(vethb77859b) entered forwarding state
Aug 10 05:44:51 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [  258.325488] docker0: port 1(vethb77859b) entered forwarding state
Aug 10 05:44:51 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [  258.325515] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 10 05:44:54 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1901]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 10 05:44:54 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1901]: Listen normally on 6 docker0 fe80::42:e8ff:fe22:e57d UDP 123
Aug 10 05:44:54 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1901]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 10 05:44:54 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1901]: peers refreshed
Aug 10 05:44:54 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1901]: new interface(s) found: waking up resolver
Aug 10 05:45:06 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [  273.332714] docker0: port 1(vethb77859b) entered forwarding state
Aug 10 06:17:01 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 CRON[14760]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 10 06:25:01 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 CRON[15339]: (root) CMD (test -x /usr/sbin/anacron || ( cd / && run-parts --report /etc/cron.daily ))
Aug 10 06:36:37 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 upstart-socket-bridge[843]: Disconnected from Upstart
Aug 10 06:36:37 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 upstart-udev-bridge[696]: Disconnected from Upstart
Aug 10 06:36:37 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [ 3363.855165] init: upstart-udev-bridge main process (696) terminated with status 1
Aug 10 06:36:37 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [ 3363.855174] init: upstart-udev-bridge main process ended, respawning
Aug 10 06:36:37 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [ 3363.855334] init: upstart-socket-bridge main process (843) terminated with status 1
Aug 10 06:36:37 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [ 3363.855343] init: upstart-socket-bridge main process ended, respawning
Aug 10 06:36:37 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [ 3363.855419] init: upstart-file-bridge main process (1198) terminated with status 1
Aug 10 06:36:37 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [ 3363.855426] init: upstart-file-bridge main process ended, respawning
Aug 10 06:36:52 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 dbus[1157]: [system] Reloaded configuration
Aug 10 06:36:53 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 dbus[1157]: message repeated 10 times: [ [system] Reloaded configuration]
Aug 10 06:36:57 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[1901]: ntpd exiting on signal 15
Aug 10 06:37:04 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [ 3390.653989] init: apport post-stop process (29087) terminated with status 1
Aug 10 06:37:06 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 dbus[1157]: [system] Reloaded configuration
Aug 10 06:37:06 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 dbus[1157]: message repeated 5 times: [ [system] Reloaded configuration]
Aug 10 06:37:17 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 kernel: [ 3403.628404] systemd-udevd[31302]: starting version 204
Aug 10 06:37:17 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 dbus[1157]: [system] Reloaded configuration
Aug 10 06:37:17 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 dbus[1157]: message repeated 3 times: [ [system] Reloaded configuration]
Aug 10 06:37:19 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[31503]: ntpd 4.2.6p5@1.2349-o Fri Jul  6 20:19:54 UTC 2018 (1)
Aug 10 06:37:19 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[31504]: ntp_io: estimated max descriptors: 72000, initial socket boundary: 16
Aug 10 06:37:19 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[31504]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 10 06:37:19 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[31504]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 10 06:37:19 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[31504]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 10 06:37:19 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[31504]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 10 06:37:19 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[31504]: Listen normally on 3 eth0 10.20.0.144 UDP 123
Aug 10 06:37:19 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[31504]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 10 06:37:19 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[31504]: peers refreshed
Aug 10 06:37:19 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 ntpd[31504]: Listening on routing socket on fd #21 for interface updates
Aug 10 06:37:21 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 06:37:21 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 06:37:23 travis-job-21a1796f-5554-462a-8a04-ba14e8181c76 dbus[1157]: [system] Reloaded configuration
---
travis_time:end:1cf0911d:start=1533883995480599402,finish=1533883995489601651,duration=9002249
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a3e4e3b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14f9f790
travis_time:start:14f9f790
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:065bb39c
$ dmesg | grep -i kill
