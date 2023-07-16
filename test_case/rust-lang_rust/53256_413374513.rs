plain
[00:44:38]    Compiling syn v0.14.4
[00:44:38] [RUSTC-TIMING] env_logger test:false 2.166
[00:44:39] [RUSTC-TIMING] diff test:false 2.405

Broadcast message from root@travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511
 (unknown) at 23:42 ...
The system is going down for power off NOW!
[00:44:47] 
[00:44:47] Makefile:58: recipe for target 'check' failed
[00:44:47] Session terminated, terminating shell... ...terminated.
[00:44:47] make: *** [check] Terminated

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:186bc9b4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:1131e3ac
$ sudo tail -n 500 /var/log/syslog
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] Policy zone: Normal
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] console [ttyS0] enabled
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.348404] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.349709] pid_max: default: 32768 minimum: 301
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.350403] ACPI: Core revision 20150930
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.357309] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.358749] Security Framework initialized
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.359462] Yama: becoming mindful.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.360170] AppArmor: AppArmor disabled by boot time parameter
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.362790] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.373492] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.378748] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.380033] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.381388] Initializing cgroup subsys io
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.382115] Initializing cgroup subsys memory
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.382813] Initializing cgroup subsys devices
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.383473] Initializing cgroup subsys freezer
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.384331] Initializing cgroup subsys net_cls
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.385339] Initializing cgroup subsys perf_event
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.386184] Initializing cgroup subsys net_prio
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.387004] Initializing cgroup subsys hugetlb
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.387883] Initializing cgroup subsys pids
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.389298] CPU: Physical Processor ID: 0
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.390326] CPU: Processor Core ID: 0
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.392272] mce: CPU supports 32 MCE banks
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.393122] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.393988] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.397950] Freeing SMP alternatives memory: 32K
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.408905] ftrace: allocating 32185 entries in 126 pages
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.466704] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.467944] smpboot: Max logical packages: 2
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.469198] x2apic enabled
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.471398] Switched APIC routing to physical x2apic.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.475701] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.583558] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.585447] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.588921] x86: Booting SMP configuration:
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.589582] .... node  #0, CPUs:      #1
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.590522] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.595636]  #2
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.596210] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.601942]  #3
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.602561] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.607195] x86: Booted up 1 node, 4 CPUs
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.608099] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.611112] devtmpfs: initialized
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.616159] evm: security.selinux
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.616714] evm: security.SMACK64
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.617279] evm: security.SMACK64EXEC
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.618142] evm: security.SMACK64TRANSMUTE
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.618884] evm: security.SMACK64MMAP
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.619441] evm: security.ima
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.619965] evm: security.capability
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.621160] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.624706] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.626665] pinctrl core: initialized pinctrl subsystem
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.628215] RTC time: 22:56:52, date: 08/15/18
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.630032] NET: Registered protocol family 16
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.639606] cpuidle: using governor ladder
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.651607] cpuidle: using governor menu
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.652357] PCCT header not found.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.653273] ACPI: bus type PCI registered
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.654041] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.655345] PCI: Using configuration type 1 for base access
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.668613] ACPI: Added _OSI(Module Device)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.669440] ACPI: Added _OSI(Processor Device)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.670197] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.671359] ACPI: Added _OSI(Processor Aggregator Device)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.675077] ACPI: Executed 2 blocks of module-level executable AML code
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.699158] ACPI: Interpreter enabled
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.699803] ACPI: (supports S0 S3 S4 S5)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.700347] ACPI: Using IOAPIC for interrupt routing
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.701189] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.731698] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.732837] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.733986] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.735053] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.737760] PCI host bridge to bus 0000:00
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.738758] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.740055] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.741145] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.742355] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.743869] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.744763] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.745223] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.763847] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.782467] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.784269] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.792660] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.799774] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.817124] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.824872] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.831202] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.850936] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.853971] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.856747] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.859375] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.862101] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.883169] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.884604] vgaarb: loaded
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.886038] SCSI subsystem initialized
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.886888] libata version 3.00 loaded.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.886911] ACPI: bus type USB registered
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.887536] usbcore: registered new interface driver usbfs
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.888528] usbcore: registered new interface driver hub
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.889688] usbcore: registered new device driver usb
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.890786] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.892233] dmi: Firmware registration failed.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.893098] PCI: Using ACPI for IRQ routing
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.893971] PCI: pci_cache_line_size set to 64 bytes
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.894077] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.894080] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.894209] NetLabel: Initializing
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.894815] NetLabel:  domain hash size = 128
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.895590] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.896484] NetLabel:  unlabeled traffic allowed by default
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.897642] amd_nb: Cannot enumerate AMD northbridges
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.898625] clocksource: Switched to clocksource kvm-clock
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.907445] pnp: PnP ACPI init
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.908224] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.908292] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.908333] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.908382] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.908427] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.908466] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.908505] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.908677] pnp: PnP ACPI: found 7 devices
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.916594] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.918669] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.918671] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.918673] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.918674] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.918713] NET: Registered protocol family 2
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.919569] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.921245] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.922879] TCP: Hash tables configured (established 131072 bind 65536)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.924187] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.925272] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.926327] NET: Registered protocol family 1
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.927315] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.928402] PCI: CLS 0 bytes, default 64
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    0.929298] Unpacking initramfs...
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.073529] Freeing initrd memory: 21432K
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.074343] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.075402] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.077540] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.078873] hw unit of domain pp0-core 2^-0 Joules
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.079652] hw unit of domain package 2^-0 Joules
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.080308] hw unit of domain dram 2^-16 Joules
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.081196] Scanning for low memory corruption every 60 seconds
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.082775] audit: initializing netlink subsys (disabled)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.083662] audit: type=2000 audit(1534373814.688:1): initialized
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.084946] Initialise system trusted keyring
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.086031] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.087065] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.089141] zbud: loaded
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.089982] VFS: Disk quotas dquot_6.6.0
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.090812] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.092096] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.093407] fuse init (API version 7.23)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.094339] Key type big_key registered
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.095085] Allocating IMA MOK and blacklist keyrings.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.097237] Key type asymmetric registered
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.098133] Asymmetric key parser 'x509' registered
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.099219] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.100474] io scheduler noop registered
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.101087] io scheduler deadline registered (default)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.101974] io scheduler cfq registered
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.102783] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.103620] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.104620] intel_idle: does not run on family 6 model 63
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.104727] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.105886] ACPI: Power Button [PWRF]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.106593] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.107899] ACPI: Sleep Button [SLPF]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.108904] GHES: HEST is not enabled!
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.111481] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.112807] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.118182] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.119389] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.124991] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.147494] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.171703] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.195886] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.219665] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.223551] Linux agpgart interface v0.103
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.228857] loop: module loaded
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.230030] libphy: Fixed MDIO Bus: probed
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.231087] tun: Universal TUN/TAP device driver, 1.6
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.232324] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.278384] PPP generic driver version 2.4.2
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.279714] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.281210] ehci-pci: EHCI PCI platform driver
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.282311] ehci-platform: EHCI generic platform driver
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.283621] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.285203] ohci-pci: OHCI PCI platform driver
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.286559] ohci-platform: OHCI generic platform driver
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.287765] uhci_hcd: USB Universal Host Controller Interface driver
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.289410] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.292546] i8042: Warning: Keylock active
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.294978] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.296368] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.298387] mousedev: PS/2 mouse device common for all mice
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.300386] rtc_cmos 00:00: RTC can wake from S4
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.301969] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.304143] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.305933] i2c /dev entries driver
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.307019] device-mapper: uevent: version 1.0.3
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.308419] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.310985] ledtrig-cpu: registered to indicate activity on CPUs
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.313043] NET: Registered protocol family 10
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.314670] NET: Registered protocol family 17
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.315846] Key type dns_resolver registered
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.317191] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.318719] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.319958] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.321554] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.323213] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.325597] registered taskstats version 1
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.326760] Loading compiled-in X.509 certificates
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.328623] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.331076] zswap: loaded using pool lzo/zbud
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.334066] Key type trusted registered
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.338756] Key type encrypted registered
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.339871] ima: No TPM chip found, activating TPM-bypass!
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.341408] evm: HMAC attrs: 0x1
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.342467]   Magic number: 14:365:957
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.343647] rtc_cmos 00:00: setting system clock to 2018-08-15 22:56:55 UTC (1534373815)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.345920] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.347359] EDD information not available.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.348404] PM: Hibernation image not present or could not be loaded.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.349867] Freeing unused kernel memory: 1496K
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.350545] Write protecting the kernel read-only data: 14336k
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.352698] Freeing unused kernel memory: 1956K
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.353582] Freeing unused kernel memory: 92K
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.367620] systemd-udevd[119]: starting version 204
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.434138] scsi host0: Virtio SCSI HBA
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.447306] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.450230] AVX2 version of gcm_enc/dec engaged.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.451352] AES CTR mode by8 optimization enabled
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.487128] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.487266] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.487268] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.487705] sd 0:0:1:0: [sda] Write Protect is off
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.487707] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.487957] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.489837]  sda: sda1
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.490614] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    3.503101] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    4.078772] tsc: Refined TSC clocksource calibration: 2299.830 MHz
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    4.080366] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212694ad381, max_idle_ns: 440795313445 ns
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    4.339815] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    6.418812] floppy0: no floppy controllers found
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    7.570676] raid6: sse2x1   gen()  9029 MB/s
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    7.638669] raid6: sse2x1   xor()  6879 MB/s
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    7.706660] raid6: sse2x2   gen() 11436 MB/s
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    7.774659] raid6: sse2x2   xor()  7892 MB/s
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    7.842669] raid6: sse2x4   gen() 12509 MB/s
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    7.910663] raid6: sse2x4   xor()  8806 MB/s
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    7.978666] raid6: avx2x1   gen() 16423 MB/s
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.046747] raid6: avx2x2   gen() 20860 MB/s
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.114668] raid6: avx2x4   gen() 21549 MB/s
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.115524] raid6: using algorithm avx2x4 gen() 21549 MB/s
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.117211] raid6: using avx2x2 recovery algorithm
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.119883] xor: automatically using best checksumming function:
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.158661]    avx       : 22083.000 MB/sec
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.173322] Btrfs loaded
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.213297] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.214500] EXT4-fs (sda1): write access will be enabled during recovery
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.266082] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.273422] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.274242] EXT4-fs (sda1): recovery complete
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.278687] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.490714] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.608000] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.662688] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    8.850774] random: cloud-init: uninitialized urandom read (32 bytes read, 34 bits of entropy available)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    9.428062] random: cloud-init: uninitialized urandom read (32 bytes read, 42 bits of entropy available)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    9.565861] systemd-udevd[701]: starting version 204
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    9.675962] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    9.729821] intel_rapl: no valid rapl domains found in package 0
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    9.791825] ppdev: user-space parallel port driver
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    9.922489] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [    9.978444] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   10.046877] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   10.214087] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   10.545352] random: mktemp: uninitialized urandom read (12 bytes read, 57 bits of entropy available)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   10.627988] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   10.712443] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   10.750288] EXT4-fs (sda1): resized filesystem to 7864064
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   11.098377] init: failsafe main process (1092) killed by TERM signal
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO Running set_multiqueue.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO Set channels for eth0 to 4.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Starting Google Accounts daemon.
Aug 15 22:57:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Creating a new user account for me.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Created user account me.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   11.911344] random: nonblocking pool is initialized
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Creating a new user account for henrik.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Created user account henrik.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Creating a new user account for emma.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Created user account emma.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Creating a new user account for igor.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Created user account igor.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Created user account konstantinhaase.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-clock-skew: INFO Synced system time with hardware clock.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Creating a new user account for aj.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Created user account aj.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Creating a new user account for solarce.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 cron[1452]: (CRON) INFO (pidfile fd = 3)
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 cron[1491]: (CRON) STARTUP (fork ok)
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 cron[1491]: (CRON) INFO (Running @reboot jobs)
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Created user account solarce.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Creating a new user account for asari.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 acpid: starting up with netlink and the input layer
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 acpid: 1 rule loaded
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 acpid: waiting for events: event logging is off
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Created user account asari.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Creating a new user account for bogdana.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 haveged: haveged starting up
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Created user account bogdana.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Creating a new user account for konstantin.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   12.457879] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   12.473729] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Created user account konstantin.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Creating a new user account for carmen.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   12.500206] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   12.511676] Bridge firewalling registered
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   12.521559] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Created user account carmen.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Creating a new user account for maria.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Created user account maria.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   12.605069] Initializing XFRM netlink socket
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   12.613976] Netfilter messages via NETLINK v0.30.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   12.618802] ctnetlink v0.93: registering with nfnetlink.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 google-accounts: INFO Removing user packer.
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   12.750700] floppy0: no floppy controllers found
Aug 15 22:57:04 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   12.750921] work still pending
Aug 15 22:57:27 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpdate[1853]: adjust time server 169.254.169.254 offset 0.006053 sec
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpd[1888]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpd[1889]: proto: precision = 0.102 usec
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpd[1889]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpd[1889]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpd[1889]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpd[1889]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpd[1889]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpd[1889]: Listen normally on 3 eth0 10.20.2.2 UDP 123
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpd[1889]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpd[1889]: peers refreshed
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpd[1889]: Listening on routing socket on fd #21 for interface updates
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   42.666022] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 startup-script: INFO Found startup-script in metadata.
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 startup-script: INFO startup-script: job 1 at Thu Aug 16 02:07:00 2018
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 startup-script: INFO startup-script: Return code 0.
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 startup-script: INFO startup-script: Return code 0.
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 startup-script: INFO Finished running startup scripts.
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ec2: 
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ec2: #############################################################
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ec2: 1024 7c:f6:67:4e:8b:94:fd:e2:58:7c:33:ca:91:bc:8d:21  root@travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 (DSA)
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ec2: 256 f7:ae:16:59:0b:55:ac:34:08:6c:61:b2:58:ba:58:87  root@travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 (ECDSA)
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ec2: 256 ed:20:72:56:9b:bf:e4:ed:38:74:b3:c2:eb:4f:cc:d3  root@travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 (ED25519)
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ec2: 2048 4f:15:d2:89:78:0e:e3:ae:30:6f:51:7f:c8:e0:0f:27  root@travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 (RSA)
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 15 22:57:34 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ec2: #############################################################
Aug 15 22:58:03 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [   72.081856] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 15 22:59:49 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [  178.001037] device veth118dd28 entered promiscuous mode
Aug 15 22:59:49 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [  178.001115] docker0: port 1(veth118dd28) entered forwarding state
Aug 15 22:59:49 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [  178.001127] docker0: port 1(veth118dd28) entered forwarding state
Aug 15 22:59:49 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [  178.001499] docker0: port 1(veth118dd28) entered disabled state
Aug 15 22:59:49 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [  178.095550] cgroup: docker-runc (4967) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 15 22:59:49 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [  178.095553] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 15 22:59:50 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [  178.162870] eth0: renamed from vethd8f03c0
Aug 15 22:59:50 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [  178.199471] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 15 22:59:50 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [  178.200620] docker0: port 1(veth118dd28) entered forwarding state
Aug 15 22:59:50 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [  178.200637] docker0: port 1(veth118dd28) entered forwarding state
Aug 15 22:59:50 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [  178.200666] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 15 22:59:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpd[1889]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 15 22:59:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpd[1889]: Listen normally on 6 docker0 fe80::42:42ff:fee5:17ae UDP 123
Aug 15 22:59:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpd[1889]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 15 22:59:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpd[1889]: peers refreshed
Aug 15 22:59:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 ntpd[1889]: new interface(s) found: waking up resolver
Aug 15 23:00:05 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [  193.218582] docker0: port 1(veth118dd28) entered forwarding state
Aug 15 23:17:01 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 CRON[13393]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 dbus[1152]: [system] Activating service name='org.freedesktop.systemd1' (using servicehelper)
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 dbus[1152]: [system] Successfully activated service 'org.freedesktop.systemd1'
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.807390] init: tty4 main process (1444) killed by TERM signal
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.807486] init: tty5 main process (1447) killed by TERM signal
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.807562] init: tty2 main process (1456) killed by TERM signal
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.807636] init: tty3 main process (1457) killed by TERM signal
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.807713] init: tty6 main process (1459) killed by TERM signal
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.807855] init: cron main process (1491) killed by TERM signal
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.807995] init: irqbalance main process (1502) killed by TERM signal
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.808068] init: tty1 main process (1928) killed by TERM signal
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.808147] init: ttyS0 main process (1937) killed by TERM signal
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.808354] init: plymouth-upstart-bridge main process (22225) terminated with status 1
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.808361] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.844215] init: plymouth-upstart-bridge main process (22256) terminated with status 1
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.844228] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.863496] init: plymouth-upstart-bridge main process (22267) terminated with status 1
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.863517] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.870680] init: plymouth-upstart-bridge main process (22274) terminated with status 1
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.870691] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.875161] init: plymouth-upstart-bridge main process (22276) terminated with status 1
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.875172] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.882035] init: apport post-stop process (22218) terminated with status 1
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.882393] init: plymouth-upstart-bridge main process (22280) terminated with status 1
Aug 15 23:42:52 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.882402] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 23:42:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.897964] init: plymouth-upstart-bridge main process (22293) terminated with status 1
Aug 15 23:42:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.897974] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 23:42:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.898184] vethd8f03c0: renamed from eth0
Aug 15 23:42:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.904390] init: plymouth-upstart-bridge main process (22297) terminated with status 1
Aug 15 23:42:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.904401] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 23:42:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.908140] init: plymouth-upstart-bridge main process (22299) terminated with status 1
Aug 15 23:42:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.908152] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 23:42:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.910484] init: plymouth-upstart-bridge main process (22301) terminated with status 1
Aug 15 23:42:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.910494] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 23:42:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.913817] init: plymouth-upstart-bridge main process (22302) terminated with status 1
Aug 15 23:42:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.913824] init: plymouth-upstart-bridge respawning too fast, stopped
Aug 15 23:42:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 kernel: [ 2760.953907] docker0: port 1(veth118dd28) entered disabled state
Aug 15 23:42:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 shutdown-script: INFO Starting shutdown scripts.
Aug 15 23:42:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 shutdown-script: INFO No shutdown scripts found in metadata.
Aug 15 23:42:53 travis-job-985a6165-18d4-4c3a-83ea-c5ca7e2dc511 shutdown-script: INFO Finished running shutdown scripts.
