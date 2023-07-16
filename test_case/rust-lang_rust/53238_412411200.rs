plain
[00:25:28] [RUSTC-TIMING] alloc test:false 6.681
[00:25:28]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:25:28] [RUSTC-TIMING] panic_unwind test:false 0.280

Broadcast message from root@travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b
 (unknown) at 5:20 ...
The system is going down for power off NOW!
[00:25:38] 
[00:25:38] Session terminated, terminating shell... ...terminated.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:194b873e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:1eaa9b58
$ sudo tail -n 500 /var/log/syslog
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] Policy zone: Normal
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] Hierarchical RCU implementation.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] console [ttyS0] enabled
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.910588] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.920415] pid_max: default: 32768 minimum: 301
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.923871] ACPI: Core revision 20150930
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.934619] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.942895] Security Framework initialized
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.946475] Yama: becoming mindful.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.949894] AppArmor: AppArmor disabled by boot time parameter
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.957207] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.973382] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.987115] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    0.994865] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.004970] Initializing cgroup subsys io
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.012195] Initializing cgroup subsys memory
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.017423] Initializing cgroup subsys devices
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.022098] Initializing cgroup subsys freezer
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.028650] Initializing cgroup subsys net_cls
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.032593] Initializing cgroup subsys perf_event
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.040614] Initializing cgroup subsys net_prio
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.048994] Initializing cgroup subsys hugetlb
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.054533] Initializing cgroup subsys pids
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.059347] CPU: Physical Processor ID: 0
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.063870] CPU: Processor Core ID: 0
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.069092] mce: CPU supports 32 MCE banks
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.074542] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.079820] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.088264] Freeing SMP alternatives memory: 32K
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.102885] ftrace: allocating 32185 entries in 126 pages
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.164002] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.171016] smpboot: Max logical packages: 2
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.175627] x2apic enabled
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.178182] Switched APIC routing to physical x2apic.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.186482] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.297069] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.303767] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.310434] x86: Booting SMP configuration:
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.314080] .... node  #0, CPUs:      #1
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.317346] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.325498]  #2
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.326569] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.333729]  #3
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.335021] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.343581] x86: Booted up 1 node, 4 CPUs
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.346046] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.356195] devtmpfs: initialized
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.362558] evm: security.selinux
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.365422] evm: security.SMACK64
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.368213] evm: security.SMACK64EXEC
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.370768] evm: security.SMACK64TRANSMUTE
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.373546] evm: security.SMACK64MMAP
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.376470] evm: security.ima
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.378553] evm: security.capability
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.381357] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.390412] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.399943] pinctrl core: initialized pinctrl subsystem
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.404875] RTC time:  4:54:12, date: 08/13/18
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.410298] NET: Registered protocol family 16
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.425155] cpuidle: using governor ladder
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.441169] cpuidle: using governor menu
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.443484] PCCT header not found.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.446658] ACPI: bus type PCI registered
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.449303] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.454406] PCI: Using configuration type 1 for base access
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.476055] ACPI: Added _OSI(Module Device)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.479937] ACPI: Added _OSI(Processor Device)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.483602] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.487265] ACPI: Added _OSI(Processor Aggregator Device)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.493921] ACPI: Executed 2 blocks of module-level executable AML code
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.523240] ACPI: Interpreter enabled
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.526676] ACPI: (supports S0 S3 S4 S5)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.530126] ACPI: Using IOAPIC for interrupt routing
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.534366] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.572113] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.578615] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.584025] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.588800] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.601672] PCI host bridge to bus 0000:00
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.606136] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.615384] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.624166] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.629668] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.636242] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.642286] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.642755] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.691187] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.732233] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.738759] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.763283] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.780857] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.815469] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.830650] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.842564] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.879639] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.889253] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.901257] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.912580] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.923246] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.949662] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.955728] vgaarb: loaded
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.961550] SCSI subsystem initialized
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.970185] libata version 3.00 loaded.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.970223] ACPI: bus type USB registered
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.975190] usbcore: registered new interface driver usbfs
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.981043] usbcore: registered new interface driver hub
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.987181] usbcore: registered new device driver usb
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    1.991712] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.001623] dmi: Firmware registration failed.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.008627] PCI: Using ACPI for IRQ routing
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.013248] PCI: pci_cache_line_size set to 64 bytes
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.013383] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.013386] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.013551] NetLabel: Initializing
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.020462] NetLabel:  domain hash size = 128
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.026263] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.032944] NetLabel:  unlabeled traffic allowed by default
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.039416] amd_nb: Cannot enumerate AMD northbridges
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.044985] clocksource: Switched to clocksource kvm-clock
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.058422] pnp: PnP ACPI init
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.060741] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.060815] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.060858] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.060906] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.061011] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.061058] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.061098] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.061252] pnp: PnP ACPI: found 7 devices
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.074750] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.082686] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.082689] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.082691] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.082693] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.082739] NET: Registered protocol family 2
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.086078] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.092221] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.098198] TCP: Hash tables configured (established 131072 bind 65536)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.102958] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.107004] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.117225] NET: Registered protocol family 1
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.123925] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.129170] PCI: CLS 0 bytes, default 64
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    2.129250] Unpacking initramfs...
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.241042] Freeing initrd memory: 21432K
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.246960] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.253200] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.265004] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.273886] hw unit of domain pp0-core 2^-0 Joules
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.280010] hw unit of domain package 2^-0 Joules
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.285497] hw unit of domain dram 2^-16 Joules
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.291149] Scanning for low memory corruption every 60 seconds
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.298954] audit: initializing netlink subsys (disabled)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.303411] audit: type=2000 audit(1534136054.957:1): initialized
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.309881] Initialise system trusted keyring
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.316143] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.320511] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.325821] zbud: loaded
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.328247] VFS: Disk quotas dquot_6.6.0
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.332268] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.340192] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.345125] fuse init (API version 7.23)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.349629] Key type big_key registered
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.353439] Allocating IMA MOK and blacklist keyrings.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.366689] Key type asymmetric registered
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.371459] Asymmetric key parser 'x509' registered
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.377444] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.384619] io scheduler noop registered
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.387902] io scheduler deadline registered (default)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.392449] io scheduler cfq registered
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.394867] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.400063] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.406061] intel_idle: does not run on family 6 model 63
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.406166] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.412115] ACPI: Power Button [PWRF]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.417242] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.430420] ACPI: Sleep Button [SLPF]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.437191] GHES: HEST is not enabled!
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.445653] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.452879] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.467087] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.472847] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.490759] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.518679] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.552151] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.585988] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.614737] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.631536] Linux agpgart interface v0.103
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.643126] loop: module loaded
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.645897] libphy: Fixed MDIO Bus: probed
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.649038] tun: Universal TUN/TAP device driver, 1.6
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.654869] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.737420] PPP generic driver version 2.4.2
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.742568] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.750093] ehci-pci: EHCI PCI platform driver
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.755417] ehci-platform: EHCI generic platform driver
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.761201] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.766263] ohci-pci: OHCI PCI platform driver
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.769686] ohci-platform: OHCI generic platform driver
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.775304] uhci_hcd: USB Universal Host Controller Interface driver
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.783421] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.791676] i8042: Warning: Keylock active
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.796867] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.800864] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.808093] mousedev: PS/2 mouse device common for all mice
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.816779] rtc_cmos 00:00: RTC can wake from S4
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.820517] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.826764] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.833489] i2c /dev entries driver
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.837193] device-mapper: uevent: version 1.0.3
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.843104] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.853881] ledtrig-cpu: registered to indicate activity on CPUs
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.864128] NET: Registered protocol family 10
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.868200] NET: Registered protocol family 17
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.875961] Key type dns_resolver registered
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.882776] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.888955] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.897849] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.902279] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.910210] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.920300] registered taskstats version 1
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.928851] Loading compiled-in X.509 certificates
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.942902] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.953791] zswap: loaded using pool lzo/zbud
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.962655] Key type trusted registered
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.975439] Key type encrypted registered
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.980230] ima: No TPM chip found, activating TPM-bypass!
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.985851] evm: HMAC attrs: 0x1
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.990172]   Magic number: 14:301:919
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    4.994495] rtc_cmos 00:00: setting system clock to 2018-08-13 04:54:16 UTC (1534136056)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.002767] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.008547] EDD information not available.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.011612] PM: Hibernation image not present or could not be loaded.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.013257] Freeing unused kernel memory: 1496K
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.018454] Write protecting the kernel read-only data: 14336k
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.024103] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.024570] Freeing unused kernel memory: 1956K
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.036845] Freeing unused kernel memory: 92K
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.068014] systemd-udevd[119]: starting version 204
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.173604] scsi host0: Virtio SCSI HBA
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.190711] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.204155] AVX2 version of gcm_enc/dec engaged.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.207916] AES CTR mode by8 optimization enabled
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.289115] tsc: Refined TSC clocksource calibration: 2300.007 MHz
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.297291] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x21273b92eef, max_idle_ns: 440795226594 ns
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.426697] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.428016] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.428018] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.432021] sd 0:0:1:0: [sda] Write Protect is off
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.432023] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.433665] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.444186]  sda: sda1
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    5.450694] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    6.001553] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    8.145244] floppy0: no floppy controllers found
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    9.329011] raid6: sse2x1   gen()  8490 MB/s
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    9.400998] raid6: sse2x1   xor()  6392 MB/s
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    9.468994] raid6: sse2x2   gen() 10381 MB/s
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    9.537000] raid6: sse2x2   xor()  6950 MB/s
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    9.604994] raid6: sse2x4   gen() 12370 MB/s
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    9.677001] raid6: sse2x4   xor()  8712 MB/s
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    9.745000] raid6: avx2x1   gen() 16592 MB/s
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    9.817001] raid6: avx2x2   gen() 19456 MB/s
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    9.885001] raid6: avx2x4   gen() 22047 MB/s
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    9.889283] raid6: using algorithm avx2x4 gen() 22047 MB/s
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    9.893277] raid6: using avx2x2 recovery algorithm
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    9.898853] xor: automatically using best checksumming function:
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    9.944995]    avx       : 26681.000 MB/sec
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [    9.963437] Btrfs loaded
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   10.051932] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   10.057382] EXT4-fs (sda1): write access will be enabled during recovery
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   10.183324] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   10.196389] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   10.200795] EXT4-fs (sda1): recovery complete
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   10.212005] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   10.537481] random: init: uninitialized urandom read (12 bytes read, 23 bits of entropy available)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   10.729846] random: mountall: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   10.804535] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   11.082143] random: cloud-init: uninitialized urandom read (32 bytes read, 33 bits of entropy available)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   11.824101] random: cloud-init: uninitialized urandom read (32 bytes read, 40 bits of entropy available)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   12.003095] systemd-udevd[701]: starting version 204
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   12.158621] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   12.203982] intel_rapl: no valid rapl domains found in package 0
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   12.272638] ppdev: user-space parallel port driver
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   12.422992] random: mktemp: uninitialized urandom read (6 bytes read, 49 bits of entropy available)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   12.497687] random: mktemp: uninitialized urandom read (6 bytes read, 50 bits of entropy available)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   12.578187] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   12.754145] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   13.047544] random: mktemp: uninitialized urandom read (12 bytes read, 54 bits of entropy available)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   13.137194] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   13.224976] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   13.265224] EXT4-fs (sda1): resized filesystem to 7864064
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   13.567595] init: failsafe main process (1095) killed by TERM signal
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO Running set_multiqueue.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO Set channels for eth0 to 4.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Starting Google Accounts daemon.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Creating a new user account for me.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Created user account me.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Creating a new user account for henrik.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Created user account henrik.
Aug 13 04:54:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Creating a new user account for emma.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-clock-skew: INFO Synced system time with hardware clock.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Created user account emma.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Creating a new user account for igor.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Created user account igor.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Created user account konstantinhaase.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Creating a new user account for aj.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Created user account aj.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Creating a new user account for solarce.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Created user account solarce.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Creating a new user account for asari.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Created user account asari.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Creating a new user account for bogdana.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Created user account bogdana.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Creating a new user account for konstantin.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Created user account konstantin.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Creating a new user account for carmen.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Created user account carmen.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Creating a new user account for maria.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Created user account maria.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   15.151678] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   15.154499] Bridge firewalling registered
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   15.164666] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b google-accounts: INFO Removing user packer.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   15.196194] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   15.241153] floppy0: no floppy controllers found
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   15.272787] Initializing XFRM netlink socket
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   15.281618] Netfilter messages via NETLINK v0.30.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   15.284437] ctnetlink v0.93: registering with nfnetlink.
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 04:54:26 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 04:54:27 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   15.504619] random: nonblocking pool is initialized
Aug 13 04:54:31 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b cron[1710]: (CRON) INFO (pidfile fd = 3)
Aug 13 04:54:31 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b cron[1740]: (CRON) STARTUP (fork ok)
Aug 13 04:54:31 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b cron[1740]: (CRON) INFO (Running @reboot jobs)
Aug 13 04:54:31 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 04:54:31 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 04:54:31 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b acpid: starting up with netlink and the input layer
Aug 13 04:54:31 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b acpid: 1 rule loaded
Aug 13 04:54:31 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b acpid: waiting for events: event logging is off
Aug 13 04:54:31 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b haveged: haveged starting up
Aug 13 04:54:31 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   20.418895] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpd[1857]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpd[1858]: proto: precision = 0.197 usec
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpd[1858]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpd[1858]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpd[1858]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpd[1858]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpd[1858]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpd[1858]: Listen normally on 3 eth0 10.20.0.233 UDP 123
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpd[1858]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpd[1858]: peers refreshed
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpd[1858]: Listening on routing socket on fd #21 for interface updates
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   25.649766] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b startup-script: INFO Found startup-script in metadata.
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b startup-script: INFO startup-script: job 1 at Mon Aug 13 08:04:00 2018
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b startup-script: INFO startup-script: Return code 0.
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b startup-script: INFO startup-script: Return code 0.
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b startup-script: INFO Finished running startup scripts.
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ec2: 
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ec2: #############################################################
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ec2: 1024 7e:82:42:82:93:16:80:1a:e7:10:4f:31:9f:ec:28:d7  root@travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b (DSA)
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ec2: 256 fb:ae:3a:a5:e6:73:3f:c2:03:cc:c9:2b:49:17:41:35  root@travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b (ECDSA)
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ec2: 256 2e:00:7d:4f:ce:02:87:85:9d:b8:5c:60:95:dd:1b:d1  root@travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b (ED25519)
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ec2: 2048 e6:ea:38:1a:a9:c3:81:62:5b:17:32:05:72:1f:e0:96  root@travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b (RSA)
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 13 04:54:37 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ec2: #############################################################
Aug 13 04:54:41 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpdate[2521]: the NTP socket is in use, exiting
Aug 13 04:55:14 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [   62.598826] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 13 04:56:09 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [  118.222156] device veth1e4ebcc entered promiscuous mode
Aug 13 04:56:09 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [  118.222213] docker0: port 1(veth1e4ebcc) entered forwarding state
Aug 13 04:56:09 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [  118.222224] docker0: port 1(veth1e4ebcc) entered forwarding state
Aug 13 04:56:09 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [  118.223257] docker0: port 1(veth1e4ebcc) entered disabled state
Aug 13 04:56:09 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [  118.321738] cgroup: docker-runc (4799) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 13 04:56:09 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [  118.321741] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 13 04:56:09 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [  118.385272] eth0: renamed from veth9e18672
Aug 13 04:56:09 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [  118.420928] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 13 04:56:09 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [  118.421983] docker0: port 1(veth1e4ebcc) entered forwarding state
Aug 13 04:56:09 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [  118.422000] docker0: port 1(veth1e4ebcc) entered forwarding state
Aug 13 04:56:09 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [  118.422019] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 13 04:56:13 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpd[1858]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 13 04:56:13 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpd[1858]: Listen normally on 6 docker0 fe80::42:5fff:feb6:b054 UDP 123
Aug 13 04:56:13 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpd[1858]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 13 04:56:13 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpd[1858]: peers refreshed
Aug 13 04:56:13 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b ntpd[1858]: new interface(s) found: waking up resolver
Aug 13 04:56:25 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [  133.436813] docker0: port 1(veth1e4ebcc) entered forwarding state
Aug 13 05:17:01 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b CRON[24093]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 13 05:20:52 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b dbus[1151]: [system] Activating service name='org.freedesktop.systemd1' (using servicehelper)
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b dbus[1151]: [system] Successfully activated service 'org.freedesktop.systemd1'
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.552920] init: tty4 main process (1705) killed by TERM signal
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.553199] init: tty5 main process (1708) killed by TERM signal
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.553383] init: tty2 main process (1712) killed by TERM signal
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.553557] init: tty3 main process (1713) killed by TERM signal
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.553726] init: tty6 main process (1715) killed by TERM signal
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.554059] init: cron main process (1740) killed by TERM signal
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.554367] init: irqbalance main process (1765) killed by TERM signal
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.554533] init: tty1 main process (1897) killed by TERM signal
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.554702] init: ttyS0 main process (1906) killed by TERM signal
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.555187] init: plymouth-upstart-bridge main process (3513) terminated with status 1
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.555194] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.618129] init: plymouth-upstart-bridge main process (3560) terminated with status 1
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.618137] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.631067] init: plymouth-upstart-bridge main process (3576) terminated with status 1
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.631095] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.636683] veth9e18672: renamed from eth0
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.638840] init: plymouth-upstart-bridge main process (3580) terminated with status 1
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.638847] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.644807] init: plymouth-upstart-bridge main process (3583) terminated with status 1
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.644814] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.654216] init: apport post-stop process (3506) terminated with status 1
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.655814] init: plymouth-upstart-bridge main process (3587) terminated with status 1
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.655822] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.664176] init: plymouth-upstart-bridge main process (3595) terminated with status 1
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.664184] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.669806] init: plymouth-upstart-bridge main process (3599) terminated with status 1
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.669814] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.675154] init: plymouth-upstart-bridge main process (3601) terminated with status 1
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.675161] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.676928] docker0: port 1(veth1e4ebcc) entered disabled state
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b shutdown-script: INFO Starting shutdown scripts.
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.682266] init: plymouth-upstart-bridge main process (3603) terminated with status 1
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.682274] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b shutdown-script: INFO No shutdown scripts found in metadata.
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b shutdown-script: INFO Finished running shutdown scripts.
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.686506] init: plymouth-upstart-bridge main process (3606) terminated with status 1
Aug 13 05:20:53 travis-job-20d4b90f-ea5b-4194-9cc5-cac84ae9a17b kernel: [ 1601.686515] init: plymouth-upstart-bridge respawning too fast, stopped
---
travis_time:end:0228f2a8:start=1534137653910741003,finish=1534137653918537442,duration=7796439
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0df61788
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:089b7888
travis_time:start:089b7888
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:05b26cd8
$ dmesg | grep -i kill
$ dmesg | grep -i kill
[   13.567595] init: failsafe main process (1095) killed by TERM signal
[ 1601.552920] init: tty4 main process (1705) killed by TERM signal
[ 1601.553199] init: tty5 main process (1708) killed by TERM signal
[ 1601.553383] init: tty2 main process (1712) killed by TERM signal
[ 1601.553557] init: tty3 main process (1713) killed by TERM signal
[ 1601.553726] init: tty6 main process (1715) killed by TERM signal
[ 1601.554059] init: cron main process (1740) killed by TERM signal
[ 1601.554367] init: irqbalance main process (1765) killed by TERM signal
[ 1601.554533] init: tty1 main process (1897) killed by TERM signal
[ 1601.554702] init: ttyS0 main process (1906) killed by TERM signal
[ 1601.707335] init: wait-for-state (google-shutdown-scripts-block-rsysloggoogle-shutdown-scripts) main process (3586) killed by TERM signal
[ 1601.769401] init: wait-for-state (rcplymouth-shutdown) main process (3623) killed by TERM signal
travis_fold:end:after_failure.7

Done. Your build exited with 1.
