plain
[01:07:59] [RUSTC-TIMING] rustc_passes test:false 31.540
[01:07:59]    Compiling rustc_codegen_utils v0.0.0 (file:///checkout/src/librustc_codegen_utils)
[01:08:01] [RUSTC-TIMING] rustc_save_analysis test:false 67.219

Broadcast message from root@travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a
 (unknown) at 21:17 ...
The system is going down for power off NOW!
[01:08:05] 
[01:08:05] Session terminated, terminating shell... ...terminated.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:07203926
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:1fb51d83
$ sudo tail -n 500 /var/log/syslog
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] Policy zone: Normal
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] Hierarchical RCU implementation.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] console [ttyS0] enabled
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.302343] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.303722] pid_max: default: 32768 minimum: 301
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.304834] ACPI: Core revision 20150930
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.311589] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.312836] Security Framework initialized
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.313647] Yama: becoming mindful.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.314250] AppArmor: AppArmor disabled by boot time parameter
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.316964] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.326305] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.330765] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.331813] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.333004] Initializing cgroup subsys io
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.333669] Initializing cgroup subsys memory
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.334285] Initializing cgroup subsys devices
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.334931] Initializing cgroup subsys freezer
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.335755] Initializing cgroup subsys net_cls
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.336487] Initializing cgroup subsys perf_event
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.337385] Initializing cgroup subsys net_prio
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.338403] Initializing cgroup subsys hugetlb
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.339195] Initializing cgroup subsys pids
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.340014] CPU: Physical Processor ID: 0
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.340594] CPU: Processor Core ID: 0
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.342041] mce: CPU supports 32 MCE banks
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.342863] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.344136] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.347578] Freeing SMP alternatives memory: 32K
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.357722] ftrace: allocating 32185 entries in 126 pages
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.412929] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.414086] smpboot: Max logical packages: 2
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.415341] x2apic enabled
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.416886] Switched APIC routing to physical x2apic.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.420778] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.529565] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.531282] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.533545] x86: Booting SMP configuration:
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.534239] .... node  #0, CPUs:      #1
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.535103] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.539297]  #2
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.539745] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.543879]  #3
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.544465] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.548491] x86: Booted up 1 node, 4 CPUs
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.549080] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.551515] devtmpfs: initialized
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.555512] evm: security.selinux
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.556095] evm: security.SMACK64
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.556718] evm: security.SMACK64EXEC
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.557284] evm: security.SMACK64TRANSMUTE
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.557899] evm: security.SMACK64MMAP
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.558512] evm: security.ima
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.558941] evm: security.capability
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.559771] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.561148] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.562233] pinctrl core: initialized pinctrl subsystem
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.563253] RTC time: 20:07:54, date: 08/13/18
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.564801] NET: Registered protocol family 16
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.577625] cpuidle: using governor ladder
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.589605] cpuidle: using governor menu
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.590435] PCCT header not found.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.591102] ACPI: bus type PCI registered
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.591955] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.593095] PCI: Using configuration type 1 for base access
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.606516] ACPI: Added _OSI(Module Device)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.607203] ACPI: Added _OSI(Processor Device)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.607895] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.608572] ACPI: Added _OSI(Processor Aggregator Device)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.611833] ACPI: Executed 2 blocks of module-level executable AML code
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.634774] ACPI: Interpreter enabled
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.635433] ACPI: (supports S0 S3 S4 S5)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.636031] ACPI: Using IOAPIC for interrupt routing
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.636924] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.666941] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.667984] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.668994] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.670164] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.672691] PCI host bridge to bus 0000:00
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.673324] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.674306] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.675273] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.676361] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.677388] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.678448] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.678850] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.689907] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.701156] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.702667] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.707601] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.711212] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.720785] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.725007] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.728229] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.738540] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.741203] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.743424] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.745455] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.747502] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.767835] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.769018] vgaarb: loaded
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.769650] SCSI subsystem initialized
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.770358] libata version 3.00 loaded.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.770389] ACPI: bus type USB registered
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.771144] usbcore: registered new interface driver usbfs
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.772043] usbcore: registered new interface driver hub
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.772842] usbcore: registered new device driver usb
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.773767] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.774774] dmi: Firmware registration failed.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.775704] PCI: Using ACPI for IRQ routing
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.776323] PCI: pci_cache_line_size set to 64 bytes
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.776419] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.776420] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.776536] NetLabel: Initializing
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.777020] NetLabel:  domain hash size = 128
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.777618] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.778303] NetLabel:  unlabeled traffic allowed by default
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.779188] amd_nb: Cannot enumerate AMD northbridges
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.779961] clocksource: Switched to clocksource kvm-clock
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.787267] pnp: PnP ACPI init
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.787858] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.787931] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.787993] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.788050] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.788098] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.788144] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.788199] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.788371] pnp: PnP ACPI: found 7 devices
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.796463] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.797803] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.797806] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.797808] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.797809] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.797836] NET: Registered protocol family 2
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.798651] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.800496] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.801482] TCP: Hash tables configured (established 131072 bind 65536)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.802393] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.803283] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.804879] NET: Registered protocol family 1
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.805495] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.806360] PCI: CLS 0 bytes, default 64
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    0.806401] Unpacking initramfs...
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.816084] Freeing initrd memory: 21432K
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.816993] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.818067] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.819766] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.821368] hw unit of domain pp0-core 2^-0 Joules
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.822461] hw unit of domain package 2^-0 Joules
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.823666] hw unit of domain dram 2^-16 Joules
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.824634] Scanning for low memory corruption every 60 seconds
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.826096] audit: initializing netlink subsys (disabled)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.827079] audit: type=2000 audit(1534190876.755:1): initialized
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.828540] Initialise system trusted keyring
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.829394] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.830325] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.832448] zbud: loaded
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.833085] VFS: Disk quotas dquot_6.6.0
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.833754] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.834940] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.836072] fuse init (API version 7.23)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.837026] Key type big_key registered
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.837608] Allocating IMA MOK and blacklist keyrings.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.839392] Key type asymmetric registered
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.839980] Asymmetric key parser 'x509' registered
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.840849] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.842344] io scheduler noop registered
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.842964] io scheduler deadline registered (default)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.843711] io scheduler cfq registered
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.844481] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.845483] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.846615] intel_idle: does not run on family 6 model 63
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.846700] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.848342] ACPI: Power Button [PWRF]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.848992] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.850401] ACPI: Sleep Button [SLPF]
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.851352] GHES: HEST is not enabled!
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.853986] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.854924] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.858546] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.859631] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.863640] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.886232] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.910128] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.934236] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.957973] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.961271] Linux agpgart interface v0.103
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.964480] loop: module loaded
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.965513] libphy: Fixed MDIO Bus: probed
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.966712] tun: Universal TUN/TAP device driver, 1.6
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.968301] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.994734] PPP generic driver version 2.4.2
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.996290] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.998379] ehci-pci: EHCI PCI platform driver
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    2.999801] ehci-platform: EHCI generic platform driver
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.001428] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.003282] ohci-pci: OHCI PCI platform driver
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.004771] ohci-platform: OHCI generic platform driver
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.006409] uhci_hcd: USB Universal Host Controller Interface driver
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.008401] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.011761] i8042: Warning: Keylock active
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.014066] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.015520] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.017501] mousedev: PS/2 mouse device common for all mice
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.018789] rtc_cmos 00:00: RTC can wake from S4
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.020509] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.022495] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.024321] i2c /dev entries driver
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.025358] device-mapper: uevent: version 1.0.3
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.026825] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.029269] ledtrig-cpu: registered to indicate activity on CPUs
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.031676] NET: Registered protocol family 10
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.033237] NET: Registered protocol family 17
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.034598] Key type dns_resolver registered
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.036045] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.037823] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.039346] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.040648] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.041894] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.044786] registered taskstats version 1
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.046063] Loading compiled-in X.509 certificates
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.048269] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.051212] zswap: loaded using pool lzo/zbud
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.054380] Key type trusted registered
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.058909] Key type encrypted registered
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.060071] ima: No TPM chip found, activating TPM-bypass!
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.061585] evm: HMAC attrs: 0x1
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.062860]   Magic number: 14:176:144
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.064242] acpi device:15: hash matches
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.065440] rtc_cmos 00:00: setting system clock to 2018-08-13 20:07:57 UTC (1534190877)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.068083] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.069730] EDD information not available.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.070977] PM: Hibernation image not present or could not be loaded.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.072373] Freeing unused kernel memory: 1496K
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.073041] Write protecting the kernel read-only data: 14336k
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.074703] Freeing unused kernel memory: 1956K
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.075649] Freeing unused kernel memory: 92K
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.088856] systemd-udevd[119]: starting version 204
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.136594] scsi host0: Virtio SCSI HBA
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.145544] AVX2 version of gcm_enc/dec engaged.
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.146254] AES CTR mode by8 optimization enabled
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.147748] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.179624] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.179631] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.181618] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.182632] sd 0:0:1:0: [sda] Write Protect is off
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.183293] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.183364] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.186091]  sda: sda1
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.187154] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.216437] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.824264] tsc: Refined TSC clocksource calibration: 2300.000 MHz
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    3.825284] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735223b2, max_idle_ns: 440795277976 ns
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    4.054031] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    6.136324] floppy0: no floppy controllers found
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.287981] raid6: sse2x1   gen()  9027 MB/s
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.355977] raid6: sse2x1   xor()  6946 MB/s
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.423983] raid6: sse2x2   gen() 10944 MB/s
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.492029] raid6: sse2x2   xor()  7449 MB/s
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.559991] raid6: sse2x4   gen() 12615 MB/s
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.627982] raid6: sse2x4   xor()  8773 MB/s
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.695984] raid6: avx2x1   gen() 16689 MB/s
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.763982] raid6: avx2x2   gen() 19087 MB/s
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.831981] raid6: avx2x4   gen() 22224 MB/s
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.832773] raid6: using algorithm avx2x4 gen() 22224 MB/s
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.833540] raid6: using avx2x2 recovery algorithm
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.835603] xor: automatically using best checksumming function:
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.875972]    avx       : 27413.000 MB/sec
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.889528] Btrfs loaded
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.923347] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.924461] EXT4-fs (sda1): write access will be enabled during recovery
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.986156] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.991132] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.991846] EXT4-fs (sda1): recovery complete
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    7.995828] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    8.188108] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    8.291486] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    8.331832] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    8.498938] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    8.954714] random: cloud-init: uninitialized urandom read (32 bytes read, 46 bits of entropy available)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    9.070801] systemd-udevd[702]: starting version 204
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    9.167776] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    9.238409] intel_rapl: no valid rapl domains found in package 0
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    9.280750] intel_rapl: no valid rapl domains found in package 0
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    9.293450] ppdev: user-space parallel port driver
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    9.397579] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    9.440113] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    9.504955] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    9.662627] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    9.918386] random: mktemp: uninitialized urandom read (12 bytes read, 62 bits of entropy available)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [    9.979752] random: mktemp: uninitialized urandom read (6 bytes read, 63 bits of entropy available)
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [   10.038662] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [   10.067881] EXT4-fs (sda1): resized filesystem to 7864064
Aug 13 20:08:04 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [   10.285550] init: failsafe main process (1096) killed by TERM signal
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO Running set_multiqueue.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO Set channels for eth0 to 4.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [   10.973094] random: nonblocking pool is initialized
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Starting Google Accounts daemon.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Creating a new user account for me.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Created user account me.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Creating a new user account for henrik.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Created user account henrik.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Creating a new user account for emma.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Created user account emma.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Creating a new user account for igor.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Created user account igor.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a cron[1415]: (CRON) INFO (pidfile fd = 3)
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a cron[1468]: (CRON) STARTUP (fork ok)
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a cron[1468]: (CRON) INFO (Running @reboot jobs)
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Created user account konstantinhaase.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a acpid: starting up with netlink and the input layer
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a acpid: 1 rule loaded
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a acpid: waiting for events: event logging is off
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Creating a new user account for aj.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Created user account aj.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a haveged: haveged starting up
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Creating a new user account for solarce.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Created user account solarce.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [   11.447408] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [   11.456863] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Creating a new user account for asari.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Created user account asari.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Creating a new user account for bogdana.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Created user account bogdana.
Aug 13 20:08:05 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Creating a new user account for konstantin.
Aug 13 20:08:06 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Created user account konstantin.
Aug 13 20:08:06 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Creating a new user account for carmen.
Aug 13 20:08:06 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Created user account carmen.
Aug 13 20:08:06 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Creating a new user account for maria.
Aug 13 20:08:06 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Created user account maria.
Aug 13 20:08:06 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [   11.668840] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 13 20:08:06 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [   11.671568] Bridge firewalling registered
Aug 13 20:08:06 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-accounts: INFO Removing user packer.
Aug 13 20:08:06 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [   11.679017] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 13 20:08:06 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [   11.734941] Initializing XFRM netlink socket
Aug 13 20:08:06 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [   11.740542] Netfilter messages via NETLINK v0.30.
Aug 13 20:08:06 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [   11.743042] ctnetlink v0.93: registering with nfnetlink.
Aug 13 20:08:06 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a google-clock-skew: INFO Synced system time with hardware clock.
Aug 13 20:08:06 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [   12.272035] floppy0: no floppy controllers found
Aug 13 20:08:28 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpdate[1843]: adjust time server 169.254.169.254 offset -0.001722 sec
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpd[1876]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpd[1877]: proto: precision = 0.103 usec
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpd[1877]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpd[1877]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpd[1877]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpd[1877]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpd[1877]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpd[1877]: Listen normally on 3 eth0 10.20.0.58 UDP 123
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpd[1877]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpd[1877]: peers refreshed
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpd[1877]: Listening on routing socket on fd #21 for interface updates
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [   41.634009] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a startup-script: INFO Found startup-script in metadata.
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a startup-script: INFO startup-script: job 1 at Mon Aug 13 23:18:00 2018
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a startup-script: INFO startup-script: Return code 0.
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a startup-script: INFO startup-script: Return code 0.
Aug 13 20:08:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a startup-script: INFO Finished running startup scripts.
Aug 13 20:08:36 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ec2: 
Aug 13 20:08:36 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ec2: #############################################################
Aug 13 20:08:36 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 13 20:08:36 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ec2: 1024 dc:47:90:eb:56:e3:2f:73:16:c6:de:30:97:8a:01:4e  root@travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a (DSA)
Aug 13 20:08:36 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ec2: 256 98:70:79:e5:9c:6f:89:7a:5b:83:6f:37:02:d2:97:2d  root@travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a (ECDSA)
Aug 13 20:08:36 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ec2: 256 cd:a9:cf:bf:73:47:b5:41:bf:25:17:40:3b:2a:fd:c7  root@travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a (ED25519)
Aug 13 20:08:36 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ec2: 2048 41:8d:5c:3e:44:fb:18:c8:38:90:f4:b6:90:41:e1:43  root@travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a (RSA)
Aug 13 20:08:36 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 13 20:08:36 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ec2: #############################################################
Aug 13 20:09:15 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [   81.749402] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 13 20:10:20 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [  145.987967] device vethf41ee6c entered promiscuous mode
Aug 13 20:10:20 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [  145.988042] docker0: port 1(vethf41ee6c) entered forwarding state
Aug 13 20:10:20 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [  145.988049] docker0: port 1(vethf41ee6c) entered forwarding state
Aug 13 20:10:20 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [  145.988834] docker0: port 1(vethf41ee6c) entered disabled state
Aug 13 20:10:20 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [  146.066926] cgroup: docker-runc (4938) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 13 20:10:20 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [  146.066929] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 13 20:10:20 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [  146.134885] eth0: renamed from veth436322d
Aug 13 20:10:20 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [  146.168915] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 13 20:10:20 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [  146.169898] docker0: port 1(vethf41ee6c) entered forwarding state
Aug 13 20:10:20 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [  146.169913] docker0: port 1(vethf41ee6c) entered forwarding state
Aug 13 20:10:20 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [  146.169936] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 13 20:10:23 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpd[1877]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 13 20:10:23 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpd[1877]: Listen normally on 6 docker0 fe80::42:1dff:feaa:52a1 UDP 123
Aug 13 20:10:23 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpd[1877]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 13 20:10:23 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpd[1877]: peers refreshed
Aug 13 20:10:23 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a ntpd[1877]: new interface(s) found: waking up resolver
Aug 13 20:10:35 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [  161.178238] docker0: port 1(vethf41ee6c) entered forwarding state
Aug 13 20:17:01 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a CRON[12318]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 13 21:17:01 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a CRON[28662]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 13 21:17:21 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a dbus[1150]: [system] Activating service name='org.freedesktop.systemd1' (using servicehelper)
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a dbus[1150]: [system] Successfully activated service 'org.freedesktop.systemd1'
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.351588] init: tty4 main process (1410) killed by TERM signal
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.351953] init: tty5 main process (1413) killed by TERM signal
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.352229] init: tty2 main process (1422) killed by TERM signal
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.352816] init: tty3 main process (1423) killed by TERM signal
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.353074] init: tty6 main process (1425) killed by TERM signal
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.353555] init: cron main process (1468) killed by TERM signal
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.354042] init: irqbalance main process (1489) killed by TERM signal
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.354297] init: tty1 main process (1918) killed by TERM signal
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.354582] init: ttyS0 main process (1927) killed by TERM signal
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.355051] init: plymouth-upstart-bridge main process (28755) terminated with status 1
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.355057] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.561481] init: plymouth-upstart-bridge main process (28806) terminated with status 1
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.561494] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.612579] init: plymouth-upstart-bridge main process (28816) terminated with status 1
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.612592] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.627667] init: plymouth-upstart-bridge main process (28819) terminated with status 1
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.627681] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.634622] veth436322d: renamed from eth0
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.640385] init: plymouth-upstart-bridge main process (28821) terminated with status 1
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.640398] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.666051] init: apport post-stop process (28750) terminated with status 1
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.666365] init: plymouth-upstart-bridge main process (28826) terminated with status 1
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.666376] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.675830] init: plymouth-upstart-bridge main process (28835) terminated with status 1
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.675839] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a shutdown-script: INFO Starting shutdown scripts.
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.682962] init: plymouth-upstart-bridge main process (28837) terminated with status 1
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.682978] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.685995] init: plymouth-upstart-bridge main process (28839) terminated with status 1
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.686006] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.688685] docker0: port 1(vethf41ee6c) entered disabled state
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a shutdown-script: INFO No shutdown scripts found in metadata.
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a shutdown-script: INFO Finished running shutdown scripts.
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.694070] init: plymouth-upstart-bridge main process (28840) terminated with status 1
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.694082] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.701125] init: plymouth-upstart-bridge main process (28846) terminated with status 1
Aug 13 21:17:22 travis-job-48841ecc-be24-4253-8dd1-43a48e8cf69a kernel: [ 4168.701150] init: plymouth-upstart-bridge respawning too fast, stopped
