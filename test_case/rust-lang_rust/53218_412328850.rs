plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:078cda3c
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:1c45d100
$ sudo tail -n 500 /var/log/syslog
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] Policy zone: Normal
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] console [ttyS0] enabled
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.329954] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.331212] pid_max: default: 32768 minimum: 301
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.331864] ACPI: Core revision 20150930
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.338072] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.339370] Security Framework initialized
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.340153] Yama: becoming mindful.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.340696] AppArmor: AppArmor disabled by boot time parameter
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.343515] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.352509] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.356653] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.357674] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.359293] Initializing cgroup subsys io
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.359934] Initializing cgroup subsys memory
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.360810] Initializing cgroup subsys devices
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.361835] Initializing cgroup subsys freezer
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.362601] Initializing cgroup subsys net_cls
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.363212] Initializing cgroup subsys perf_event
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.363897] Initializing cgroup subsys net_prio
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.364530] Initializing cgroup subsys hugetlb
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.365655] Initializing cgroup subsys pids
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.366361] CPU: Physical Processor ID: 0
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.366994] CPU: Processor Core ID: 0
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.367613] mce: CPU supports 32 MCE banks
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.368314] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.369103] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.372311] Freeing SMP alternatives memory: 32K
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.382409] ftrace: allocating 32185 entries in 126 pages
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.435464] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.436513] smpboot: Max logical packages: 2
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.437651] x2apic enabled
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.439542] Switched APIC routing to physical x2apic.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.442958] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.549179] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.550597] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.552971] x86: Booting SMP configuration:
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.553635] .... node  #0, CPUs:      #1
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.554412] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.558461]  #2
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.559055] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.562843]  #3
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.563263] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.566824] x86: Booted up 1 node, 4 CPUs
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.567909] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.570378] devtmpfs: initialized
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.574965] evm: security.selinux
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.575667] evm: security.SMACK64
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.576231] evm: security.SMACK64EXEC
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.576781] evm: security.SMACK64TRANSMUTE
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.577363] evm: security.SMACK64MMAP
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.578267] evm: security.ima
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.578710] evm: security.capability
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.579849] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.581571] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.583187] pinctrl core: initialized pinctrl subsystem
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.584203] RTC time:  5:59:41, date: 08/12/18
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.585673] NET: Registered protocol family 16
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.597217] cpuidle: using governor ladder
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.609217] cpuidle: using governor menu
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.609925] PCCT header not found.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.610703] ACPI: bus type PCI registered
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.611491] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.612671] PCI: Using configuration type 1 for base access
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.626251] ACPI: Added _OSI(Module Device)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.627107] ACPI: Added _OSI(Processor Device)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.627754] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.628448] ACPI: Added _OSI(Processor Aggregator Device)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.631637] ACPI: Executed 2 blocks of module-level executable AML code
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.654676] ACPI: Interpreter enabled
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.655374] ACPI: (supports S0 S3 S4 S5)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.655959] ACPI: Using IOAPIC for interrupt routing
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.656851] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.686364] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.687517] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.688457] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.689359] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.691608] PCI host bridge to bus 0000:00
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.692211] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.693525] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.694625] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.695732] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.696857] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.697874] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.698288] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.713505] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.727778] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.729246] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.735220] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.739401] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.752546] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.757457] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.761488] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.773990] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.776450] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.778702] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.780906] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.783090] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.803097] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.804317] vgaarb: loaded
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.805208] SCSI subsystem initialized
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.806040] libata version 3.00 loaded.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.806064] ACPI: bus type USB registered
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.806749] usbcore: registered new interface driver usbfs
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.807599] usbcore: registered new interface driver hub
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.808538] usbcore: registered new device driver usb
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.809530] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.810572] dmi: Firmware registration failed.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.811337] PCI: Using ACPI for IRQ routing
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.812131] PCI: pci_cache_line_size set to 64 bytes
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.812227] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.812229] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.812355] NetLabel: Initializing
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.812878] NetLabel:  domain hash size = 128
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.813550] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.814255] NetLabel:  unlabeled traffic allowed by default
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.815176] amd_nb: Cannot enumerate AMD northbridges
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.816081] clocksource: Switched to clocksource kvm-clock
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.823306] pnp: PnP ACPI init
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.823890] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.823956] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.824000] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.824050] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.824106] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.824153] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.824194] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.824359] pnp: PnP ACPI: found 7 devices
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.832121] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.833786] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.833789] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.833791] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.833792] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.833845] NET: Registered protocol family 2
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.834839] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.836603] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.837670] TCP: Hash tables configured (established 131072 bind 65536)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.838742] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.839634] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.841307] NET: Registered protocol family 1
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.842204] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.843132] PCI: CLS 0 bytes, default 64
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    0.843182] Unpacking initramfs...
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.847874] Freeing initrd memory: 21432K
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.848849] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.850266] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.852505] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.854245] hw unit of domain pp0-core 2^-0 Joules
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.855152] hw unit of domain package 2^-0 Joules
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.856024] hw unit of domain dram 2^-0 Joules
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.857540] Scanning for low memory corruption every 60 seconds
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.859939] audit: initializing netlink subsys (disabled)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.861228] audit: type=2000 audit(1534053583.099:1): initialized
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.863413] Initialise system trusted keyring
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.864983] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.866002] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.868619] zbud: loaded
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.869561] VFS: Disk quotas dquot_6.6.0
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.871166] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.873907] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.876074] fuse init (API version 7.23)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.878117] Key type big_key registered
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.879789] Allocating IMA MOK and blacklist keyrings.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.883915] Key type asymmetric registered
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.885089] Asymmetric key parser 'x509' registered
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.886523] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.888591] io scheduler noop registered
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.889717] io scheduler deadline registered (default)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.891123] io scheduler cfq registered
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.892447] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.893975] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.895858] intel_idle: does not run on family 6 model 45
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.895952] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.898056] ACPI: Power Button [PWRF]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.899441] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.901821] ACPI: Sleep Button [SLPF]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.903469] GHES: HEST is not enabled!
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.906581] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.908821] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.914804] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.916480] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.923507] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.947138] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.971512] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    2.995687] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.020896] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.027093] Linux agpgart interface v0.103
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.033129] loop: module loaded
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.034973] libphy: Fixed MDIO Bus: probed
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.037050] tun: Universal TUN/TAP device driver, 1.6
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.040122] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.093868] PPP generic driver version 2.4.2
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.096142] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.099644] ehci-pci: EHCI PCI platform driver
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.102960] ehci-platform: EHCI generic platform driver
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.106624] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.110887] ohci-pci: OHCI PCI platform driver
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.113905] ohci-platform: OHCI generic platform driver
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.117773] uhci_hcd: USB Universal Host Controller Interface driver
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.121487] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.127323] i8042: Warning: Keylock active
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.130878] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.135018] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.140763] mousedev: PS/2 mouse device common for all mice
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.145456] rtc_cmos 00:00: RTC can wake from S4
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.149723] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.153928] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.157776] i2c /dev entries driver
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.160229] device-mapper: uevent: version 1.0.3
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.163849] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.168952] ledtrig-cpu: registered to indicate activity on CPUs
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.174492] NET: Registered protocol family 10
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.177807] NET: Registered protocol family 17
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.181463] Key type dns_resolver registered
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.185321] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.190496] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.194570] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.198010] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.201862] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.207607] registered taskstats version 1
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.210037] Loading compiled-in X.509 certificates
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.213142] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.218922] zswap: loaded using pool lzo/zbud
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.225727] Key type trusted registered
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.233928] Key type encrypted registered
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.237563] ima: No TPM chip found, activating TPM-bypass!
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.240529] evm: HMAC attrs: 0x1
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.243492]   Magic number: 14:882:971
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.246452] rtc_cmos 00:00: setting system clock to 2018-08-12 05:59:43 UTC (1534053583)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.254369] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.258156] EDD information not available.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.260775] PM: Hibernation image not present or could not be loaded.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.262531] Freeing unused kernel memory: 1496K
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.266102] Write protecting the kernel read-only data: 14336k
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.270653] Freeing unused kernel memory: 1956K
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.274897] Freeing unused kernel memory: 92K
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.297178] systemd-udevd[118]: starting version 204
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.344480] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.379500] scsi host0: Virtio SCSI HBA
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.389594] AVX version of gcm_enc/dec engaged.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.392870] AES CTR mode by8 optimization enabled
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.396189] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.483699] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.483702] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.492030] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.497157] sd 0:0:1:0: [sda] Write Protect is off
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.500406] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.501015] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.510662]  sda: sda1
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.514378] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.856284] tsc: Refined TSC clocksource calibration: 2600.001 MHz
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    3.860288] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3ce1c4c, max_idle_ns: 440795206275 ns
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    4.221323] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    6.372509] floppy0: no floppy controllers found
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    7.540155] raid6: sse2x1   gen()  8729 MB/s
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    7.608147] raid6: sse2x1   xor()  6473 MB/s
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    7.676141] raid6: sse2x2   gen() 10658 MB/s
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    7.744189] raid6: sse2x2   xor()  7065 MB/s
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    7.812144] raid6: sse2x4   gen() 12418 MB/s
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    7.880181] raid6: sse2x4   xor()  8658 MB/s
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    7.884657] raid6: using algorithm sse2x4 gen() 12418 MB/s
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    7.888133] raid6: .... xor() 8658 MB/s, rmw enabled
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    7.891329] raid6: using ssse3x2 recovery algorithm
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    7.897723] xor: automatically using best checksumming function:
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    7.940264]    avx       : 27592.000 MB/sec
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    7.959591] Btrfs loaded
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    8.027005] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    8.031980] EXT4-fs (sda1): write access will be enabled during recovery
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    8.137071] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    8.147657] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    8.150960] EXT4-fs (sda1): recovery complete
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    8.160857] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    8.424637] random: init: uninitialized urandom read (12 bytes read, 22 bits of entropy available)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    8.580996] random: mountall: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    8.640450] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    8.888776] random: cloud-init: uninitialized urandom read (32 bytes read, 32 bits of entropy available)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    9.557728] random: cloud-init: uninitialized urandom read (32 bytes read, 40 bits of entropy available)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    9.707883] systemd-udevd[702]: starting version 204
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    9.856419] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    9.922174] intel_rapl: no valid rapl domains found in package 0
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    9.976724] intel_rapl: no valid rapl domains found in package 0
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [    9.998706] ppdev: user-space parallel port driver
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   10.110140] random: mktemp: uninitialized urandom read (6 bytes read, 50 bits of entropy available)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   10.175727] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   10.248264] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   10.422294] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   10.801545] random: mktemp: uninitialized urandom read (12 bytes read, 54 bits of entropy available)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   10.886712] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   10.972838] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   11.030916] EXT4-fs (sda1): resized filesystem to 7864064
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   11.363755] init: failsafe main process (1093) killed by TERM signal
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO Running set_multiqueue.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO Set channels for eth0 to 4.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 12 05:59:51 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 12 05:59:52 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug 12 05:59:52 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 12 05:59:52 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 12 05:59:52 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Starting Google Accounts daemon.
Aug 12 05:59:52 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Creating a new user account for me.
Aug 12 05:59:52 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Created user account me.
Aug 12 05:59:52 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Creating a new user account for henrik.
Aug 12 05:59:52 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Created user account henrik.
Aug 12 05:59:52 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Creating a new user account for emma.
Aug 12 05:59:52 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Created user account emma.
Aug 12 05:59:52 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Creating a new user account for igor.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-clock-skew: INFO Synced system time with hardware clock.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Created user account igor.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Created user account konstantinhaase.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Creating a new user account for aj.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Created user account aj.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Creating a new user account for solarce.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Created user account solarce.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Creating a new user account for asari.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   12.856918] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   12.860861] Bridge firewalling registered
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   12.873723] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Created user account asari.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Creating a new user account for bogdana.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   12.921958] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   12.936268] floppy0: no floppy controllers found
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   12.967518] random: nonblocking pool is initialized
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Created user account bogdana.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Creating a new user account for konstantin.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   13.012020] Initializing XFRM netlink socket
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   13.027428] Netfilter messages via NETLINK v0.30.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   13.032355] ctnetlink v0.93: registering with nfnetlink.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Created user account konstantin.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Creating a new user account for carmen.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Created user account carmen.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Creating a new user account for maria.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Created user account maria.
Aug 12 05:59:53 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 google-accounts: INFO Removing user packer.
Aug 12 05:59:54 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 cron[1707]: (CRON) INFO (pidfile fd = 3)
Aug 12 05:59:54 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 cron[1738]: (CRON) STARTUP (fork ok)
Aug 12 05:59:54 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 cron[1738]: (CRON) INFO (Running @reboot jobs)
Aug 12 05:59:54 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 12 05:59:54 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 12 05:59:54 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 acpid: starting up with netlink and the input layer
Aug 12 05:59:54 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 acpid: 1 rule loaded
Aug 12 05:59:54 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 acpid: waiting for events: event logging is off
Aug 12 05:59:54 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 haveged: haveged starting up
Aug 12 05:59:54 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   14.002817] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1852]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1853]: proto: precision = 0.110 usec
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1853]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1853]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1853]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1853]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1853]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1853]: Listen normally on 3 eth0 10.20.0.60 UDP 123
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1853]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1853]: peers refreshed
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1853]: Listening on routing socket on fd #21 for interface updates
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   19.184241] init: plymouth-upstart-bridge main process ended, respawning
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 startup-script: INFO Found startup-script in metadata.
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 startup-script: INFO startup-script: job 1 at Sun Aug 12 09:09:00 2018
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 startup-script: INFO startup-script: Return code 0.
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 startup-script: INFO startup-script: Return code 0.
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 startup-script: INFO Finished running startup scripts.
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ec2: 
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ec2: #############################################################
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ec2: 1024 90:63:8f:d1:fb:74:12:02:6f:6c:dc:32:83:36:f1:c5  root@travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 (DSA)
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ec2: 256 99:32:96:07:e8:e5:56:d1:5a:16:33:68:4d:92:dc:85  root@travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 (ECDSA)
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ec2: 256 6e:58:61:8a:78:39:4b:e7:9b:a3:42:00:df:53:f4:c7  root@travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 (ED25519)
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ec2: 2048 84:52:60:a0:3c:ad:e4:67:39:ae:58:05:e7:b1:8d:12  root@travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 (RSA)
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 12 05:59:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ec2: #############################################################
Aug 12 06:00:08 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpdate[2254]: the NTP socket is in use, exiting
Aug 12 06:00:44 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [   64.203368] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 12 06:04:35 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [  295.056434] device veth8d78803 entered promiscuous mode
Aug 12 06:04:35 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [  295.056487] docker0: port 1(veth8d78803) entered forwarding state
Aug 12 06:04:35 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [  295.056494] docker0: port 1(veth8d78803) entered forwarding state
Aug 12 06:04:35 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [  295.056938] docker0: port 1(veth8d78803) entered disabled state
Aug 12 06:04:35 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [  295.174124] cgroup: docker-runc (4986) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 12 06:04:35 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [  295.174128] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 12 06:04:35 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [  295.257622] eth0: renamed from veth62d79d7
Aug 12 06:04:35 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [  295.297845] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 12 06:04:35 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [  295.299177] docker0: port 1(veth8d78803) entered forwarding state
Aug 12 06:04:35 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [  295.299198] docker0: port 1(veth8d78803) entered forwarding state
Aug 12 06:04:35 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [  295.299254] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 12 06:04:39 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1853]: Listen normally on 5 docker0 fe80::42:b8ff:fe71:ddf5 UDP 123
Aug 12 06:04:39 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1853]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 12 06:04:39 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1853]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 12 06:04:39 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1853]: peers refreshed
Aug 12 06:04:39 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1853]: new interface(s) found: waking up resolver
Aug 12 06:04:50 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [  310.300832] docker0: port 1(veth8d78803) entered forwarding state
Aug 12 06:17:01 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 CRON[12257]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 12 06:25:01 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 CRON[22800]: (root) CMD (test -x /usr/sbin/anacron || ( cd / && run-parts --report /etc/cron.daily ))
Aug 12 06:41:09 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 upstart-socket-bridge[847]: Disconnected from Upstart
Aug 12 06:41:09 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 upstart-udev-bridge[696]: Disconnected from Upstart
Aug 12 06:41:09 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [ 2488.715150] init: upstart-udev-bridge main process (696) terminated with status 1
Aug 12 06:41:09 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [ 2488.715165] init: upstart-udev-bridge main process ended, respawning
Aug 12 06:41:09 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [ 2488.715345] init: upstart-socket-bridge main process (847) terminated with status 1
Aug 12 06:41:09 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [ 2488.715355] init: upstart-socket-bridge main process ended, respawning
Aug 12 06:41:09 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [ 2488.715467] init: upstart-file-bridge main process (1203) terminated with status 1
Aug 12 06:41:09 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [ 2488.715477] init: upstart-file-bridge main process ended, respawning
Aug 12 06:41:23 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 dbus[1160]: [system] Reloaded configuration
Aug 12 06:41:23 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 dbus[1160]: message repeated 9 times: [ [system] Reloaded configuration]
Aug 12 06:41:28 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[1853]: ntpd exiting on signal 15
Aug 12 06:41:41 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [ 2520.735510] init: apport post-stop process (24131) terminated with status 1
Aug 12 06:41:44 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 dbus[1160]: [system] Reloaded configuration
Aug 12 06:41:56 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 kernel: [ 2536.299477] systemd-udevd[26097]: starting version 204
Aug 12 06:41:57 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 dbus[1160]: message repeated 10 times: [ [system] Reloaded configuration]
Aug 12 06:41:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[26280]: ntpd 4.2.6p5@1.2349-o Fri Jul  6 20:19:54 UTC 2018 (1)
Aug 12 06:41:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[26281]: ntp_io: estimated max descriptors: 72000, initial socket boundary: 16
Aug 12 06:41:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[26281]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 12 06:41:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[26281]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 12 06:41:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[26281]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 12 06:41:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[26281]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 12 06:41:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[26281]: Listen normally on 3 eth0 10.20.0.60 UDP 123
Aug 12 06:41:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[26281]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 12 06:41:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[26281]: peers refreshed
Aug 12 06:41:59 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 ntpd[26281]: Listening on routing socket on fd #21 for interface updates
Aug 12 06:42:00 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 12 06:42:00 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 12 06:42:03 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 dbus[1160]: [system] Reloaded configuration
Aug 12 06:47:01 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 CRON[27615]: (root) CMD (test -x /usr/sbin/anacron || ( cd / && run-parts --report /etc/cron.weekly ))
Aug 12 07:17:01 travis-job-48ad7ebc-ce31-474a-b769-77e7f29a4e12 CRON[7239]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
---
travis_time:end:0e86b81a:start=1534058565355034313,finish=1534058565365296958,duration=10262645
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:022e625e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08d783e7
travis_time:start:08d783e7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:1dec21c0
$ dmesg | grep -i kill
