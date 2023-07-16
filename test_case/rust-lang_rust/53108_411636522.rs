plain
[00:58:00]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:58:00]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:58:04] [RUSTC-TIMING] rustc_llvm test:false 0.205

Broadcast message from root@travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6
 (unknown) at 4:54 ...
The system is going down for power off NOW!
[00:58:14] 
[00:58:14] Session terminated, terminating shell... ...terminated.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:0cd781be
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:08789cac
$ sudo tail -n 500 /var/log/syslog
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] Policy zone: Normal
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] console [ttyS0] enabled
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.301424] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.302598] pid_max: default: 32768 minimum: 301
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.303236] ACPI: Core revision 20150930
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.309463] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.310472] Security Framework initialized
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.311036] Yama: becoming mindful.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.311665] AppArmor: AppArmor disabled by boot time parameter
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.314066] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.322710] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.326796] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.327896] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.329151] Initializing cgroup subsys io
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.329763] Initializing cgroup subsys memory
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.330367] Initializing cgroup subsys devices
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.331035] Initializing cgroup subsys freezer
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.331664] Initializing cgroup subsys net_cls
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.332273] Initializing cgroup subsys perf_event
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.332999] Initializing cgroup subsys net_prio
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.333625] Initializing cgroup subsys hugetlb
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.334234] Initializing cgroup subsys pids
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.334967] CPU: Physical Processor ID: 0
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.335521] CPU: Processor Core ID: 0
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.336827] mce: CPU supports 32 MCE banks
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.337532] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.338357] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.341547] Freeing SMP alternatives memory: 32K
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.351037] ftrace: allocating 32185 entries in 126 pages
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.404077] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.405196] smpboot: Max logical packages: 2
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.406392] x2apic enabled
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.407871] Switched APIC routing to physical x2apic.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.411487] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.516704] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.518351] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.520631] x86: Booting SMP configuration:
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.521323] .... node  #0, CPUs:      #1
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.522241] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.526259]  #2
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.526730] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.531747]  #3
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.532206] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.536385] x86: Booted up 1 node, 4 CPUs
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.537127] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.539383] devtmpfs: initialized
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.543575] evm: security.selinux
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.544174] evm: security.SMACK64
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.544668] evm: security.SMACK64EXEC
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.545180] evm: security.SMACK64TRANSMUTE
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.545795] evm: security.SMACK64MMAP
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.546351] evm: security.ima
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.546773] evm: security.capability
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.547544] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.548987] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.550116] pinctrl core: initialized pinctrl subsystem
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.551097] RTC time:  3:55:14, date: 08/09/18
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.552591] NET: Registered protocol family 16
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.564752] cpuidle: using governor ladder
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.576753] cpuidle: using governor menu
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.577414] PCCT header not found.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.577995] ACPI: bus type PCI registered
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.578594] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.579637] PCI: Using configuration type 1 for base access
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.593621] ACPI: Added _OSI(Module Device)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.594368] ACPI: Added _OSI(Processor Device)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.595009] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.595671] ACPI: Added _OSI(Processor Aggregator Device)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.598952] ACPI: Executed 2 blocks of module-level executable AML code
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.621941] ACPI: Interpreter enabled
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.622574] ACPI: (supports S0 S3 S4 S5)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.623117] ACPI: Using IOAPIC for interrupt routing
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.623888] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.653606] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.654536] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.655524] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.656459] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.658670] PCI host bridge to bus 0000:00
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.659306] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.660399] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.661357] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.662440] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.663472] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.664535] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.664949] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.677281] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.690488] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.691967] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.696659] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.701173] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.712276] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.716869] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.720458] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.732021] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.734032] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.735987] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.738055] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.739987] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.760013] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.761269] vgaarb: loaded
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.761921] SCSI subsystem initialized
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.762623] libata version 3.00 loaded.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.762647] ACPI: bus type USB registered
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.763262] usbcore: registered new interface driver usbfs
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.764180] usbcore: registered new interface driver hub
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.765001] usbcore: registered new device driver usb
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.765925] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.766906] dmi: Firmware registration failed.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.767795] PCI: Using ACPI for IRQ routing
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.768462] PCI: pci_cache_line_size set to 64 bytes
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.768555] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.768556] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.768661] NetLabel: Initializing
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.769208] NetLabel:  domain hash size = 128
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.769827] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.770531] NetLabel:  unlabeled traffic allowed by default
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.771390] amd_nb: Cannot enumerate AMD northbridges
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.772222] clocksource: Switched to clocksource kvm-clock
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.779232] pnp: PnP ACPI init
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.779773] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.779836] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.779882] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.779933] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.779975] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.780017] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.780058] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.780203] pnp: PnP ACPI: found 7 devices
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.787615] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.789028] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.789030] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.789032] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.789033] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.789063] NET: Registered protocol family 2
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.789913] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.791894] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.792915] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.793984] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.794919] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.796600] NET: Registered protocol family 1
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.797352] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.798322] PCI: CLS 0 bytes, default 64
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    0.798368] Unpacking initramfs...
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.786165] Freeing initrd memory: 21432K
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.786875] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.787818] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.789297] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.790520] hw unit of domain pp0-core 2^-0 Joules
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.791187] hw unit of domain package 2^-0 Joules
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.791828] hw unit of domain dram 2^-16 Joules
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.792531] Scanning for low memory corruption every 60 seconds
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.793822] audit: initializing netlink subsys (disabled)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.794623] audit: type=2000 audit(1533786916.268:1): initialized
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.795722] Initialise system trusted keyring
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.796749] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.798049] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.800078] zbud: loaded
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.800733] VFS: Disk quotas dquot_6.6.0
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.801322] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.802476] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.803745] fuse init (API version 7.23)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.804619] Key type big_key registered
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.805278] Allocating IMA MOK and blacklist keyrings.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.807187] Key type asymmetric registered
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.807921] Asymmetric key parser 'x509' registered
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.808828] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.810050] io scheduler noop registered
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.810763] io scheduler deadline registered (default)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.811710] io scheduler cfq registered
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.812449] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.813267] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.814537] intel_idle: does not run on family 6 model 63
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.814634] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.816065] ACPI: Power Button [PWRF]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.816842] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.818852] ACPI: Sleep Button [SLPF]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.819904] GHES: HEST is not enabled!
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.822243] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.823301] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.827565] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.828586] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.833383] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.855692] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.878320] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.900913] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.923654] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.927079] Linux agpgart interface v0.103
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.930364] loop: module loaded
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.931140] libphy: Fixed MDIO Bus: probed
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.931851] tun: Universal TUN/TAP device driver, 1.6
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.932571] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.959924] PPP generic driver version 2.4.2
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.960795] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.961795] ehci-pci: EHCI PCI platform driver
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.962528] ehci-platform: EHCI generic platform driver
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.963239] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.964102] ohci-pci: OHCI PCI platform driver
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.967486] ohci-platform: OHCI generic platform driver
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.968388] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.969412] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.971117] i8042: Warning: Keylock active
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.972751] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.973585] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.974622] mousedev: PS/2 mouse device common for all mice
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.975781] rtc_cmos 00:00: RTC can wake from S4
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.976858] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.978063] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.979128] i2c /dev entries driver
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.979942] device-mapper: uevent: version 1.0.3
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.981145] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.982585] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.984164] NET: Registered protocol family 10
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.985397] NET: Registered protocol family 17
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.986250] Key type dns_resolver registered
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.987366] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.988412] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.989352] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.990839] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.991740] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.993204] registered taskstats version 1
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.993871] Loading compiled-in X.509 certificates
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.995192] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.996873] zswap: loaded using pool lzo/zbud
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    2.999267] Key type trusted registered
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.003162] Key type encrypted registered
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.004302] ima: No TPM chip found, activating TPM-bypass!
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.005275] evm: HMAC attrs: 0x1
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.006175]   Magic number: 14:892:916
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.006801] tty ptmx: hash matches
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.007550] rtc_cmos 00:00: setting system clock to 2018-08-09 03:55:16 UTC (1533786916)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.009034] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.010157] EDD information not available.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.010894] PM: Hibernation image not present or could not be loaded.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.012214] Freeing unused kernel memory: 1496K
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.012872] Write protecting the kernel read-only data: 14336k
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.014414] Freeing unused kernel memory: 1956K
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.015434] Freeing unused kernel memory: 92K
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.028180] systemd-udevd[118]: starting version 204
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.087382] scsi host0: Virtio SCSI HBA
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.091861] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.097077] AVX2 version of gcm_enc/dec engaged.
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.097867] AES CTR mode by8 optimization enabled
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.133276] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.133285] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.135163] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.136275] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.136981] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.137069] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.139657]  sda: sda1
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.140749] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.176878] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.792346] tsc: Refined TSC clocksource calibration: 2300.001 MHz
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    3.793417] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735f0517, max_idle_ns: 440795237604 ns
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    4.014025] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    6.084393] floppy0: no floppy controllers found
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.240277] raid6: sse2x1   gen()  8740 MB/s
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.308249] raid6: sse2x1   xor()  6778 MB/s
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.376275] raid6: sse2x2   gen() 11035 MB/s
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.444252] raid6: sse2x2   xor()  7468 MB/s
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.512262] raid6: sse2x4   gen() 12757 MB/s
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.580269] raid6: sse2x4   xor()  8963 MB/s
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.648266] raid6: avx2x1   gen() 16961 MB/s
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.716284] raid6: avx2x2   gen() 19811 MB/s
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.784264] raid6: avx2x4   gen() 22852 MB/s
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.784905] raid6: using algorithm avx2x4 gen() 22852 MB/s
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.785649] raid6: using avx2x2 recovery algorithm
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.787704] xor: automatically using best checksumming function:
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.828273]    avx       : 27379.000 MB/sec
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.842175] Btrfs loaded
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.882403] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.883365] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.944031] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.949200] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.949942] EXT4-fs (sda1): recovery complete
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    7.953925] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    8.135686] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    8.234910] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    8.277375] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    8.439665] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    8.898412] random: cloud-init: uninitialized urandom read (32 bytes read, 44 bits of entropy available)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    9.016554] systemd-udevd[700]: starting version 204
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    9.110857] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    9.195330] intel_rapl: no valid rapl domains found in package 0
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    9.262875] ppdev: user-space parallel port driver
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    9.284895] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    9.333637] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    9.395155] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    9.556458] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    9.786777] random: mktemp: uninitialized urandom read (12 bytes read, 60 bits of entropy available)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    9.856646] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    9.918174] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [    9.952696] EXT4-fs (sda1): resized filesystem to 7864064
Aug  9 03:55:23 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [   10.237174] init: failsafe main process (1092) killed by TERM signal
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO Running set_multiqueue.
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO Set channels for eth0 to 4.
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [   10.945367] random: nonblocking pool is initialized
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Starting Google Accounts daemon.
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Creating a new user account for me.
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Created user account me.
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 cron[1393]: (CRON) INFO (pidfile fd = 3)
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 cron[1430]: (CRON) STARTUP (fork ok)
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 cron[1430]: (CRON) INFO (Running @reboot jobs)
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Creating a new user account for henrik.
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 acpid: starting up with netlink and the input layer
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 acpid: 1 rule loaded
Aug  9 03:55:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 acpid: waiting for events: event logging is off
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Created user account henrik.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Creating a new user account for emma.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 haveged: haveged starting up
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Created user account emma.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Creating a new user account for igor.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [   11.421683] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [   11.430742] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Created user account igor.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Created user account konstantinhaase.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Creating a new user account for aj.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Created user account aj.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Creating a new user account for solarce.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Created user account solarce.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Creating a new user account for asari.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Created user account asari.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Creating a new user account for bogdana.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Created user account bogdana.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Creating a new user account for konstantin.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Created user account konstantin.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [   11.713553] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [   11.717104] Bridge firewalling registered
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [   11.726614] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Creating a new user account for carmen.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Created user account carmen.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Creating a new user account for maria.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [   11.793774] Initializing XFRM netlink socket
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [   11.800724] Netfilter messages via NETLINK v0.30.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [   11.810005] ctnetlink v0.93: registering with nfnetlink.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Created user account maria.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 google-accounts: INFO Removing user packer.
Aug  9 03:55:25 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [   12.188356] floppy0: no floppy controllers found
Aug  9 03:55:48 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpdate[1858]: adjust time server 169.254.169.254 offset 0.001674 sec
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpd[1894]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpd[1895]: proto: precision = 0.164 usec
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpd[1895]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpd[1895]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpd[1895]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpd[1895]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpd[1895]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpd[1895]: Listen normally on 3 eth0 10.20.1.42 UDP 123
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpd[1895]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpd[1895]: peers refreshed
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpd[1895]: Listening on routing socket on fd #21 for interface updates
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [   41.594069] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 startup-script: INFO Found startup-script in metadata.
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 startup-script: INFO startup-script: job 1 at Thu Aug  9 07:05:00 2018
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 startup-script: INFO startup-script: Return code 0.
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 startup-script: INFO startup-script: Return code 0.
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 startup-script: INFO Finished running startup scripts.
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ec2: 
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ec2: #############################################################
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ec2: 1024 54:28:12:2b:f0:95:cd:c2:45:2f:f1:b1:52:f0:87:63  root@travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 (DSA)
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ec2: 256 0c:ba:26:cb:40:66:bb:c6:26:18:b3:1d:f9:5a:97:ca  root@travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 (ECDSA)
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ec2: 256 cf:cb:ab:a0:a1:c0:81:10:e9:ba:93:dc:f4:a8:e4:0a  root@travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 (ED25519)
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ec2: 2048 10:17:f8:24:44:8e:72:01:1b:e6:af:d3:2d:c9:5a:fe  root@travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 (RSA)
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 03:55:55 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ec2: #############################################################
Aug  9 03:56:24 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [   71.028556] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 03:58:26 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [  192.591713] device vethc552723 entered promiscuous mode
Aug  9 03:58:26 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [  192.678986] cgroup: docker-runc (4895) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 03:58:26 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [  192.678989] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  9 03:58:26 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [  192.746530] eth0: renamed from veth00b5696
Aug  9 03:58:26 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [  192.780561] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  9 03:58:26 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [  192.781667] docker0: port 1(vethc552723) entered forwarding state
Aug  9 03:58:26 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [  192.781686] docker0: port 1(vethc552723) entered forwarding state
Aug  9 03:58:26 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [  192.781713] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  9 03:58:30 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpd[1895]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  9 03:58:30 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpd[1895]: Listen normally on 6 docker0 fe80::42:8ff:fe9d:6cb7 UDP 123
Aug  9 03:58:30 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpd[1895]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  9 03:58:30 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpd[1895]: peers refreshed
Aug  9 03:58:30 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 ntpd[1895]: new interface(s) found: waking up resolver
Aug  9 03:58:41 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [  207.826284] docker0: port 1(vethc552723) entered forwarding state
Aug  9 04:17:01 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 CRON[15078]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 dbus[1143]: [system] Activating service name='org.freedesktop.systemd1' (using servicehelper)
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 dbus[1143]: [system] Successfully activated service 'org.freedesktop.systemd1'
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.886838] init: tty4 main process (1387) killed by TERM signal
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.887091] init: tty5 main process (1391) killed by TERM signal
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.887321] init: tty2 main process (1395) killed by TERM signal
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.887556] init: tty3 main process (1396) killed by TERM signal
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.887788] init: tty6 main process (1398) killed by TERM signal
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.888241] init: cron main process (1430) killed by TERM signal
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.888686] init: irqbalance main process (1438) killed by TERM signal
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.889037] init: tty1 main process (1934) killed by TERM signal
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.889255] init: ttyS0 main process (1943) killed by TERM signal
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.889818] init: plymouth-upstart-bridge main process (15158) terminated with status 1
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.889827] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.952126] docker0: port 1(vethc552723) entered disabled state
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.952305] veth00b5696: renamed from eth0
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.962459] init: plymouth-upstart-bridge main process (15204) terminated with status 1
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.962471] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.991248] init: plymouth-upstart-bridge main process (15216) terminated with status 1
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3564.991259] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.001789] init: plymouth-upstart-bridge main process (15219) terminated with status 1
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.001799] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.005892] init: plymouth-upstart-bridge main process (15224) terminated with status 1
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.005902] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.013788] init: apport post-stop process (15154) terminated with status 1
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.014222] init: plymouth-upstart-bridge main process (15231) terminated with status 1
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.014232] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.022480] docker0: port 1(vethc552723) entered disabled state
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.023225] init: plymouth-upstart-bridge main process (15238) terminated with status 1
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.023236] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.024600] device vethc552723 left promiscuous mode
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.024603] docker0: port 1(vethc552723) entered disabled state
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.030669] init: plymouth-upstart-bridge main process (15240) terminated with status 1
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.030679] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.037728] init: plymouth-upstart-bridge main process (15242) terminated with status 1
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.037739] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.043352] init: plymouth-upstart-bridge main process (15244) terminated with status 1
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.043360] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.050230] init: plymouth-upstart-bridge main process (15246) terminated with status 1
Aug  9 04:54:38 travis-job-7d87c75c-4fa0-4f65-8a56-b404a5bd5aa6 kernel: [ 3565.050240] init: plymouth-upstart-bridge respawning too fast, stopped
