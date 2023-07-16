plain
[00:30:37] [RUSTC-TIMING] proc_macro test:false 13.869
[00:30:37]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:31:06] [RUSTC-TIMING] syntax_ext test:false 29.064

Broadcast message from root@travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea
 (unknown) at 8:48 ...
The system is going down for power off NOW!
[00:31:46] 
[00:31:46] Session terminated, terminating shell... ...terminated.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:00482ed4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0e9784e6
$ sudo tail -n 500 /var/log/syslog
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] Policy zone: Normal
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] console [ttyS0] enabled
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.821163] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.828017] pid_max: default: 32768 minimum: 301
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.833426] ACPI: Core revision 20150930
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.843365] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.850071] Security Framework initialized
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.853310] Yama: becoming mindful.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.856982] AppArmor: AppArmor disabled by boot time parameter
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.864575] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.879153] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.889317] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.894658] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.900662] Initializing cgroup subsys io
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.903536] Initializing cgroup subsys memory
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.907500] Initializing cgroup subsys devices
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.911021] Initializing cgroup subsys freezer
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.914652] Initializing cgroup subsys net_cls
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.918024] Initializing cgroup subsys perf_event
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.921792] Initializing cgroup subsys net_prio
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.926063] Initializing cgroup subsys hugetlb
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.929803] Initializing cgroup subsys pids
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.933009] CPU: Physical Processor ID: 0
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.936537] CPU: Processor Core ID: 0
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.940081] mce: CPU supports 32 MCE banks
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.943051] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.948041] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.955386] Freeing SMP alternatives memory: 32K
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    0.967513] ftrace: allocating 32185 entries in 126 pages
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.018957] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.023860] smpboot: Max logical packages: 2
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.027579] x2apic enabled
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.031301] Switched APIC routing to physical x2apic.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.039846] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.152778] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.161862] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.170414] x86: Booting SMP configuration:
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.173532] .... node  #0, CPUs:      #1
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.177464] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.185792]  #2
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.187258] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.194178]  #3
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.195725] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.203556] x86: Booted up 1 node, 4 CPUs
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.206233] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.213483] devtmpfs: initialized
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.220524] evm: security.selinux
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.223292] evm: security.SMACK64
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.225308] evm: security.SMACK64EXEC
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.228664] evm: security.SMACK64TRANSMUTE
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.231759] evm: security.SMACK64MMAP
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.234234] evm: security.ima
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.237509] evm: security.capability
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.241390] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.248387] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.253568] pinctrl core: initialized pinctrl subsystem
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.258507] RTC time:  8:15:11, date: 08/07/18
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.262333] NET: Registered protocol family 16
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.276934] cpuidle: using governor ladder
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.292870] cpuidle: using governor menu
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.296337] PCCT header not found.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.298430] ACPI: bus type PCI registered
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.301171] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.306401] PCI: Using configuration type 1 for base access
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.323108] ACPI: Added _OSI(Module Device)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.326612] ACPI: Added _OSI(Processor Device)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.330655] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.334936] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.341893] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.371098] ACPI: Interpreter enabled
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.373621] ACPI: (supports S0 S3 S4 S5)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.377375] ACPI: Using IOAPIC for interrupt routing
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.381023] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.417124] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.422230] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.428496] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.433286] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.442418] PCI host bridge to bus 0000:00
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.445539] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.452800] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.460616] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.468292] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.478508] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.483861] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.484336] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.518981] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.552902] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.558880] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.574588] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.585476] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.627776] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.657315] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.670818] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.698343] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.706508] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.712987] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.720530] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.727260] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.751390] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.756039] vgaarb: loaded
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.758453] SCSI subsystem initialized
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.762376] libata version 3.00 loaded.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.762425] ACPI: bus type USB registered
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.765688] usbcore: registered new interface driver usbfs
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.770590] usbcore: registered new interface driver hub
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.778019] usbcore: registered new device driver usb
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.783241] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.788946] dmi: Firmware registration failed.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.793095] PCI: Using ACPI for IRQ routing
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.797649] PCI: pci_cache_line_size set to 64 bytes
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.797792] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.797795] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.797964] NetLabel: Initializing
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.802728] NetLabel:  domain hash size = 128
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.805958] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.809047] NetLabel:  unlabeled traffic allowed by default
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.813172] amd_nb: Cannot enumerate AMD northbridges
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.816602] clocksource: Switched to clocksource kvm-clock
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.827525] pnp: PnP ACPI init
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.830947] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.831028] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.831075] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.831123] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.831162] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.831233] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.831277] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.831437] pnp: PnP ACPI: found 7 devices
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.844183] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.851639] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.851642] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.851644] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.851645] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.851687] NET: Registered protocol family 2
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.855483] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.861139] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.867018] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.871486] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.876682] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.882277] NET: Registered protocol family 1
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.885711] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.891639] PCI: CLS 0 bytes, default 64
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    1.891711] Unpacking initramfs...
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.017861] Freeing initrd memory: 21432K
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.021257] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.025834] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.032240] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.038188] hw unit of domain pp0-core 2^-0 Joules
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.041907] hw unit of domain package 2^-0 Joules
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.045040] hw unit of domain dram 2^-16 Joules
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.047801] Scanning for low memory corruption every 60 seconds
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.052869] audit: initializing netlink subsys (disabled)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.057663] audit: type=2000 audit(1533629713.661:1): initialized
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.062138] Initialise system trusted keyring
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.065947] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.071105] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.077610] zbud: loaded
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.079969] VFS: Disk quotas dquot_6.6.0
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.082691] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.088422] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.093390] fuse init (API version 7.23)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.097185] Key type big_key registered
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.100427] Allocating IMA MOK and blacklist keyrings.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.110820] Key type asymmetric registered
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.114989] Asymmetric key parser 'x509' registered
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.118860] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.127617] io scheduler noop registered
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.130603] io scheduler deadline registered (default)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.134880] io scheduler cfq registered
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.137536] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.141863] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.146512] intel_idle: does not run on family 6 model 63
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.146620] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.151550] ACPI: Power Button [PWRF]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.153824] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.159119] ACPI: Sleep Button [SLPF]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.162188] GHES: HEST is not enabled!
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.167667] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.171923] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.185652] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.191622] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.206634] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.233495] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.261499] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.289031] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.318562] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.327468] Linux agpgart interface v0.103
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.337073] loop: module loaded
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.340141] libphy: Fixed MDIO Bus: probed
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.343776] tun: Universal TUN/TAP device driver, 1.6
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.347537] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.407046] PPP generic driver version 2.4.2
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.408852] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.411809] ehci-pci: EHCI PCI platform driver
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.413514] ehci-platform: EHCI generic platform driver
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.415620] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.418300] ohci-pci: OHCI PCI platform driver
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.419675] ohci-platform: OHCI generic platform driver
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.421062] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.422867] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.426010] i8042: Warning: Keylock active
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.428137] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.429427] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.430989] mousedev: PS/2 mouse device common for all mice
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.433261] rtc_cmos 00:00: RTC can wake from S4
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.434958] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.437153] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.439318] i2c /dev entries driver
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.440245] device-mapper: uevent: version 1.0.3
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.441546] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.443782] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.446229] NET: Registered protocol family 10
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.447775] NET: Registered protocol family 17
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.449112] Key type dns_resolver registered
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.451125] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.453280] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.454897] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.455875] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.457580] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.461199] registered taskstats version 1
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.462762] Loading compiled-in X.509 certificates
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.464950] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.467589] zswap: loaded using pool lzo/zbud
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.470955] Key type trusted registered
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.475996] Key type encrypted registered
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.476666] ima: No TPM chip found, activating TPM-bypass!
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.477548] evm: HMAC attrs: 0x1
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.478420]   Magic number: 14:104:270
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.479155] rtc_cmos 00:00: setting system clock to 2018-08-07 08:15:14 UTC (1533629714)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.480865] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.481604] EDD information not available.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.482560] PM: Hibernation image not present or could not be loaded.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.484056] Freeing unused kernel memory: 1496K
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.484817] Write protecting the kernel read-only data: 14336k
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.487094] Freeing unused kernel memory: 1956K
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.488625] Freeing unused kernel memory: 92K
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.503089] systemd-udevd[120]: starting version 204
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.566713] scsi host0: Virtio SCSI HBA
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.571076] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.574919] AVX2 version of gcm_enc/dec engaged.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.576139] AES CTR mode by8 optimization enabled
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.623471] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.623650] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.623651] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.624107] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.624109] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.624167] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.626731]  sda: sda1
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.627878] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    4.633339] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    5.044756] tsc: Refined TSC clocksource calibration: 2300.000 MHz
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    5.048895] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735223b2, max_idle_ns: 440795277976 ns
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    5.468314] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    7.552883] floppy0: no floppy controllers found
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    8.724616] raid6: sse2x1   gen()  8746 MB/s
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    8.792622] raid6: sse2x1   xor()  6525 MB/s
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    8.860619] raid6: sse2x2   gen() 10585 MB/s
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    8.928641] raid6: sse2x2   xor()  7123 MB/s
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.000622] raid6: sse2x4   gen() 12007 MB/s
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.068616] raid6: sse2x4   xor()  8633 MB/s
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.136620] raid6: avx2x1   gen() 16812 MB/s
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.204628] raid6: avx2x2   gen() 19690 MB/s
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.272615] raid6: avx2x4   gen() 22331 MB/s
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.275889] raid6: using algorithm avx2x4 gen() 22331 MB/s
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.279505] raid6: using avx2x2 recovery algorithm
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.284925] xor: automatically using best checksumming function:
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.328731]    avx       : 26965.000 MB/sec
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.346715] Btrfs loaded
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.409257] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.414549] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.521802] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.535115] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.539083] EXT4-fs (sda1): recovery complete
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.550676] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [    9.873011] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   10.059157] random: mountall: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   10.123591] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   10.368383] random: cloud-init: uninitialized urandom read (32 bytes read, 33 bits of entropy available)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   10.942183] random: cloud-init: uninitialized urandom read (32 bytes read, 41 bits of entropy available)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   11.078628] systemd-udevd[702]: starting version 204
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   11.198902] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   11.248517] intel_rapl: no valid rapl domains found in package 0
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   11.306449] ppdev: user-space parallel port driver
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   11.393722] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   11.445745] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   11.507075] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   11.671703] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   11.926467] random: mktemp: uninitialized urandom read (12 bytes read, 56 bits of entropy available)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   12.001593] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   12.086378] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   12.123095] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   12.486518] init: failsafe main process (1096) killed by TERM signal
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO Running set_multiqueue.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO Set channels for eth0 to 4.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 08:15:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 08:15:23 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  7 08:15:23 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 08:15:23 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 08:15:23 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Starting Google Accounts daemon.
Aug  7 08:15:23 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Creating a new user account for me.
Aug  7 08:15:23 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   13.337584] random: nonblocking pool is initialized
Aug  7 08:15:23 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Created user account me.
Aug  7 08:15:23 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Creating a new user account for henrik.
Aug  7 08:15:23 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Created user account henrik.
Aug  7 08:15:23 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Creating a new user account for emma.
Aug  7 08:15:23 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Created user account emma.
Aug  7 08:15:23 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Creating a new user account for igor.
Aug  7 08:15:23 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Created user account igor.
Aug  7 08:15:23 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Created user account konstantinhaase.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Creating a new user account for aj.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Created user account aj.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Creating a new user account for solarce.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Created user account solarce.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Creating a new user account for asari.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea cron[1468]: (CRON) INFO (pidfile fd = 3)
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Created user account asari.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea cron[1511]: (CRON) STARTUP (fork ok)
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea cron[1511]: (CRON) INFO (Running @reboot jobs)
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Creating a new user account for bogdana.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea acpid: starting up with netlink and the input layer
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea acpid: 1 rule loaded
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea acpid: waiting for events: event logging is off
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Created user account bogdana.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Creating a new user account for konstantin.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Created user account konstantin.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   13.976681] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   13.980440] Bridge firewalling registered
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   13.991345] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Creating a new user account for carmen.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   14.024779] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Created user account carmen.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Creating a new user account for maria.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Created user account maria.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   14.096116] Initializing XFRM netlink socket
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   14.104591] Netfilter messages via NETLINK v0.30.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   14.106994] ctnetlink v0.93: registering with nfnetlink.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea google-accounts: INFO Removing user packer.
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   14.248712] floppy0: no floppy controllers found
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea haveged: haveged starting up
Aug  7 08:15:24 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   14.593748] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 08:15:47 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpdate[1845]: adjust time server 169.254.169.254 offset 0.001806 sec
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpd[1878]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpd[1879]: proto: precision = 0.107 usec
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpd[1879]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpd[1879]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpd[1879]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpd[1879]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpd[1879]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpd[1879]: Listen normally on 3 eth0 10.20.1.111 UDP 123
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpd[1879]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpd[1879]: peers refreshed
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpd[1879]: Listening on routing socket on fd #21 for interface updates
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   44.815721] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea startup-script: INFO Found startup-script in metadata.
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea startup-script: INFO startup-script: job 1 at Tue Aug  7 11:25:00 2018
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea startup-script: INFO startup-script: Return code 0.
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea startup-script: INFO startup-script: Return code 0.
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea startup-script: INFO Finished running startup scripts.
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ec2: 
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ec2: #############################################################
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ec2: 1024 af:1e:0e:2f:2c:21:d4:e1:ae:91:fe:30:26:60:a2:1c  root@travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea (DSA)
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ec2: 256 50:9f:d1:05:1a:4f:fb:d7:ee:86:bb:81:05:4b:be:2b  root@travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea (ECDSA)
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ec2: 256 9b:7b:54:ec:22:cf:ee:03:e5:66:15:29:9f:94:28:b6  root@travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea (ED25519)
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ec2: 2048 d5:34:7c:8b:7d:0e:e5:61:8b:cb:2b:42:a2:ed:54:a8  root@travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea (RSA)
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 08:15:55 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ec2: #############################################################
Aug  7 08:16:25 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [   75.291222] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 08:17:01 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea CRON[4753]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  7 08:17:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [  131.844216] device veth2678179 entered promiscuous mode
Aug  7 08:17:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [  131.975271] cgroup: docker-runc (4856) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 08:17:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [  131.975274] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 08:17:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [  132.058782] eth0: renamed from veth03c248b
Aug  7 08:17:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [  132.103689] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 08:17:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [  132.104912] docker0: port 1(veth2678179) entered forwarding state
Aug  7 08:17:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [  132.104940] docker0: port 1(veth2678179) entered forwarding state
Aug  7 08:17:22 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [  132.104963] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 08:17:26 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpd[1879]: Listen normally on 5 docker0 fe80::42:3bff:fe91:5258 UDP 123
Aug  7 08:17:26 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpd[1879]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  7 08:17:26 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpd[1879]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 08:17:26 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpd[1879]: peers refreshed
Aug  7 08:17:26 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea ntpd[1879]: new interface(s) found: waking up resolver
Aug  7 08:17:37 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [  147.142510] docker0: port 1(veth2678179) entered forwarding state
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea dbus[1158]: [system] Activating service name='org.freedesktop.systemd1' (using servicehelper)
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea dbus[1158]: [system] Successfully activated service 'org.freedesktop.systemd1'
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.238189] init: tty4 main process (1458) killed by TERM signal
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.238338] init: tty5 main process (1465) killed by TERM signal
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.238455] init: tty2 main process (1470) killed by TERM signal
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.238651] init: tty3 main process (1471) killed by TERM signal
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.238803] init: tty6 main process (1473) killed by TERM signal
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.239009] init: cron main process (1511) killed by TERM signal
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.239228] init: irqbalance main process (1526) killed by TERM signal
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.239325] init: tty1 main process (1918) killed by TERM signal
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.239462] init: ttyS0 main process (1927) killed by TERM signal
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.239827] init: plymouth-upstart-bridge main process (6282) terminated with status 1
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.239834] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.307243] init: plymouth-upstart-bridge main process (6320) terminated with status 1
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.307255] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.331996] veth03c248b: renamed from eth0
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.332534] init: plymouth-upstart-bridge main process (6334) terminated with status 1
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.332544] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.345463] init: plymouth-upstart-bridge main process (6337) terminated with status 1
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.345474] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.355346] init: plymouth-upstart-bridge main process (6339) terminated with status 1
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.355357] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.368136] init: apport post-stop process (6275) terminated with status 1
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.368378] init: plymouth-upstart-bridge main process (6343) terminated with status 1
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.368387] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea shutdown-script: INFO Starting shutdown scripts.
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.381228] init: plymouth-upstart-bridge main process (6352) terminated with status 1
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.381238] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea shutdown-script: INFO No shutdown scripts found in metadata.
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea shutdown-script: INFO Finished running shutdown scripts.
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.389021] docker0: port 1(veth2678179) entered disabled state
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.391134] init: plymouth-upstart-bridge main process (6355) terminated with status 1
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.391145] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.400679] init: plymouth-upstart-bridge main process (6361) terminated with status 1
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.400691] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.407756] init: plymouth-upstart-bridge main process (6365) terminated with status 1
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.407766] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.419380] docker0: port 1(veth2678179) entered disabled state
Aug  7 08:48:12 travis-job-6206c06f-11aa-4f36-8879-ddcb489d03ea kernel: [ 1982.421699] device veth2678179 left promiscuous mode
