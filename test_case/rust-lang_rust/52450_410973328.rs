plain
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:2b7d03f9
$ sudo tail -n 500 /var/log/syslog
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] kvm-clock: using sched offset of 1514896888 cycles
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] Zone ranges:
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000]   Device   empty
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] Movable zone start for each node
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] Early memory node ranges
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] Policy zone: Normal
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] console [ttyS0] enabled
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.308623] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.309934] pid_max: default: 32768 minimum: 301
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.310579] ACPI: Core revision 20150930
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.316902] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.317932] Security Framework initialized
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.318504] Yama: becoming mindful.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.319027] AppArmor: AppArmor disabled by boot time parameter
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.321883] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.331036] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.335552] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.336703] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.338275] Initializing cgroup subsys io
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.338879] Initializing cgroup subsys memory
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.339866] Initializing cgroup subsys devices
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.340540] Initializing cgroup subsys freezer
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.341266] Initializing cgroup subsys net_cls
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.342156] Initializing cgroup subsys perf_event
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.342823] Initializing cgroup subsys net_prio
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.343482] Initializing cgroup subsys hugetlb
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.344254] Initializing cgroup subsys pids
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.344981] CPU: Physical Processor ID: 0
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.345590] CPU: Processor Core ID: 0
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.347020] mce: CPU supports 32 MCE banks
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.347843] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.348687] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.352099] Freeing SMP alternatives memory: 32K
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.361909] ftrace: allocating 32185 entries in 126 pages
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.419882] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.421136] smpboot: Max logical packages: 2
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.422396] x2apic enabled
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.424288] Switched APIC routing to physical x2apic.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.428015] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.535806] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.537597] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.539844] x86: Booting SMP configuration:
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.540457] .... node  #0, CPUs:      #1
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.541264] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.545616]  #2
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.546084] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.550676]  #3
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.551146] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.555206] x86: Booted up 1 node, 4 CPUs
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.555854] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.558144] devtmpfs: initialized
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.562437] evm: security.selinux
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.563013] evm: security.SMACK64
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.563496] evm: security.SMACK64EXEC
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.564063] evm: security.SMACK64TRANSMUTE
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.564650] evm: security.SMACK64MMAP
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.565216] evm: security.ima
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.565680] evm: security.capability
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.566571] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.568050] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.569147] pinctrl core: initialized pinctrl subsystem
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.570088] RTC time:  6:24:35, date: 08/07/18
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.571800] NET: Registered protocol family 16
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.583855] cpuidle: using governor ladder
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.595870] cpuidle: using governor menu
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.596535] PCCT header not found.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.597225] ACPI: bus type PCI registered
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.597929] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.599057] PCI: Using configuration type 1 for base access
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.612711] ACPI: Added _OSI(Module Device)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.613386] ACPI: Added _OSI(Processor Device)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.614025] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.614684] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.618009] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.641244] ACPI: Interpreter enabled
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.641928] ACPI: (supports S0 S3 S4 S5)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.643065] ACPI: Using IOAPIC for interrupt routing
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.643787] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.673061] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.674176] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.675180] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.676157] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.678496] PCI host bridge to bus 0000:00
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.679135] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.680112] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.681073] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.682185] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.683356] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.684213] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.684637] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.698493] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.712905] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.714568] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.719745] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.723788] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.736708] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.741525] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.745210] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.756382] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.759763] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.762077] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.764100] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.766028] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.786023] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.787270] vgaarb: loaded
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.787897] SCSI subsystem initialized
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.788550] libata version 3.00 loaded.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.788571] ACPI: bus type USB registered
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.789268] usbcore: registered new interface driver usbfs
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.790080] usbcore: registered new interface driver hub
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.790960] usbcore: registered new device driver usb
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.791879] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.792850] dmi: Firmware registration failed.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.793600] PCI: Using ACPI for IRQ routing
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.794285] PCI: pci_cache_line_size set to 64 bytes
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.794388] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.794390] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.794506] NetLabel: Initializing
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.795054] NetLabel:  domain hash size = 128
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.795697] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.796471] NetLabel:  unlabeled traffic allowed by default
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.797397] amd_nb: Cannot enumerate AMD northbridges
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.798167] clocksource: Switched to clocksource kvm-clock
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.805424] pnp: PnP ACPI init
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.806019] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.806090] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.806138] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.806205] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.806253] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.806297] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.806340] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.806503] pnp: PnP ACPI: found 7 devices
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.813839] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.815207] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.815209] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.815210] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.815212] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.815244] NET: Registered protocol family 2
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.816215] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.818183] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.819200] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.820197] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.821077] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.822799] NET: Registered protocol family 1
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.823455] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.824350] PCI: CLS 0 bytes, default 64
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    0.824400] Unpacking initramfs...
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.861035] Freeing initrd memory: 21432K
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.861730] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.862749] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.864341] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.865600] hw unit of domain pp0-core 2^-0 Joules
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.866321] hw unit of domain package 2^-0 Joules
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.867052] hw unit of domain dram 2^-16 Joules
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.867797] Scanning for low memory corruption every 60 seconds
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.869192] audit: initializing netlink subsys (disabled)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.870070] audit: type=2000 audit(1533623077.673:1): initialized
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.871288] Initialise system trusted keyring
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.872204] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.873209] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.875391] zbud: loaded
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.876103] VFS: Disk quotas dquot_6.6.0
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.876720] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.878491] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.879863] fuse init (API version 7.23)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.880651] Key type big_key registered
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.881281] Allocating IMA MOK and blacklist keyrings.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.883098] Key type asymmetric registered
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.883745] Asymmetric key parser 'x509' registered
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.884527] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.885715] io scheduler noop registered
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.886309] io scheduler deadline registered (default)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.887060] io scheduler cfq registered
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.887691] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.888477] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.889491] intel_idle: does not run on family 6 model 63
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.889579] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.890652] ACPI: Power Button [PWRF]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.891273] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.892327] ACPI: Sleep Button [SLPF]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.893199] GHES: HEST is not enabled!
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.895522] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.896488] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.900534] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.901487] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.905971] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.928461] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.951301] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.974292] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    2.997323] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.000183] Linux agpgart interface v0.103
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.003163] loop: module loaded
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.003952] libphy: Fixed MDIO Bus: probed
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.004689] tun: Universal TUN/TAP device driver, 1.6
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.005626] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.040875] PPP generic driver version 2.4.2
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.041790] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.043075] ehci-pci: EHCI PCI platform driver
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.043803] ehci-platform: EHCI generic platform driver
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.044772] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.045733] ohci-pci: OHCI PCI platform driver
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.046554] ohci-platform: OHCI generic platform driver
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.047544] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.048782] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.050735] i8042: Warning: Keylock active
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.052499] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.053776] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.055011] mousedev: PS/2 mouse device common for all mice
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.056426] rtc_cmos 00:00: RTC can wake from S4
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.057577] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.059035] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.060124] i2c /dev entries driver
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.060825] device-mapper: uevent: version 1.0.3
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.061810] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.063391] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.064925] NET: Registered protocol family 10
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.065933] NET: Registered protocol family 17
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.066822] Key type dns_resolver registered
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.067931] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.068973] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.070119] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.071170] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.072749] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.074586] registered taskstats version 1
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.075283] Loading compiled-in X.509 certificates
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.076875] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.078527] zswap: loaded using pool lzo/zbud
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.080908] Key type trusted registered
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.084470] Key type encrypted registered
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.085245] ima: No TPM chip found, activating TPM-bypass!
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.086347] evm: HMAC attrs: 0x1
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.087337]   Magic number: 14:545:417
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.087995] acpi LNXCPU:dc: hash matches
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.089002] rtc_cmos 00:00: setting system clock to 2018-08-07 06:24:38 UTC (1533623078)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.090524] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.091578] EDD information not available.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.092343] PM: Hibernation image not present or could not be loaded.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.093670] Freeing unused kernel memory: 1496K
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.094356] Write protecting the kernel read-only data: 14336k
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.095895] Freeing unused kernel memory: 1956K
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.096775] Freeing unused kernel memory: 92K
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.109324] systemd-udevd[119]: starting version 204
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.171687] scsi host0: Virtio SCSI HBA
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.175485] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.179805] AVX2 version of gcm_enc/dec engaged.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.181220] AES CTR mode by8 optimization enabled
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.215330] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.215355] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.217338] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.218405] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.219101] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.219168] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.221741]  sda: sda1
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.222885] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.254635] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.866311] tsc: Refined TSC clocksource calibration: 2300.001 MHz
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    3.867350] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735f0517, max_idle_ns: 440795237604 ns
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    4.091822] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    6.166361] floppy0: no floppy controllers found
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    7.318217] raid6: sse2x1   gen()  8712 MB/s
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    7.386201] raid6: sse2x1   xor()  6854 MB/s
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    7.454193] raid6: sse2x2   gen() 11211 MB/s
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    7.522200] raid6: sse2x2   xor()  7507 MB/s
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    7.590197] raid6: sse2x4   gen() 12759 MB/s
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    7.658192] raid6: sse2x4   xor()  8903 MB/s
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    7.726202] raid6: avx2x1   gen() 16837 MB/s
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    7.794190] raid6: avx2x2   gen() 19787 MB/s
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    7.862200] raid6: avx2x4   gen() 22863 MB/s
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    7.862965] raid6: using algorithm avx2x4 gen() 22863 MB/s
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    7.863710] raid6: using avx2x2 recovery algorithm
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    7.865664] xor: automatically using best checksumming function:
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    7.906199]    avx       : 27374.000 MB/sec
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    7.920536] Btrfs loaded
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    7.962552] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    7.963586] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    8.023679] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    8.028629] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    8.029489] EXT4-fs (sda1): recovery complete
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    8.033985] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    8.226782] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    8.352603] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    8.402707] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    8.581127] random: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    9.118301] random: cloud-init: uninitialized urandom read (32 bytes read, 45 bits of entropy available)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    9.252204] systemd-udevd[702]: starting version 204
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    9.354891] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    9.421398] intel_rapl: no valid rapl domains found in package 0
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    9.490905] ppdev: user-space parallel port driver
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    9.580964] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    9.635997] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    9.701371] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [    9.873422] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   10.126459] random: mktemp: uninitialized urandom read (12 bytes read, 61 bits of entropy available)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   10.190892] random: mktemp: uninitialized urandom read (6 bytes read, 62 bits of entropy available)
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   10.258292] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   10.297948] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   10.579780] init: failsafe main process (1094) killed by TERM signal
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 rsyslogd-2307: warning: ~ action is deprecated, consider using the 'stop' statement instead [try http://www.rsyslog.com/e/2307 ]
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 rsyslogd: rsyslogd's groupid changed to 104
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 rsyslogd: rsyslogd's userid changed to 101
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO Running set_multiqueue.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO Set channels for eth0 to 4.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   11.307826] random: nonblocking pool is initialized
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 06:24:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 google-accounts: INFO Starting Google Accounts daemon.
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 cron[1387]: (CRON) INFO (pidfile fd = 3)
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 cron[1427]: (CRON) STARTUP (fork ok)
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 cron[1427]: (CRON) INFO (Running @reboot jobs)
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 google-accounts: INFO Creating a new user account for me.
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 acpid: starting up with netlink and the input layer
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 acpid: 1 rule loaded
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 acpid: waiting for events: event logging is off
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 google-accounts: INFO Created user account me.
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 haveged: haveged starting up
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 google-accounts: INFO Creating a new user account for bogdana.
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 google-accounts: INFO Created user account bogdana.
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   11.833752] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 google-accounts: INFO Creating a new user account for aj.
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   11.846824] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 google-accounts: INFO Created user account aj.
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 google-accounts: INFO Creating a new user account for asari.
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 google-accounts: INFO Created user account asari.
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   11.973270] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 google-accounts: INFO Removing user packer.
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   11.979760] Bridge firewalling registered
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   11.992054] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   12.054128] Initializing XFRM netlink socket
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   12.061054] Netfilter messages via NETLINK v0.30.
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   12.065351] ctnetlink v0.93: registering with nfnetlink.
Aug  7 06:24:47 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   12.446288] floppy0: no floppy controllers found
Aug  7 06:24:53 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpdate[975]: adjust time server 169.254.169.254 offset -0.353873 sec
Aug  7 06:25:00 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 CRON[1760]: (root) CMD (test -x /usr/sbin/anacron || ( cd / && run-parts --report /etc/cron.daily ))
Aug  7 06:25:16 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1800]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 06:25:16 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1801]: proto: precision = 0.103 usec
Aug  7 06:25:16 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1801]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 06:25:16 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1801]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 06:25:16 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1801]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 06:25:16 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1801]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  7 06:25:16 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1801]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 06:25:16 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1801]: Listen normally on 3 eth0 10.20.0.27 UDP 123
Aug  7 06:25:16 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1801]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 06:25:16 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1801]: peers refreshed
Aug  7 06:25:16 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1801]: Listening on routing socket on fd #21 for interface updates
Aug  7 06:25:17 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   42.029078] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 06:25:17 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 startup-script: INFO Starting startup scripts.
Aug  7 06:25:17 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 startup-script: INFO Found startup-script in metadata.
Aug  7 06:25:17 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 06:25:17 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 startup-script: INFO startup-script: job 1 at Tue Aug  7 09:35:00 2018
Aug  7 06:25:17 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 startup-script: INFO startup-script: Return code 0.
Aug  7 06:25:17 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 startup-script: INFO Finished running startup scripts.
Aug  7 06:25:17 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ec2: 
Aug  7 06:25:17 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ec2: #############################################################
Aug  7 06:25:17 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 06:25:17 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ec2: 1024 6d:c5:c4:c5:35:28:bc:ef:d5:32:62:b5:a5:94:98:68  root@travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 (DSA)
Aug  7 06:25:17 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ec2: 256 d0:ba:62:ee:25:d8:5a:b0:6d:47:25:5c:e2:d3:91:cf  root@travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 (ECDSA)
Aug  7 06:25:17 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ec2: 256 6b:7d:d0:81:c1:0e:e1:fe:20:f0:dd:77:73:9a:5e:fe  root@travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 (ED25519)
Aug  7 06:25:17 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ec2: 2048 f6:fa:04:e0:9b:04:53:39:3f:7c:25:7a:ea:7b:5d:d9  root@travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 (RSA)
Aug  7 06:25:17 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 06:25:17 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ec2: #############################################################
Aug  7 06:25:46 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [   71.254049] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 06:28:39 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [  244.763848] device veth1676426 entered promiscuous mode
Aug  7 06:28:39 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [  244.763948] docker0: port 1(veth1676426) entered forwarding state
Aug  7 06:28:39 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [  244.763960] docker0: port 1(veth1676426) entered forwarding state
Aug  7 06:28:39 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [  244.764406] docker0: port 1(veth1676426) entered disabled state
Aug  7 06:28:39 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [  244.869135] cgroup: docker-runc (4933) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 06:28:39 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [  244.869138] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 06:28:40 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [  244.936831] eth0: renamed from vethecf1d2f
Aug  7 06:28:40 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [  244.978390] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 06:28:40 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [  244.979382] docker0: port 1(veth1676426) entered forwarding state
Aug  7 06:28:40 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [  244.979400] docker0: port 1(veth1676426) entered forwarding state
Aug  7 06:28:40 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [  244.979463] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 06:28:43 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1801]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  7 06:28:43 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1801]: Listen normally on 6 docker0 fe80::42:65ff:fe09:5ffd UDP 123
Aug  7 06:28:43 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1801]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 06:28:43 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1801]: peers refreshed
Aug  7 06:28:43 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1801]: new interface(s) found: waking up resolver
Aug  7 06:28:55 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [  259.992096] docker0: port 1(veth1676426) entered forwarding state
Aug  7 06:56:04 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 upstart-socket-bridge[848]: Disconnected from Upstart
Aug  7 06:56:04 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 upstart-udev-bridge[696]: Disconnected from Upstart
Aug  7 06:56:04 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [ 1889.508891] init: upstart-udev-bridge main process (696) terminated with status 1
Aug  7 06:56:04 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [ 1889.508903] init: upstart-udev-bridge main process ended, respawning
Aug  7 06:56:04 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [ 1889.509006] init: upstart-socket-bridge main process (848) terminated with status 1
Aug  7 06:56:04 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [ 1889.509016] init: upstart-socket-bridge main process ended, respawning
Aug  7 06:56:04 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [ 1889.509119] init: upstart-file-bridge main process (1203) terminated with status 1
Aug  7 06:56:04 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [ 1889.509129] init: upstart-file-bridge main process ended, respawning
Aug  7 06:56:31 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 dbus[1148]: [system] Reloaded configuration
Aug  7 06:56:31 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 dbus[1148]: message repeated 16 times: [ [system] Reloaded configuration]
Aug  7 06:56:38 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[1801]: ntpd exiting on signal 15
Aug  7 06:56:54 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [ 1939.833644] init: apport post-stop process (15260) terminated with status 1
Aug  7 06:56:56 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 dbus[1148]: [system] Reloaded configuration
Aug  7 06:56:56 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 dbus[1148]: message repeated 4 times: [ [system] Reloaded configuration]
Aug  7 06:57:00 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 kernel: [ 1945.851788] systemd-udevd[17640]: starting version 204
Aug  7 06:57:01 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 dbus[1148]: [system] Reloaded configuration
Aug  7 06:57:01 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 dbus[1148]: message repeated 3 times: [ [system] Reloaded configuration]
Aug  7 06:57:01 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[17823]: ntpd 4.2.6p5@1.2349-o Fri Jul  6 20:19:54 UTC 2018 (1)
Aug  7 06:57:01 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[17824]: ntp_io: estimated max descriptors: 72000, initial socket boundary: 16
Aug  7 06:57:01 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[17824]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 06:57:01 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[17824]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  7 06:57:01 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[17824]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 06:57:01 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[17824]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 06:57:01 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[17824]: Listen normally on 3 eth0 10.20.0.27 UDP 123
Aug  7 06:57:01 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[17824]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 06:57:01 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[17824]: peers refreshed
Aug  7 06:57:01 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 ntpd[17824]: Listening on routing socket on fd #21 for interface updates
Aug  7 06:57:02 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 06:57:02 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 06:57:03 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 dbus[1148]: [system] Reloaded configuration
Aug  7 07:17:01 travis-job-6a3af157-89e3-4f2a-9af3-3b02f6421893 CRON[24871]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
---
travis_time:end:063795f0:start=1533629597040646209,finish=1533629597054151863,duration=13505654
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05296a9b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:049edf68
travis_time:start:049edf68
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0a1841ba
$ dmesg | grep -i kill
