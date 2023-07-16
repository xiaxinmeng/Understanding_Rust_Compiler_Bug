plain
[00:46:58] [RUSTC-TIMING] proc_macro test:false 10.617
[00:46:58]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:47:21] [RUSTC-TIMING] syntax_ext test:false 22.947

Broadcast message from root@travis-job-762c7a94-2056-42ad-9a39-ba422d568555
 (unknown) at 10:33 ...
The system is going down for power off NOW!
[00:47:28] 
[00:47:28] Session terminated, terminating shell... ...terminated.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:0acf6758
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:1945a04f
$ sudo tail -n 500 /var/log/syslog
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] Policy zone: Normal
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] console [ttyS0] enabled
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.511032] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.515909] pid_max: default: 32768 minimum: 301
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.519003] ACPI: Core revision 20150930
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.528407] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.532972] Security Framework initialized
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.536145] Yama: becoming mindful.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.538894] AppArmor: AppArmor disabled by boot time parameter
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.543794] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.558296] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.568402] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.573888] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.577492] Initializing cgroup subsys io
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.580126] Initializing cgroup subsys memory
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.582579] Initializing cgroup subsys devices
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.585722] Initializing cgroup subsys freezer
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.588497] Initializing cgroup subsys net_cls
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.590409] Initializing cgroup subsys perf_event
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.592372] Initializing cgroup subsys net_prio
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.594728] Initializing cgroup subsys hugetlb
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.597406] Initializing cgroup subsys pids
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.599875] CPU: Physical Processor ID: 0
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.601787] CPU: Processor Core ID: 0
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.603490] mce: CPU supports 32 MCE banks
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.606133] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.608740] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.614183] Freeing SMP alternatives memory: 32K
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.626731] ftrace: allocating 32185 entries in 126 pages
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.687996] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.690718] smpboot: Max logical packages: 2
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.693252] x2apic enabled
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.696948] Switched APIC routing to physical x2apic.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.702113] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.811714] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.816004] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.823211] x86: Booting SMP configuration:
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.824956] .... node  #0, CPUs:      #1
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.827188] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.833219]  #2
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.834181] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.839870]  #3
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.840922] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.846409] x86: Booted up 1 node, 4 CPUs
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.848157] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.853108] devtmpfs: initialized
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.859933] evm: security.selinux
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.861935] evm: security.SMACK64
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.863393] evm: security.SMACK64EXEC
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.865251] evm: security.SMACK64TRANSMUTE
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.867448] evm: security.SMACK64MMAP
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.870245] evm: security.ima
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.871393] evm: security.capability
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.874222] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.879060] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.883478] pinctrl core: initialized pinctrl subsystem
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.887245] RTC time:  9:44:35, date: 08/13/18
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.891458] NET: Registered protocol family 16
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.903841] cpuidle: using governor ladder
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.915863] cpuidle: using governor menu
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.917483] PCCT header not found.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.918995] ACPI: bus type PCI registered
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.921208] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.925273] PCI: Using configuration type 1 for base access
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.941190] ACPI: Added _OSI(Module Device)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.943443] ACPI: Added _OSI(Processor Device)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.946261] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.948225] ACPI: Added _OSI(Processor Aggregator Device)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.953477] ACPI: Executed 2 blocks of module-level executable AML code
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.980296] ACPI: Interpreter enabled
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.982121] ACPI: (supports S0 S3 S4 S5)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.984221] ACPI: Using IOAPIC for interrupt routing
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    0.986157] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.020754] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.024153] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.026912] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.030106] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.035476] PCI host bridge to bus 0000:00
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.037345] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.040541] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.043784] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.047436] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.050441] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.052840] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.053323] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.077440] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.102251] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.108439] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.118122] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.125575] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.148430] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.157689] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.165000] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.187831] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.194839] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.200676] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.206787] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.212643] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.237247] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.240675] vgaarb: loaded
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.242137] SCSI subsystem initialized
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.244210] libata version 3.00 loaded.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.244240] ACPI: bus type USB registered
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.245682] usbcore: registered new interface driver usbfs
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.247859] usbcore: registered new interface driver hub
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.250418] usbcore: registered new device driver usb
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.253096] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.256030] dmi: Firmware registration failed.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.258732] PCI: Using ACPI for IRQ routing
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.260535] PCI: pci_cache_line_size set to 64 bytes
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.260644] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.260646] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.260784] NetLabel: Initializing
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.262587] NetLabel:  domain hash size = 128
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.264469] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.267249] NetLabel:  unlabeled traffic allowed by default
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.271624] amd_nb: Cannot enumerate AMD northbridges
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.275094] clocksource: Switched to clocksource kvm-clock
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.284824] pnp: PnP ACPI init
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.287313] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.287383] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.287428] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.287478] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.287521] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.287562] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.287603] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.287786] pnp: PnP ACPI: found 7 devices
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.297309] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.301578] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.301582] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.301583] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.301585] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.301623] NET: Registered protocol family 2
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.303762] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.308248] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.312484] TCP: Hash tables configured (established 131072 bind 65536)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.316086] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.318653] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.322072] NET: Registered protocol family 1
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.324632] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.327893] PCI: CLS 0 bytes, default 64
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    1.328757] Unpacking initramfs...
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.555195] Freeing initrd memory: 21432K
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.557479] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.561357] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.566106] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.569634] hw unit of domain pp0-core 2^-0 Joules
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.571925] hw unit of domain package 2^-0 Joules
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.574795] hw unit of domain dram 2^-0 Joules
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.576724] Scanning for low memory corruption every 60 seconds
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.579663] audit: initializing netlink subsys (disabled)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.581921] audit: type=2000 audit(1534153478.013:1): initialized
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.585320] Initialise system trusted keyring
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.587839] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.590656] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.596049] zbud: loaded
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.597502] VFS: Disk quotas dquot_6.6.0
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.599307] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.602442] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.605459] fuse init (API version 7.23)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.607767] Key type big_key registered
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.609347] Allocating IMA MOK and blacklist keyrings.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.615888] Key type asymmetric registered
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.618528] Asymmetric key parser 'x509' registered
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.621016] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.624981] io scheduler noop registered
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.627566] io scheduler deadline registered (default)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.630411] io scheduler cfq registered
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.632895] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.635894] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.639686] intel_idle: does not run on family 6 model 45
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.639795] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.643051] ACPI: Power Button [PWRF]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.644668] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.650335] ACPI: Sleep Button [SLPF]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.652457] GHES: HEST is not enabled!
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.658199] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.661080] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.671672] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.674293] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.684456] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.709235] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.735732] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.761653] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.787756] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.794244] Linux agpgart interface v0.103
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.799952] loop: module loaded
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.802285] libphy: Fixed MDIO Bus: probed
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.803845] tun: Universal TUN/TAP device driver, 1.6
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.806822] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.862695] PPP generic driver version 2.4.2
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.865236] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.868515] ehci-pci: EHCI PCI platform driver
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.871032] ehci-platform: EHCI generic platform driver
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.873764] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.877038] ohci-pci: OHCI PCI platform driver
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.879325] ohci-platform: OHCI generic platform driver
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.882047] uhci_hcd: USB Universal Host Controller Interface driver
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.884631] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.889474] i8042: Warning: Keylock active
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.893219] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.895332] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.898552] mousedev: PS/2 mouse device common for all mice
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.902051] rtc_cmos 00:00: RTC can wake from S4
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.905300] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.909028] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.912444] i2c /dev entries driver
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.914694] device-mapper: uevent: version 1.0.3
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.917982] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.922510] ledtrig-cpu: registered to indicate activity on CPUs
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.927477] NET: Registered protocol family 10
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.930258] NET: Registered protocol family 17
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.932732] Key type dns_resolver registered
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.935392] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.937755] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.940776] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.944189] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.947121] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.952559] registered taskstats version 1
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.954499] Loading compiled-in X.509 certificates
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.957893] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.962588] zswap: loaded using pool lzo/zbud
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.967231] Key type trusted registered
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.974483] Key type encrypted registered
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.976922] ima: No TPM chip found, activating TPM-bypass!
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.979872] evm: HMAC attrs: 0x1
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.981975]   Magic number: 14:625:727
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.984328] block loop7: hash matches
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.986348] acpi LNXCPU:2d: hash matches
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.988952] rtc_cmos 00:00: setting system clock to 2018-08-13 09:44:38 UTC (1534153478)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.992901] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.995773] EDD information not available.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    3.998512] PM: Hibernation image not present or could not be loaded.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.000103] Freeing unused kernel memory: 1496K
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.002070] Write protecting the kernel read-only data: 14336k
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.005777] Freeing unused kernel memory: 1956K
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.007842] Freeing unused kernel memory: 92K
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.025943] systemd-udevd[118]: starting version 204
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.100648] scsi host0: Virtio SCSI HBA
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.105959] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.116589] AVX version of gcm_enc/dec engaged.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.116875] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.122690] AES CTR mode by8 optimization enabled
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.162070] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.165196] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.168469] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.172042] sd 0:0:1:0: [sda] Write Protect is off
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.173915] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.174157] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.181402]  sda: sda1
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.184279] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.575265] tsc: Refined TSC clocksource calibration: 2599.761 MHz
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.578604] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x25795a3d23f, max_idle_ns: 440795283713 ns
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    4.944440] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    7.071539] floppy0: no floppy controllers found
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.235111] raid6: sse2x1   gen()  9302 MB/s
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.303105] raid6: sse2x1   xor()  6788 MB/s
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.371111] raid6: sse2x2   gen() 11256 MB/s
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.439112] raid6: sse2x2   xor()  7529 MB/s
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.507117] raid6: sse2x4   gen() 12483 MB/s
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.575112] raid6: sse2x4   xor()  8639 MB/s
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.576638] raid6: using algorithm sse2x4 gen() 12483 MB/s
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.578630] raid6: .... xor() 8639 MB/s, rmw enabled
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.579604] raid6: using ssse3x2 recovery algorithm
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.582363] xor: automatically using best checksumming function:
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.623109]    avx       : 22126.000 MB/sec
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.638141] Btrfs loaded
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.681538] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.683138] EXT4-fs (sda1): write access will be enabled during recovery
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.754138] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.761281] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.762161] EXT4-fs (sda1): recovery complete
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.767119] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    8.984527] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    9.095387] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    9.139277] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    9.344107] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [    9.919932] random: cloud-init: uninitialized urandom read (32 bytes read, 47 bits of entropy available)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   10.053369] systemd-udevd[701]: starting version 204
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   10.165306] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   10.219873] intel_rapl: no valid rapl domains found in package 0
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   10.268050] intel_rapl: no valid rapl domains found in package 0
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   10.270891] ppdev: user-space parallel port driver
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   10.372025] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   10.426146] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   10.487671] random: cloud-init: uninitialized urandom read (32 bytes read, 60 bits of entropy available)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   10.650397] random: cloud-init: uninitialized urandom read (32 bytes read, 60 bits of entropy available)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   10.901254] random: mktemp: uninitialized urandom read (12 bytes read, 63 bits of entropy available)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   10.976751] random: mktemp: uninitialized urandom read (6 bytes read, 63 bits of entropy available)
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   11.057303] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   11.095666] EXT4-fs (sda1): resized filesystem to 7864064
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   11.458958] init: failsafe main process (1092) killed by TERM signal
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO Running set_multiqueue.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO Set channels for eth0 to 4.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 09:44:46 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Starting Google Accounts daemon.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-clock-skew: INFO Synced system time with hardware clock.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Creating a new user account for me.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   12.148652] random: nonblocking pool is initialized
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Created user account me.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Creating a new user account for henrik.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Created user account henrik.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Creating a new user account for emma.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Created user account emma.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Creating a new user account for igor.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Created user account igor.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Created user account konstantinhaase.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Creating a new user account for aj.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Created user account aj.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Creating a new user account for solarce.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 cron[1451]: (CRON) INFO (pidfile fd = 3)
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Created user account solarce.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 cron[1490]: (CRON) STARTUP (fork ok)
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 cron[1490]: (CRON) INFO (Running @reboot jobs)
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Creating a new user account for asari.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 acpid: starting up with netlink and the input layer
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 acpid: 1 rule loaded
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 acpid: waiting for events: event logging is off
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Created user account asari.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Creating a new user account for bogdana.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 haveged: haveged starting up
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Created user account bogdana.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   12.868947] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Creating a new user account for konstantin.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   12.883296] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   12.922029] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   12.927137] Bridge firewalling registered
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Created user account konstantin.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   12.940252] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Creating a new user account for carmen.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Created user account carmen.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Creating a new user account for maria.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   13.044766] Initializing XFRM netlink socket
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   13.054196] Netfilter messages via NETLINK v0.30.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   13.057840] ctnetlink v0.93: registering with nfnetlink.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Created user account maria.
Aug 13 09:44:47 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 google-accounts: INFO Removing user packer.
Aug 13 09:44:48 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   13.255387] floppy0: no floppy controllers found
Aug 13 09:45:10 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 ntpdate[1861]: adjust time server 169.254.169.254 offset 0.005451 sec
Aug 13 09:45:17 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 ntpd[1896]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 13 09:45:17 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 ntpd[1897]: proto: precision = 0.102 usec
Aug 13 09:45:17 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 ntpd[1897]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 13 09:45:17 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 ntpd[1897]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 13 09:45:17 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 ntpd[1897]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 09:45:17 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 ntpd[1897]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 13 09:45:17 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 ntpd[1897]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 13 09:45:17 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 ntpd[1897]: Listen normally on 3 eth0 10.20.255.4 UDP 123
Aug 13 09:45:17 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 ntpd[1897]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 13 09:45:17 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 ntpd[1897]: peers refreshed
Aug 13 09:45:17 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 ntpd[1897]: Listening on routing socket on fd #21 for interface updates
Aug 13 09:45:17 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 kernel: [   43.088104] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 09:45:18 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 startup-script: INFO Found startup-script in metadata.
Aug 13 09:45:18 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 13 09:45:18 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 startup-script: INFO startup-script: job 1 at Mon Aug 13 12:55:00 2018
Aug 13 09:45:18 travis-job-762c7a94-2056-42ad-9a39-ba422d568555 startup-script: INFO startup-script: Return code 0.
