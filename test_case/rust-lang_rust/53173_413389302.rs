plain
[00:27:39]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:27:40]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:27:40]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)

Broadcast message from root@travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba
 (unknown) at 1:06 ...
The system is going down for power off NOW!
[00:28:05] 
[00:28:05] Session terminated, terminating shell... ...terminated.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:066003a8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0570f832
$ sudo tail -n 500 /var/log/syslog
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] Policy zone: Normal
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] Hierarchical RCU implementation.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] console [ttyS0] enabled
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.340458] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.341870] pid_max: default: 32768 minimum: 301
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.343125] ACPI: Core revision 20150930
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.350241] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.351889] Security Framework initialized
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.352556] Yama: becoming mindful.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.353088] AppArmor: AppArmor disabled by boot time parameter
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.355915] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.365463] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.369723] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.371292] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.373578] Initializing cgroup subsys io
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.374245] Initializing cgroup subsys memory
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.374878] Initializing cgroup subsys devices
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.375880] Initializing cgroup subsys freezer
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.376594] Initializing cgroup subsys net_cls
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.377275] Initializing cgroup subsys perf_event
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.377923] Initializing cgroup subsys net_prio
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.378719] Initializing cgroup subsys hugetlb
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.379636] Initializing cgroup subsys pids
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.380343] CPU: Physical Processor ID: 0
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.380923] CPU: Processor Core ID: 0
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.382317] mce: CPU supports 32 MCE banks
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.383045] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.384301] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.387802] Freeing SMP alternatives memory: 32K
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.397906] ftrace: allocating 32185 entries in 126 pages
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.455753] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.457089] smpboot: Max logical packages: 2
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.458259] x2apic enabled
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.459760] Switched APIC routing to physical x2apic.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.463387] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.571544] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.573053] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.575514] x86: Booting SMP configuration:
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.576206] .... node  #0, CPUs:      #1
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.576995] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.581272]  #2
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.581772] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.586409]  #3
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.586850] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.590897] x86: Booted up 1 node, 4 CPUs
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.591493] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.593774] devtmpfs: initialized
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.597993] evm: security.selinux
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.598561] evm: security.SMACK64
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.599041] evm: security.SMACK64EXEC
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.599628] evm: security.SMACK64TRANSMUTE
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.600295] evm: security.SMACK64MMAP
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.600835] evm: security.ima
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.601317] evm: security.capability
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.602125] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.603913] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.605262] pinctrl core: initialized pinctrl subsystem
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.606264] RTC time:  0:37:24, date: 08/16/18
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.607731] NET: Registered protocol family 16
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.619586] cpuidle: using governor ladder
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.631577] cpuidle: using governor menu
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.632193] PCCT header not found.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.632798] ACPI: bus type PCI registered
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.633471] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.634703] PCI: Using configuration type 1 for base access
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.648615] ACPI: Added _OSI(Module Device)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.649431] ACPI: Added _OSI(Processor Device)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.650247] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.651019] ACPI: Added _OSI(Processor Aggregator Device)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.654406] ACPI: Executed 2 blocks of module-level executable AML code
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.677820] ACPI: Interpreter enabled
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.678457] ACPI: (supports S0 S3 S4 S5)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.679034] ACPI: Using IOAPIC for interrupt routing
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.679808] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.709675] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.710801] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.711866] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.712898] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.715671] PCI host bridge to bus 0000:00
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.716293] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.717396] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.718399] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.719882] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.721050] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.721978] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.722435] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.737292] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.751637] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.753050] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.758784] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.763643] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.776255] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.781458] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.785553] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.798893] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.801309] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.803523] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.805698] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.807618] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.828092] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.829081] vgaarb: loaded
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.829883] SCSI subsystem initialized
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.830605] libata version 3.00 loaded.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.830627] ACPI: bus type USB registered
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.831333] usbcore: registered new interface driver usbfs
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.832162] usbcore: registered new interface driver hub
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.833037] usbcore: registered new device driver usb
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.833980] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.834986] dmi: Firmware registration failed.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.835756] PCI: Using ACPI for IRQ routing
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.836383] PCI: pci_cache_line_size set to 64 bytes
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.836477] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.836479] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.836587] NetLabel: Initializing
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.837158] NetLabel:  domain hash size = 128
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.837850] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.838551] NetLabel:  unlabeled traffic allowed by default
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.839601] amd_nb: Cannot enumerate AMD northbridges
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.840471] clocksource: Switched to clocksource kvm-clock
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.848312] pnp: PnP ACPI init
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.849006] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.849078] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.849125] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.849175] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.849231] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.849277] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.849322] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.849480] pnp: PnP ACPI: found 7 devices
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.857349] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.858621] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.858623] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.858625] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.858626] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.858667] NET: Registered protocol family 2
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.859640] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.861491] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.862789] TCP: Hash tables configured (established 131072 bind 65536)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.863862] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.864770] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.866563] NET: Registered protocol family 1
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.867280] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.868403] PCI: CLS 0 bytes, default 64
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    0.868515] Unpacking initramfs...
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.923367] Freeing initrd memory: 21432K
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.924127] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.925226] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.926800] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.928154] hw unit of domain pp0-core 2^-0 Joules
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.928973] hw unit of domain package 2^-0 Joules
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.929724] hw unit of domain dram 2^-16 Joules
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.930550] Scanning for low memory corruption every 60 seconds
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.931828] audit: initializing netlink subsys (disabled)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.932621] audit: type=2000 audit(1534379847.035:1): initialized
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.933779] Initialise system trusted keyring
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.934822] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.935741] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.937810] zbud: loaded
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.938525] VFS: Disk quotas dquot_6.6.0
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.939120] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.940402] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.941690] fuse init (API version 7.23)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.942446] Key type big_key registered
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.943106] Allocating IMA MOK and blacklist keyrings.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.944912] Key type asymmetric registered
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.945510] Asymmetric key parser 'x509' registered
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.946855] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.948208] io scheduler noop registered
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.948814] io scheduler deadline registered (default)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.949598] io scheduler cfq registered
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.950220] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.951079] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.952224] intel_idle: does not run on family 6 model 63
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.952313] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.953515] ACPI: Power Button [PWRF]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.954130] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.955296] ACPI: Sleep Button [SLPF]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.956253] GHES: HEST is not enabled!
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.958385] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.959502] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.963745] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.965063] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.970104] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    2.992643] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.015748] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.038911] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.062275] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.065846] Linux agpgart interface v0.103
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.069377] loop: module loaded
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.070586] libphy: Fixed MDIO Bus: probed
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.071482] tun: Universal TUN/TAP device driver, 1.6
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.073153] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.109487] PPP generic driver version 2.4.2
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.110693] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.112419] ehci-pci: EHCI PCI platform driver
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.113561] ehci-platform: EHCI generic platform driver
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.114597] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.116261] ohci-pci: OHCI PCI platform driver
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.117277] ohci-platform: OHCI generic platform driver
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.118671] uhci_hcd: USB Universal Host Controller Interface driver
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.120272] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.122736] i8042: Warning: Keylock active
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.124973] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.126314] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.128056] mousedev: PS/2 mouse device common for all mice
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.129465] rtc_cmos 00:00: RTC can wake from S4
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.130842] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.132733] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.134364] i2c /dev entries driver
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.135245] device-mapper: uevent: version 1.0.3
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.136628] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.138419] ledtrig-cpu: registered to indicate activity on CPUs
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.140345] NET: Registered protocol family 10
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.141867] NET: Registered protocol family 17
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.142903] Key type dns_resolver registered
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.144260] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.145744] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.147095] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.148613] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.150348] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.152789] registered taskstats version 1
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.153989] Loading compiled-in X.509 certificates
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.156115] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.158893] zswap: loaded using pool lzo/zbud
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.162072] Key type trusted registered
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.166229] Key type encrypted registered
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.167377] ima: No TPM chip found, activating TPM-bypass!
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.168996] evm: HMAC attrs: 0x1
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.170212]   Magic number: 14:790:607
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.171581] rtc_cmos 00:00: setting system clock to 2018-08-16 00:37:27 UTC (1534379847)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.174063] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.175589] EDD information not available.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.176860] PM: Hibernation image not present or could not be loaded.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.178257] Freeing unused kernel memory: 1496K
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.178958] Write protecting the kernel read-only data: 14336k
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.180648] Freeing unused kernel memory: 1956K
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.181611] Freeing unused kernel memory: 92K
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.194826] systemd-udevd[119]: starting version 204
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.253417] scsi host0: Virtio SCSI HBA
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.262088] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.270795] AVX2 version of gcm_enc/dec engaged.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.272338] AES CTR mode by8 optimization enabled
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.310478] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.310489] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.310490] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.310752] sd 0:0:1:0: [sda] Write Protect is off
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.310754] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.310799] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.311879]  sda: sda1
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.312492] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.329036] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.928627] tsc: Refined TSC clocksource calibration: 2300.001 MHz
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    3.929979] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735f0517, max_idle_ns: 440795237604 ns
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    4.166190] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    6.268667] floppy0: no floppy controllers found
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    7.420504] raid6: sse2x1   gen()  8624 MB/s
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    7.488537] raid6: sse2x1   xor()  6703 MB/s
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    7.556506] raid6: sse2x2   gen() 10894 MB/s
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    7.624505] raid6: sse2x2   xor()  7296 MB/s
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    7.692511] raid6: sse2x4   gen() 12683 MB/s
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    7.760501] raid6: sse2x4   xor()  8764 MB/s
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    7.828503] raid6: avx2x1   gen() 16634 MB/s
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    7.896502] raid6: avx2x2   gen() 19005 MB/s
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    7.964502] raid6: avx2x4   gen() 20414 MB/s
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    7.965282] raid6: using algorithm avx2x4 gen() 20414 MB/s
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    7.966024] raid6: using avx2x2 recovery algorithm
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    7.968121] xor: automatically using best checksumming function:
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    8.008501]    avx       : 27094.000 MB/sec
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    8.022792] Btrfs loaded
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    8.060568] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    8.061913] EXT4-fs (sda1): write access will be enabled during recovery
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    8.131980] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    8.139629] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    8.140523] EXT4-fs (sda1): recovery complete
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    8.144952] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    8.349332] random: init: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    8.462552] random: mountall: uninitialized urandom read (12 bytes read, 32 bits of entropy available)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    8.511933] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    8.691071] random: cloud-init: uninitialized urandom read (32 bytes read, 40 bits of entropy available)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    9.245137] random: cloud-init: uninitialized urandom read (32 bytes read, 48 bits of entropy available)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    9.382658] systemd-udevd[701]: starting version 204
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    9.483384] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    9.555426] intel_rapl: no valid rapl domains found in package 0
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    9.595767] intel_rapl: no valid rapl domains found in package 0
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    9.643425] ppdev: user-space parallel port driver
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    9.695304] random: mktemp: uninitialized urandom read (6 bytes read, 60 bits of entropy available)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    9.744071] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    9.799250] random: cloud-init: uninitialized urandom read (32 bytes read, 61 bits of entropy available)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [    9.963195] random: cloud-init: uninitialized urandom read (32 bytes read, 61 bits of entropy available)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   10.230559] random: mktemp: uninitialized urandom read (12 bytes read, 64 bits of entropy available)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   10.303185] random: mktemp: uninitialized urandom read (6 bytes read, 65 bits of entropy available)
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   10.375739] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   10.412981] EXT4-fs (sda1): resized filesystem to 7864064
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   10.669653] init: failsafe main process (1093) killed by TERM signal
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO Running set_multiqueue.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO Set channels for eth0 to 4.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Starting Google Accounts daemon.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   11.345561] random: nonblocking pool is initialized
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Creating a new user account for me.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Created user account me.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Creating a new user account for henrik.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Created user account henrik.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Creating a new user account for emma.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Created user account emma.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Creating a new user account for igor.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Created user account igor.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 16 00:37:35 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Created user account konstantinhaase.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-clock-skew: INFO Synced system time with hardware clock.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Creating a new user account for aj.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Created user account aj.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba cron[1444]: (CRON) INFO (pidfile fd = 3)
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Creating a new user account for solarce.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba cron[1484]: (CRON) STARTUP (fork ok)
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba cron[1484]: (CRON) INFO (Running @reboot jobs)
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Created user account solarce.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba acpid: starting up with netlink and the input layer
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba acpid: 1 rule loaded
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba acpid: waiting for events: event logging is off
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Creating a new user account for asari.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba haveged: haveged starting up
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Created user account asari.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Creating a new user account for bogdana.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   11.872095] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Created user account bogdana.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   11.885211] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Creating a new user account for konstantin.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Created user account konstantin.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Creating a new user account for carmen.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Created user account carmen.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Creating a new user account for maria.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Created user account maria.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   12.050150] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   12.054787] Bridge firewalling registered
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba google-accounts: INFO Removing user packer.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   12.064628] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   12.130995] Initializing XFRM netlink socket
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   12.138862] Netfilter messages via NETLINK v0.30.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   12.142797] ctnetlink v0.93: registering with nfnetlink.
Aug 16 00:37:36 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   12.560671] floppy0: no floppy controllers found
Aug 16 00:37:59 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpdate[1866]: adjust time server 169.254.169.254 offset 0.006030 sec
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpd[1902]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpd[1903]: proto: precision = 0.127 usec
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpd[1903]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpd[1903]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpd[1903]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpd[1903]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpd[1903]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpd[1903]: Listen normally on 3 eth0 10.20.0.95 UDP 123
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpd[1903]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpd[1903]: peers refreshed
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpd[1903]: Listening on routing socket on fd #21 for interface updates
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   42.053167] init: plymouth-upstart-bridge main process ended, respawning
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba startup-script: INFO Found startup-script in metadata.
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba startup-script: INFO startup-script: job 1 at Thu Aug 16 03:48:00 2018
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba startup-script: INFO startup-script: Return code 0.
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba startup-script: INFO startup-script: Return code 0.
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba startup-script: INFO Finished running startup scripts.
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ec2: 
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ec2: #############################################################
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ec2: 1024 62:d8:37:39:e9:46:cc:a4:8e:87:26:34:e4:10:8d:8d  root@travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba (DSA)
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ec2: 256 2c:af:53:8f:f1:a5:b2:fa:51:1e:51:09:cc:af:ce:8f  root@travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba (ECDSA)
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ec2: 256 10:91:b9:f9:32:5d:00:d9:56:5c:5f:a7:42:15:ae:c6  root@travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba (ED25519)
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ec2: 2048 08:7d:8c:d2:81:6e:6b:a4:7e:43:0b:0b:ac:b9:3f:9e  root@travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba (RSA)
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 16 00:38:06 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ec2: #############################################################
Aug 16 00:38:34 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [   69.754686] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 16 00:40:50 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [  205.678462] device veth96a31b1 entered promiscuous mode
Aug 16 00:40:50 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [  205.678539] docker0: port 1(veth96a31b1) entered forwarding state
Aug 16 00:40:50 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [  205.678547] docker0: port 1(veth96a31b1) entered forwarding state
Aug 16 00:40:50 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [  205.678880] docker0: port 1(veth96a31b1) entered disabled state
Aug 16 00:40:50 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [  205.782087] cgroup: docker-runc (4977) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 16 00:40:50 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [  205.782090] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 16 00:40:50 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [  205.857635] eth0: renamed from veth207e640
Aug 16 00:40:50 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [  205.891804] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 16 00:40:50 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [  205.892753] docker0: port 1(veth96a31b1) entered forwarding state
Aug 16 00:40:50 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [  205.892773] docker0: port 1(veth96a31b1) entered forwarding state
Aug 16 00:40:50 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [  205.892801] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 16 00:40:53 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpd[1903]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 16 00:40:53 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpd[1903]: Listen normally on 6 docker0 fe80::42:dcff:fe69:1501 UDP 123
Aug 16 00:40:53 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpd[1903]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 16 00:40:53 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpd[1903]: peers refreshed
Aug 16 00:40:53 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba ntpd[1903]: new interface(s) found: waking up resolver
Aug 16 00:41:05 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [  220.925398] docker0: port 1(veth96a31b1) entered forwarding state
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba dbus[1147]: [system] Activating service name='org.freedesktop.systemd1' (using servicehelper)
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba dbus[1147]: [system] Successfully activated service 'org.freedesktop.systemd1'
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.774834] init: tty4 main process (1434) killed by TERM signal
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.775026] init: tty5 main process (1442) killed by TERM signal
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.775170] init: tty2 main process (1446) killed by TERM signal
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.775322] init: tty3 main process (1447) killed by TERM signal
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.775470] init: tty6 main process (1449) killed by TERM signal
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.775747] init: cron main process (1484) killed by TERM signal
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.776014] init: irqbalance main process (1503) killed by TERM signal
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.776203] init: tty1 main process (1942) killed by TERM signal
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.776348] init: ttyS0 main process (1951) killed by TERM signal
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.776704] init: plymouth-upstart-bridge main process (32645) terminated with status 1
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.776711] init: plymouth-upstart-bridge main process ended, respawning
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.823644] init: plymouth-upstart-bridge main process (32682) terminated with status 1
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.823654] init: plymouth-upstart-bridge main process ended, respawning
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.842520] init: plymouth-upstart-bridge main process (32693) terminated with status 1
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.842531] init: plymouth-upstart-bridge main process ended, respawning
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.858113] init: plymouth-upstart-bridge main process (32701) terminated with status 1
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.858126] init: plymouth-upstart-bridge main process ended, respawning
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.872261] docker0: port 1(veth96a31b1) entered disabled state
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.872348] veth207e640: renamed from eth0
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.872617] init: plymouth-upstart-bridge main process (32703) terminated with status 1
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.872631] init: plymouth-upstart-bridge main process ended, respawning
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.884420] init: plymouth-upstart-bridge main process (32706) terminated with status 1
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.884429] init: plymouth-upstart-bridge main process ended, respawning
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.891601] init: apport post-stop process (32638) terminated with status 1
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.891966] init: plymouth-upstart-bridge main process (32713) terminated with status 1
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.891973] init: plymouth-upstart-bridge main process ended, respawning
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.901399] init: plymouth-upstart-bridge main process (32716) terminated with status 1
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.901412] init: plymouth-upstart-bridge main process ended, respawning
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.911426] init: plymouth-upstart-bridge main process (32719) terminated with status 1
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.911438] init: plymouth-upstart-bridge main process ended, respawning
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.918967] init: plymouth-upstart-bridge main process (32722) terminated with status 1
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.918976] init: plymouth-upstart-bridge main process ended, respawning
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.921529] init: plymouth-upstart-bridge main process (32724) terminated with status 1
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.921536] init: plymouth-upstart-bridge respawning too fast, stopped
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba shutdown-script: INFO Starting shutdown scripts.
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.966126] docker0: port 1(veth96a31b1) entered disabled state
Aug 16 01:06:40 travis-job-12bf73d7-28e0-4553-8a3d-9e8cd2ac1bba kernel: [ 1755.968700] device veth96a31b1 left promiscuous mode
