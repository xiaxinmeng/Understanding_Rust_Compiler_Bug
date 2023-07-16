plain
[00:27:21] [RUSTC-TIMING] syntax_pos test:false 4.393
[00:27:21]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:27:29] [RUSTC-TIMING] rustc_errors test:false 7.867

Broadcast message from root@travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106
 (unknown) at 17:56 ...
The system is going down for power off NOW!
[00:27:55] 
[00:27:55] Session terminated, terminating shell... ...terminated.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:01a11a70
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:024976cf
$ sudo tail -n 500 /var/log/syslog
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] Policy zone: Normal
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] console [ttyS0] enabled
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.622255] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.628204] pid_max: default: 32768 minimum: 301
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.631535] ACPI: Core revision 20150930
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.640299] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.644492] Security Framework initialized
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.646600] Yama: becoming mindful.
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.649154] AppArmor: AppArmor disabled by boot time parameter
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.654007] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.666576] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.674588] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.678905] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.684993] Initializing cgroup subsys io
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.687246] Initializing cgroup subsys memory
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.690176] Initializing cgroup subsys devices
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.693334] Initializing cgroup subsys freezer
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.696993] Initializing cgroup subsys net_cls
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.700165] Initializing cgroup subsys perf_event
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.703272] Initializing cgroup subsys net_prio
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.706019] Initializing cgroup subsys hugetlb
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.708907] Initializing cgroup subsys pids
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.712080] CPU: Physical Processor ID: 0
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.714552] CPU: Processor Core ID: 0
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.718354] mce: CPU supports 32 MCE banks
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.720859] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.725143] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.731873] Freeing SMP alternatives memory: 32K
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.742876] ftrace: allocating 32185 entries in 126 pages
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.794004] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.798503] smpboot: Max logical packages: 2
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.802012] x2apic enabled
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.805332] Switched APIC routing to physical x2apic.
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.811135] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.919710] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.926137] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.932645] x86: Booting SMP configuration:
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.935689] .... node  #0, CPUs:      #1
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.938542] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.945511]  #2
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.946453] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.954248]  #3
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.955428] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.962992] x86: Booted up 1 node, 4 CPUs
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.964764] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.971043] devtmpfs: initialized
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.977064] evm: security.selinux
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.979526] evm: security.SMACK64
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.981607] evm: security.SMACK64EXEC
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.983861] evm: security.SMACK64TRANSMUTE
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.986217] evm: security.SMACK64MMAP
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.988695] evm: security.ima
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.990458] evm: security.capability
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    0.993218] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.000052] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.004697] pinctrl core: initialized pinctrl subsystem
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.008410] RTC time: 17:28:08, date: 08/08/18
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.012070] NET: Registered protocol family 16
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.023799] cpuidle: using governor ladder
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.035929] cpuidle: using governor menu
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.038366] PCCT header not found.
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.040461] ACPI: bus type PCI registered
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.043147] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.048187] PCI: Using configuration type 1 for base access
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.065841] ACPI: Added _OSI(Module Device)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.068750] ACPI: Added _OSI(Processor Device)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.071715] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.075307] ACPI: Added _OSI(Processor Aggregator Device)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.081026] ACPI: Executed 2 blocks of module-level executable AML code
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.107041] ACPI: Interpreter enabled
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.110088] ACPI: (supports S0 S3 S4 S5)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.112767] ACPI: Using IOAPIC for interrupt routing
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.115951] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.151091] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.155608] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.160921] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.165347] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.174064] PCI host bridge to bus 0000:00
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.177312] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.182076] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.187918] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.192862] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.197618] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.201640] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.202067] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.227500] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.253035] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.257730] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.266892] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.275985] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.296981] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.307063] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.314803] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.335589] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.341743] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.347884] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.353675] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.359014] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.381088] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.384295] vgaarb: loaded
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.386007] SCSI subsystem initialized
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.389381] libata version 3.00 loaded.
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.389405] ACPI: bus type USB registered
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.391963] usbcore: registered new interface driver usbfs
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.395190] usbcore: registered new interface driver hub
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.398132] usbcore: registered new device driver usb
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.402145] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.406675] dmi: Firmware registration failed.
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.410220] PCI: Using ACPI for IRQ routing
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.413753] PCI: pci_cache_line_size set to 64 bytes
Aug  8 17:28:19 travis-job-0dee625f-7e5f-4fe9-a258-a6fa415fd106 kernel: [    1.413851] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
