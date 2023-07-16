plain
[00:29:53] [RUSTC-TIMING] proc_macro test:false 13.193
[00:29:53]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:30:22] [RUSTC-TIMING] syntax_ext test:false 28.971

Broadcast message from root@travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd
 (unknown) at 22:21 ...
The system is going down for power off NOW!
[00:30:52] 
[00:30:52] Session terminated, terminating shell... ...terminated.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:08de1bdb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:00bbe1be
$ sudo tail -n 500 /var/log/syslog
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] Policy zone: Normal
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] Hierarchical RCU implementation.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] console [ttyS0] enabled
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.368462] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.370093] pid_max: default: 32768 minimum: 301
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.371805] ACPI: Core revision 20150930
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.379306] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.382454] Security Framework initialized
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.384302] Yama: becoming mindful.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.385991] AppArmor: AppArmor disabled by boot time parameter
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.389908] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.406794] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.412367] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.415924] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.418731] Initializing cgroup subsys io
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.420542] Initializing cgroup subsys memory
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.421802] Initializing cgroup subsys devices
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.423654] Initializing cgroup subsys freezer
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.425207] Initializing cgroup subsys net_cls
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.427274] Initializing cgroup subsys perf_event
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.429800] Initializing cgroup subsys net_prio
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.432369] Initializing cgroup subsys hugetlb
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.434568] Initializing cgroup subsys pids
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.436307] CPU: Physical Processor ID: 0
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.438094] CPU: Processor Core ID: 0
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.439991] mce: CPU supports 32 MCE banks
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.442093] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.444330] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.449201] Freeing SMP alternatives memory: 32K
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.460984] ftrace: allocating 32185 entries in 126 pages
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.516189] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.520030] smpboot: Max logical packages: 2
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.522932] x2apic enabled
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.525579] Switched APIC routing to physical x2apic.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.533103] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.643553] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.648906] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.661171] x86: Booting SMP configuration:
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.665507] .... node  #0, CPUs:      #1
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.676565] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.700154]  #2
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.702164] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.714773]  #3
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.716787] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.724795] x86: Booted up 1 node, 4 CPUs
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.728715] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.737351] devtmpfs: initialized
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.744195] evm: security.selinux
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.746483] evm: security.SMACK64
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.748670] evm: security.SMACK64EXEC
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.751019] evm: security.SMACK64TRANSMUTE
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.754162] evm: security.SMACK64MMAP
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.757262] evm: security.ima
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.760639] evm: security.capability
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.765789] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.777629] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.784381] pinctrl core: initialized pinctrl subsystem
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.789370] RTC time: 21:49:59, date: 08/14/18
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.793588] NET: Registered protocol family 16
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.807720] cpuidle: using governor ladder
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.820155] cpuidle: using governor menu
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.823598] PCCT header not found.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.827162] ACPI: bus type PCI registered
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.831785] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.839499] PCI: Using configuration type 1 for base access
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.857782] ACPI: Added _OSI(Module Device)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.861481] ACPI: Added _OSI(Processor Device)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.865838] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.869158] ACPI: Added _OSI(Processor Aggregator Device)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.878138] ACPI: Executed 2 blocks of module-level executable AML code
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.907680] ACPI: Interpreter enabled
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.911854] ACPI: (supports S0 S3 S4 S5)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.915135] ACPI: Using IOAPIC for interrupt routing
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.919763] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.963269] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.970874] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.978927] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    0.988924] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.004224] PCI host bridge to bus 0000:00
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.007743] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.013341] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.018844] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.031241] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.042603] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.047194] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.047637] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.095523] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.150417] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.159695] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.176566] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.188220] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.217579] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.230508] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.243608] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.279059] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.289625] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.297781] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.305210] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.312794] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.337156] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.342045] vgaarb: loaded
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.344025] SCSI subsystem initialized
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.346260] libata version 3.00 loaded.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.346302] ACPI: bus type USB registered
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.350031] usbcore: registered new interface driver usbfs
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.354277] usbcore: registered new interface driver hub
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.357591] usbcore: registered new device driver usb
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.360704] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.364553] dmi: Firmware registration failed.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.368214] PCI: Using ACPI for IRQ routing
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.371414] PCI: pci_cache_line_size set to 64 bytes
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.371634] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.371637] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.371791] NetLabel: Initializing
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.375624] NetLabel:  domain hash size = 128
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.380078] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.384667] NetLabel:  unlabeled traffic allowed by default
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.392079] amd_nb: Cannot enumerate AMD northbridges
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.400756] clocksource: Switched to clocksource kvm-clock
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.412590] pnp: PnP ACPI init
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.416999] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.417089] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.417137] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.417189] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.417231] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.417276] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.417335] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.417515] pnp: PnP ACPI: found 7 devices
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.430966] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.438362] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.438366] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.438368] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.438369] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.438411] NET: Registered protocol family 2
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.441523] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.446819] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.453536] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.460802] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.466185] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.475332] NET: Registered protocol family 1
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.480276] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.486853] PCI: CLS 0 bytes, default 64
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    1.486966] Unpacking initramfs...
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.599801] Freeing initrd memory: 21432K
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.603102] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.607510] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.617198] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.624609] hw unit of domain pp0-core 2^-0 Joules
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.628495] hw unit of domain package 2^-0 Joules
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.632721] hw unit of domain dram 2^-0 Joules
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.635654] Scanning for low memory corruption every 60 seconds
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.641216] audit: initializing netlink subsys (disabled)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.645627] audit: type=2000 audit(1534283401.929:1): initialized
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.650230] Initialise system trusted keyring
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.654294] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.660148] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.667521] zbud: loaded
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.670421] VFS: Disk quotas dquot_6.6.0
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.673707] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.679071] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.683731] fuse init (API version 7.23)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.688456] Key type big_key registered
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.692520] Allocating IMA MOK and blacklist keyrings.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.703668] Key type asymmetric registered
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.708017] Asymmetric key parser 'x509' registered
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.711396] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.716715] io scheduler noop registered
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.719863] io scheduler deadline registered (default)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.723232] io scheduler cfq registered
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.725990] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.730465] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.734549] intel_idle: does not run on family 6 model 45
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.734650] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.740079] ACPI: Power Button [PWRF]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.742829] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.747770] ACPI: Sleep Button [SLPF]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.751074] GHES: HEST is not enabled!
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.758866] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.762562] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.776184] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.781676] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.795312] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.822515] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.850776] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.878340] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.908202] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.916388] Linux agpgart interface v0.103
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.924135] loop: module loaded
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.926743] libphy: Fixed MDIO Bus: probed
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.933259] tun: Universal TUN/TAP device driver, 1.6
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    3.937558] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.004629] PPP generic driver version 2.4.2
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.008264] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.013469] ehci-pci: EHCI PCI platform driver
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.016404] ehci-platform: EHCI generic platform driver
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.020035] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.026102] ohci-pci: OHCI PCI platform driver
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.030837] ohci-platform: OHCI generic platform driver
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.034978] uhci_hcd: USB Universal Host Controller Interface driver
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.040395] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.047016] i8042: Warning: Keylock active
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.050967] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.054608] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.059861] mousedev: PS/2 mouse device common for all mice
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.064653] rtc_cmos 00:00: RTC can wake from S4
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.068448] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.073111] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.077441] i2c /dev entries driver
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.080417] device-mapper: uevent: version 1.0.3
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.084291] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.089926] ledtrig-cpu: registered to indicate activity on CPUs
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.097104] NET: Registered protocol family 10
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.101507] NET: Registered protocol family 17
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.104280] Key type dns_resolver registered
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.108332] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.113147] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.118620] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.124921] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.131163] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.139787] registered taskstats version 1
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.146285] Loading compiled-in X.509 certificates
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.152036] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.164956] zswap: loaded using pool lzo/zbud
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.174780] Key type trusted registered
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.185219] Key type encrypted registered
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.189279] ima: No TPM chip found, activating TPM-bypass!
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.195952] evm: HMAC attrs: 0x1
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.200325]   Magic number: 14:81:854
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.204158] rtc_cmos 00:00: setting system clock to 2018-08-14 21:50:02 UTC (1534283402)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.211529] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.215654] EDD information not available.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.218839] PM: Hibernation image not present or could not be loaded.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.220444] Freeing unused kernel memory: 1496K
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.224069] Write protecting the kernel read-only data: 14336k
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.229652] Freeing unused kernel memory: 1956K
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.232992] Freeing unused kernel memory: 92K
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.256358] systemd-udevd[120]: starting version 204
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.263209] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.333194] scsi host0: Virtio SCSI HBA
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.343254] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.352054] AVX version of gcm_enc/dec engaged.
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.354640] AES CTR mode by8 optimization enabled
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.465856] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.465993] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.465995] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.466554] sd 0:0:1:0: [sda] Write Protect is off
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.466557] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.466633] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.470143]  sda: sda1
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.472834] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.632972] tsc: Refined TSC clocksource calibration: 2600.005 MHz
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    4.637482] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a40ff313, max_idle_ns: 440795330741 ns
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    5.173412] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    7.305241] floppy0: no floppy controllers found
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    8.500848] raid6: sse2x1   gen()  7895 MB/s
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    8.588816] raid6: sse2x1   xor()  5763 MB/s
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    8.660825] raid6: sse2x2   gen() 10392 MB/s
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    8.728983] raid6: sse2x2   xor()  6868 MB/s
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    8.800875] raid6: sse2x4   gen() 11989 MB/s
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    8.873058] raid6: sse2x4   xor()  8295 MB/s
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    8.884202] raid6: using algorithm sse2x4 gen() 11989 MB/s
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    8.893437] raid6: .... xor() 8295 MB/s, rmw enabled
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    8.897246] raid6: using ssse3x2 recovery algorithm
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    8.904331] xor: automatically using best checksumming function:
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    8.948822]    avx       : 26168.000 MB/sec
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    8.969994] Btrfs loaded
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    9.093274] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    9.104079] EXT4-fs (sda1): write access will be enabled during recovery
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    9.283466] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    9.302609] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    9.309490] EXT4-fs (sda1): recovery complete
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    9.324538] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    9.688077] random: init: uninitialized urandom read (12 bytes read, 22 bits of entropy available)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    9.879771] random: mountall: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [    9.956682] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   10.307133] random: cloud-init: uninitialized urandom read (32 bytes read, 31 bits of entropy available)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   11.124673] random: cloud-init: uninitialized urandom read (32 bytes read, 39 bits of entropy available)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   11.295359] systemd-udevd[702]: starting version 204
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   11.469218] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   11.530564] intel_rapl: no valid rapl domains found in package 0
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   11.593531] intel_rapl: no valid rapl domains found in package 0
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   11.598299] ppdev: user-space parallel port driver
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   11.770682] random: mktemp: uninitialized urandom read (6 bytes read, 50 bits of entropy available)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   11.847279] random: mktemp: uninitialized urandom read (6 bytes read, 50 bits of entropy available)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   11.929017] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   12.121510] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   12.536459] random: mktemp: uninitialized urandom read (12 bytes read, 54 bits of entropy available)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   12.633840] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   12.738326] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   12.810570] EXT4-fs (sda1): resized filesystem to 7864064
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   13.379858] init: failsafe main process (1096) killed by TERM signal
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 14 21:50:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO Running set_multiqueue.
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO Set channels for eth0 to 4.
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Starting Google Accounts daemon.
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Creating a new user account for me.
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Created user account me.
Aug 14 21:50:12 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Creating a new user account for henrik.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-clock-skew: INFO Synced system time with hardware clock.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Created user account henrik.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Creating a new user account for emma.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Created user account emma.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Creating a new user account for igor.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   14.564853] floppy0: no floppy controllers found
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Created user account igor.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Created user account konstantinhaase.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Creating a new user account for aj.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Created user account aj.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Creating a new user account for solarce.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Created user account solarce.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Creating a new user account for asari.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Created user account asari.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   15.054272] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   15.058958] Bridge firewalling registered
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Creating a new user account for bogdana.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   15.076719] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Created user account bogdana.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   15.139247] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Creating a new user account for konstantin.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   15.209960] Initializing XFRM netlink socket
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   15.220607] Netfilter messages via NETLINK v0.30.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Created user account konstantin.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   15.225730] ctnetlink v0.93: registering with nfnetlink.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Creating a new user account for carmen.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Created user account carmen.
Aug 14 21:50:13 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Creating a new user account for maria.
Aug 14 21:50:14 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Created user account maria.
Aug 14 21:50:14 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd google-accounts: INFO Removing user packer.
Aug 14 21:50:14 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   15.788196] random: nonblocking pool is initialized
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd cron[1714]: (CRON) INFO (pidfile fd = 3)
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd cron[1752]: (CRON) STARTUP (fork ok)
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd cron[1752]: (CRON) INFO (Running @reboot jobs)
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd acpid: starting up with netlink and the input layer
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd acpid: 1 rule loaded
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd acpid: waiting for events: event logging is off
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd haveged: haveged starting up
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpdate[975]: adjust time server 169.254.169.254 offset 0.018863 sec
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   20.193581] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpd[1850]: proto: precision = 0.111 usec
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpd[1850]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpd[1850]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpd[1850]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpd[1850]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpd[1850]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpd[1850]: Listen normally on 3 eth0 10.20.2.121 UDP 123
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpd[1850]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpd[1850]: peers refreshed
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpd[1850]: Listening on routing socket on fd #21 for interface updates
Aug 14 21:50:18 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   20.384613] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 21:50:19 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd startup-script: INFO Found startup-script in metadata.
Aug 14 21:50:19 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 14 21:50:19 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd startup-script: INFO startup-script: job 1 at Wed Aug 15 01:00:00 2018
Aug 14 21:50:19 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd startup-script: INFO startup-script: Return code 0.
Aug 14 21:50:19 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd startup-script: INFO startup-script: Return code 0.
Aug 14 21:50:19 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd startup-script: INFO Finished running startup scripts.
Aug 14 21:50:19 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ec2: 
Aug 14 21:50:19 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ec2: #############################################################
Aug 14 21:50:19 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 14 21:50:19 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ec2: 1024 64:a5:0a:a2:15:13:7e:05:1b:6a:af:03:75:d6:fc:0a  root@travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd (DSA)
Aug 14 21:50:19 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ec2: 256 ca:9f:73:36:37:5b:e4:59:b6:99:c0:fa:b1:29:b0:b3  root@travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd (ECDSA)
Aug 14 21:50:19 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ec2: 256 43:34:67:da:92:25:48:9f:cc:c2:ad:5f:a4:8e:14:29  root@travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd (ED25519)
Aug 14 21:50:19 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ec2: 2048 66:31:05:27:8f:3d:3c:57:0d:cc:18:07:8a:20:bb:2d  root@travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd (RSA)
Aug 14 21:50:19 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 14 21:50:19 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ec2: #############################################################
Aug 14 21:50:28 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpdate[2252]: the NTP socket is in use, exiting
Aug 14 21:51:01 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [   62.706196] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 14 21:52:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [  133.043396] device veth9749958 entered promiscuous mode
Aug 14 21:52:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [  133.043488] docker0: port 1(veth9749958) entered forwarding state
Aug 14 21:52:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [  133.043499] docker0: port 1(veth9749958) entered forwarding state
Aug 14 21:52:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [  133.044012] docker0: port 1(veth9749958) entered disabled state
Aug 14 21:52:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [  133.160575] cgroup: docker-runc (4824) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 21:52:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [  133.160580] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 21:52:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [  133.239831] eth0: renamed from veth6e84e44
Aug 14 21:52:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [  133.289806] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 21:52:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [  133.291193] docker0: port 1(veth9749958) entered forwarding state
Aug 14 21:52:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [  133.291239] docker0: port 1(veth9749958) entered forwarding state
Aug 14 21:52:11 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [  133.291279] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 14 21:52:14 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpd[1850]: Listen normally on 5 docker0 fe80::42:3bff:fe8b:a290 UDP 123
Aug 14 21:52:14 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpd[1850]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 14 21:52:14 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpd[1850]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 14 21:52:14 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpd[1850]: peers refreshed
Aug 14 21:52:14 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd ntpd[1850]: new interface(s) found: waking up resolver
Aug 14 21:52:26 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [  148.326732] docker0: port 1(veth9749958) entered forwarding state
Aug 14 22:17:01 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd CRON[4171]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd dbus[1164]: [system] Activating service name='org.freedesktop.systemd1' (using servicehelper)
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd dbus[1164]: [system] Successfully activated service 'org.freedesktop.systemd1'
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.146868] init: tty4 main process (1709) killed by TERM signal
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.147064] init: tty5 main process (1712) killed by TERM signal
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.147359] init: tty2 main process (1716) killed by TERM signal
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.147550] init: tty3 main process (1717) killed by TERM signal
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.147736] init: tty6 main process (1719) killed by TERM signal
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.148142] init: cron main process (1752) killed by TERM signal
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.148478] init: irqbalance main process (1766) killed by TERM signal
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.148729] init: tty1 main process (1889) killed by TERM signal
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.148931] init: ttyS0 main process (1898) killed by TERM signal
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.149367] init: plymouth-upstart-bridge main process (6397) terminated with status 1
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.149375] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.194188] init: plymouth-upstart-bridge main process (6433) terminated with status 1
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.194199] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.206703] init: plymouth-upstart-bridge main process (6450) terminated with status 1
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.206714] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.218519] init: plymouth-upstart-bridge main process (6458) terminated with status 1
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.218530] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.219673] docker0: port 1(veth9749958) entered disabled state
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.219783] veth6e84e44: renamed from eth0
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.224867] init: plymouth-upstart-bridge main process (6460) terminated with status 1
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.224878] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.233094] init: plymouth-upstart-bridge main process (6464) terminated with status 1
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.233106] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.237068] init: plymouth-upstart-bridge main process (6467) terminated with status 1
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.237075] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.238409] init: apport post-stop process (6390) terminated with status 1
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.245033] init: plymouth-upstart-bridge main process (6473) terminated with status 1
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.245042] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.251587] init: plymouth-upstart-bridge main process (6476) terminated with status 1
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.251598] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.256924] init: plymouth-upstart-bridge main process (6478) terminated with status 1
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.256933] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.261876] init: plymouth-upstart-bridge main process (6480) terminated with status 1
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.261884] init: plymouth-upstart-bridge respawning too fast, stopped
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd shutdown-script: INFO Starting shutdown scripts.
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.297277] docker0: port 1(veth9749958) entered disabled state
Aug 14 22:21:54 travis-job-6a8d2dc3-913c-42fb-a763-03db3c58eafd kernel: [ 1916.299238] device veth9749958 left promiscuous mode
---
travis_time:end:034cbb1d:start=1534285315516950988,finish=1534285315523040745,duration=6089757
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01663d42
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
