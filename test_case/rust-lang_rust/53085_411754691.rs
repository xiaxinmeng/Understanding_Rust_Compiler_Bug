plain
[00:26:42] [RUSTC-TIMING] syntax_pos test:false 4.449
[00:26:42]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:26:50] [RUSTC-TIMING] rustc_errors test:false 8.561

Broadcast message from root@travis-job-7a9630de-32a6-49d4-8aff-75a66229c917
 (unknown) at 13:18 ...
The system is going down for power off NOW!
[00:27:14] 
[00:27:14] Session terminated, terminating shell... ...terminated.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:094b46aa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:00a1517d
$ sudo tail -n 500 /var/log/syslog
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] Policy zone: Normal
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] console [ttyS0] enabled
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.353054] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.354718] pid_max: default: 32768 minimum: 301
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.355545] ACPI: Core revision 20150930
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.362061] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.363170] Security Framework initialized
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.363911] Yama: becoming mindful.
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.364915] AppArmor: AppArmor disabled by boot time parameter
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.367664] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.378082] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.382750] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.383808] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.385418] Initializing cgroup subsys io
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.386477] Initializing cgroup subsys memory
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.387284] Initializing cgroup subsys devices
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.387938] Initializing cgroup subsys freezer
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.388746] Initializing cgroup subsys net_cls
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.389815] Initializing cgroup subsys perf_event
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.390575] Initializing cgroup subsys net_prio
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.391264] Initializing cgroup subsys hugetlb
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.391938] Initializing cgroup subsys pids
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.392740] CPU: Physical Processor ID: 0
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.393364] CPU: Processor Core ID: 0
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.393906] mce: CPU supports 32 MCE banks
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.394842] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.395861] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.398682] Freeing SMP alternatives memory: 32K
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.407444] ftrace: allocating 32185 entries in 126 pages
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.455503] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.456807] smpboot: Max logical packages: 2
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.459002] x2apic enabled
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.461221] Switched APIC routing to physical x2apic.
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.464912] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.572157] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.573963] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.576988] x86: Booting SMP configuration:
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.577590] .... node  #0, CPUs:      #1
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.578573] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.581942]  #2
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.582577] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.586565]  #3
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.587139] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.590267] x86: Booted up 1 node, 4 CPUs
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.590866] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.593098] devtmpfs: initialized
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.597601] evm: security.selinux
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.598134] evm: security.SMACK64
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.598719] evm: security.SMACK64EXEC
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.599339] evm: security.SMACK64TRANSMUTE
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.599966] evm: security.SMACK64MMAP
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.600778] evm: security.ima
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.601690] evm: security.capability
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.602661] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.604327] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.605839] pinctrl core: initialized pinctrl subsystem
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.606951] RTC time: 12:50:19, date: 08/09/18
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.608596] NET: Registered protocol family 16
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.620219] cpuidle: using governor ladder
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.632226] cpuidle: using governor menu
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.633013] PCCT header not found.
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.633608] ACPI: bus type PCI registered
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.634313] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.635928] PCI: Using configuration type 1 for base access
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.649361] ACPI: Added _OSI(Module Device)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.650427] ACPI: Added _OSI(Processor Device)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.651211] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.652431] ACPI: Added _OSI(Processor Aggregator Device)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.656458] ACPI: Executed 2 blocks of module-level executable AML code
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.680274] ACPI: Interpreter enabled
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.680862] ACPI: (supports S0 S3 S4 S5)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.681804] ACPI: Using IOAPIC for interrupt routing
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.683395] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.713464] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.714620] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.716107] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.717358] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.719950] PCI host bridge to bus 0000:00
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.720707] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.722247] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.723328] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.724619] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.725838] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.726731] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.727162] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.742403] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.758334] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.759920] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.765474] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.770899] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.784250] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.790390] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.794852] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.808881] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.811693] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.814419] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.817273] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.819993] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.840463] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.841605] vgaarb: loaded
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.842419] SCSI subsystem initialized
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.843456] libata version 3.00 loaded.
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.843482] ACPI: bus type USB registered
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.844118] usbcore: registered new interface driver usbfs
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.845242] usbcore: registered new interface driver hub
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.846328] usbcore: registered new device driver usb
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.847446] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.848742] dmi: Firmware registration failed.
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.849575] PCI: Using ACPI for IRQ routing
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.850177] PCI: pci_cache_line_size set to 64 bytes
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.850272] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.850275] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.850405] NetLabel: Initializing
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.850937] NetLabel:  domain hash size = 128
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.851535] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.852456] NetLabel:  unlabeled traffic allowed by default
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.853647] amd_nb: Cannot enumerate AMD northbridges
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.854525] clocksource: Switched to clocksource kvm-clock
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.862266] pnp: PnP ACPI init
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.863037] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.863116] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.863164] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.863218] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.863264] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.863307] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.863350] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.863526] pnp: PnP ACPI: found 7 devices
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.871041] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.872975] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.872977] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.872979] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.872981] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.873023] NET: Registered protocol family 2
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.874164] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.876477] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.877750] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.878814] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.879916] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.882143] NET: Registered protocol family 1
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.882974] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.884139] PCI: CLS 0 bytes, default 64
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    0.884210] Unpacking initramfs...
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.920940] Freeing initrd memory: 21432K
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.921838] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.923093] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.925278] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.926934] hw unit of domain pp0-core 2^-0 Joules
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.927826] hw unit of domain package 2^-0 Joules
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.928700] hw unit of domain dram 2^-0 Joules
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.929873] Scanning for low memory corruption every 60 seconds
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.931804] audit: initializing netlink subsys (disabled)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.933458] audit: type=2000 audit(1533819021.763:1): initialized
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.935030] Initialise system trusted keyring
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.936495] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.937880] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.940521] zbud: loaded
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.941343] VFS: Disk quotas dquot_6.6.0
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.942016] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.943293] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.944761] fuse init (API version 7.23)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.945600] Key type big_key registered
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.946594] Allocating IMA MOK and blacklist keyrings.
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.949563] Key type asymmetric registered
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.950454] Asymmetric key parser 'x509' registered
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.951777] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.953248] io scheduler noop registered
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.954048] io scheduler deadline registered (default)
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.955520] io scheduler cfq registered
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.956484] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.957488] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.958902] intel_idle: does not run on family 6 model 45
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.959003] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.960587] ACPI: Power Button [PWRF]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.961493] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.962623] ACPI: Sleep Button [SLPF]
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.964260] GHES: HEST is not enabled!
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.966611] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.967790] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.972571] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.974185] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    2.979539] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.003870] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.027430] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.050642] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.073921] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.078268] Linux agpgart interface v0.103
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.082370] loop: module loaded
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.083851] libphy: Fixed MDIO Bus: probed
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.085208] tun: Universal TUN/TAP device driver, 1.6
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.086917] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.128567] PPP generic driver version 2.4.2
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.130647] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.132471] ehci-pci: EHCI PCI platform driver
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.134485] ehci-platform: EHCI generic platform driver
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.136007] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.138312] ohci-pci: OHCI PCI platform driver
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.139792] ohci-platform: OHCI generic platform driver
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.141577] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.143225] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.146552] i8042: Warning: Keylock active
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.148977] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.150043] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.151659] mousedev: PS/2 mouse device common for all mice
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.153791] rtc_cmos 00:00: RTC can wake from S4
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.155337] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.157707] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.159714] i2c /dev entries driver
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.161135] device-mapper: uevent: version 1.0.3
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.162536] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.165553] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.167455] NET: Registered protocol family 10
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.169447] NET: Registered protocol family 17
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.170799] Key type dns_resolver registered
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.172031] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.172913] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.174217] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.176453] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.178862] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.180870] registered taskstats version 1
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.181812] Loading compiled-in X.509 certificates
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [    3.184065] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 12:50:30 travis-job-7a9630de-32a6-49d4-8aff-75a66229c917 kernel: [   
