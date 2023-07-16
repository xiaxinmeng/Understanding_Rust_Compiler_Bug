plain
[01:39:10] Initialized empty Git repository in /checkout/obj/build/ct/webrender/.git/
[01:39:10] fatal: Could not parse object '57250b2b8fa63934f80e5376a29f7dcb3f759ad6'.
[01:40:06] fatal: unable to access 'https://github.com/servo/webrender/': Could not resolve host: github.com
[01:40:06] 
[01:40:06] thread 'main' panicked at 'assertion failed: status.success()', tools/cargotest/main.rs:128:13
[01:40:06] 
[01:40:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[01:40:06] expected success, got: exit code: 101
[01:40:06] 
[01:40:06] 
[01:40:06] 
[01:40:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:40:06] Build completed unsuccessfully in 0:35:04
[01:40:06] Makefile:60: recipe for target 'check-aux' failed
[01:40:06] make: *** [check-aux] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f0d51f4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:11d1a35c
$ sudo tail -n 500 /var/log/syslog
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] Policy zone: Normal
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] console [ttyS0] enabled
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.652785] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.659309] pid_max: default: 32768 minimum: 301
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.661885] ACPI: Core revision 20150930
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.669908] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.674706] Security Framework initialized
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.677652] Yama: becoming mindful.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.680566] AppArmor: AppArmor disabled by boot time parameter
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.685287] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.697501] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.705529] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.709379] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.713362] Initializing cgroup subsys io
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.715639] Initializing cgroup subsys memory
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.719060] Initializing cgroup subsys devices
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.722377] Initializing cgroup subsys freezer
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.725064] Initializing cgroup subsys net_cls
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.728533] Initializing cgroup subsys perf_event
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.731854] Initializing cgroup subsys net_prio
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.734667] Initializing cgroup subsys hugetlb
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.736994] Initializing cgroup subsys pids
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.739476] CPU: Physical Processor ID: 0
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.741937] CPU: Processor Core ID: 0
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.744631] mce: CPU supports 32 MCE banks
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.747179] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.750511] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.756281] Freeing SMP alternatives memory: 32K
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.769163] ftrace: allocating 32185 entries in 126 pages
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.826714] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.831223] smpboot: Max logical packages: 2
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.834283] x2apic enabled
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.836798] Switched APIC routing to physical x2apic.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.843294] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.953640] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.957795] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.961472] x86: Booting SMP configuration:
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.963370] .... node  #0, CPUs:      #1
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.964912] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.969923]  #2
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.971024] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.975828]  #3
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.976678] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.981506] x86: Booted up 1 node, 4 CPUs
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.982637] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.986295] devtmpfs: initialized
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.991430] evm: security.selinux
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.992782] evm: security.SMACK64
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.993933] evm: security.SMACK64EXEC
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.995413] evm: security.SMACK64TRANSMUTE
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.997218] evm: security.SMACK64MMAP
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.998526] evm: security.ima
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    0.999749] evm: security.capability
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.001376] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.004393] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.006554] pinctrl core: initialized pinctrl subsystem
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.008287] RTC time:  5:11:00, date: 08/15/18
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.010647] NET: Registered protocol family 16
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.021644] cpuidle: using governor ladder
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.033644] cpuidle: using governor menu
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.034307] PCCT header not found.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.034866] ACPI: bus type PCI registered
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.035685] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.036764] PCI: Using configuration type 1 for base access
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.050677] ACPI: Added _OSI(Module Device)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.051392] ACPI: Added _OSI(Processor Device)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.052040] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.052684] ACPI: Added _OSI(Processor Aggregator Device)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.055885] ACPI: Executed 2 blocks of module-level executable AML code
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.079114] ACPI: Interpreter enabled
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.079986] ACPI: (supports S0 S3 S4 S5)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.080640] ACPI: Using IOAPIC for interrupt routing
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.081472] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.110570] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.111827] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.112820] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.114208] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.116959] PCI host bridge to bus 0000:00
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.117688] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.119020] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.120169] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.121920] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.123454] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.124503] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.124974] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.140275] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.156118] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.157670] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.163083] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.167843] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.180828] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.187230] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.192066] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.206235] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.208801] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.211380] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.214444] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.216828] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.236874] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.238306] vgaarb: loaded
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.238941] SCSI subsystem initialized
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.239634] libata version 3.00 loaded.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.239655] ACPI: bus type USB registered
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.240247] usbcore: registered new interface driver usbfs
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.241140] usbcore: registered new interface driver hub
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.242173] usbcore: registered new device driver usb
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.243148] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.244322] dmi: Firmware registration failed.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.245342] PCI: Using ACPI for IRQ routing
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.246015] PCI: pci_cache_line_size set to 64 bytes
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.246111] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.246113] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.246226] NetLabel: Initializing
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.246722] NetLabel:  domain hash size = 128
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.247481] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.248288] NetLabel:  unlabeled traffic allowed by default
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.249429] amd_nb: Cannot enumerate AMD northbridges
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.250330] clocksource: Switched to clocksource kvm-clock
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.257450] pnp: PnP ACPI init
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.258017] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.258084] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.258128] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.258178] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.258220] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.258260] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.258301] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.258475] pnp: PnP ACPI: found 7 devices
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.265797] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.267455] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.267457] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.267459] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.267460] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.267499] NET: Registered protocol family 2
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.268458] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.270813] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.271975] TCP: Hash tables configured (established 131072 bind 65536)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.273202] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.274378] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.276279] NET: Registered protocol family 1
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.277181] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.278515] PCI: CLS 0 bytes, default 64
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    1.278567] Unpacking initramfs...
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.288059] Freeing initrd memory: 21432K
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.288810] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.289677] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.291737] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.293199] hw unit of domain pp0-core 2^-0 Joules
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.294062] hw unit of domain package 2^-0 Joules
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.294811] hw unit of domain dram 2^-0 Joules
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.295665] Scanning for low memory corruption every 60 seconds
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.297870] audit: initializing netlink subsys (disabled)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.298755] audit: type=2000 audit(1534309862.219:1): initialized
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.300220] Initialise system trusted keyring
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.301166] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.302320] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.304812] zbud: loaded
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.305529] VFS: Disk quotas dquot_6.6.0
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.306230] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.307487] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.308775] fuse init (API version 7.23)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.309809] Key type big_key registered
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.310823] Allocating IMA MOK and blacklist keyrings.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.313788] Key type asymmetric registered
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.314682] Asymmetric key parser 'x509' registered
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.315745] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.317561] io scheduler noop registered
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.318319] io scheduler deadline registered (default)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.320478] io scheduler cfq registered
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.321446] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.322904] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.323883] intel_idle: does not run on family 6 model 45
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.323979] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.325306] ACPI: Power Button [PWRF]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.325914] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.327401] ACPI: Sleep Button [SLPF]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.328692] GHES: HEST is not enabled!
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.331405] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.333105] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.337960] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.339027] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.344889] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.367747] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.391297] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.414512] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.437920] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.441412] Linux agpgart interface v0.103
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.444846] loop: module loaded
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.446177] libphy: Fixed MDIO Bus: probed
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.447975] tun: Universal TUN/TAP device driver, 1.6
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.449442] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.485477] PPP generic driver version 2.4.2
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.486570] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.488715] ehci-pci: EHCI PCI platform driver
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.490355] ehci-platform: EHCI generic platform driver
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.492425] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.494443] ohci-pci: OHCI PCI platform driver
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.495818] ohci-platform: OHCI generic platform driver
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.498143] uhci_hcd: USB Universal Host Controller Interface driver
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.499900] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.503000] i8042: Warning: Keylock active
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.505269] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.506790] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.508518] mousedev: PS/2 mouse device common for all mice
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.510563] rtc_cmos 00:00: RTC can wake from S4
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.512183] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.514063] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.515996] i2c /dev entries driver
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.517097] device-mapper: uevent: version 1.0.3
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.518761] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.521140] ledtrig-cpu: registered to indicate activity on CPUs
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.523691] NET: Registered protocol family 10
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.524842] NET: Registered protocol family 17
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.526181] Key type dns_resolver registered
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.527775] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.529723] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.531231] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.532666] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.534072] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.536752] registered taskstats version 1
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.537925] Loading compiled-in X.509 certificates
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.539954] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.542971] zswap: loaded using pool lzo/zbud
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.546546] Key type trusted registered
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.551203] Key type encrypted registered
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.552100] ima: No TPM chip found, activating TPM-bypass!
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.553477] evm: HMAC attrs: 0x1
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.554635]   Magic number: 14:283:163
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.555571] rtc_cmos 00:00: setting system clock to 2018-08-15 05:11:03 UTC (1534309863)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.557541] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.559225] EDD information not available.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.560492] PM: Hibernation image not present or could not be loaded.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.561976] Freeing unused kernel memory: 1496K
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.562812] Write protecting the kernel read-only data: 14336k
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.564777] Freeing unused kernel memory: 1956K
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.565865] Freeing unused kernel memory: 92K
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.579376] systemd-udevd[118]: starting version 204
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.633909] AVX version of gcm_enc/dec engaged.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.634696] AES CTR mode by8 optimization enabled
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.639486] scsi host0: Virtio SCSI HBA
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.642852] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.679473] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.679494] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.681728] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.683157] sd 0:0:1:0: [sda] Write Protect is off
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.684035] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.684135] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.686835]  sda: sda1
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.687999] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    3.710686] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    4.294456] tsc: Refined TSC clocksource calibration: 2600.001 MHz
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    4.295909] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3ce1c4c, max_idle_ns: 440795206275 ns
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    4.543442] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    6.626627] floppy0: no floppy controllers found
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    7.802391] raid6: sse2x1   gen()  8579 MB/s
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    7.870393] raid6: sse2x1   xor()  6499 MB/s
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    7.942395] raid6: sse2x2   gen() 10580 MB/s
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.010417] raid6: sse2x2   xor()  7113 MB/s
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.078407] raid6: sse2x4   gen() 12378 MB/s
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.146396] raid6: sse2x4   xor()  8571 MB/s
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.149146] raid6: using algorithm sse2x4 gen() 12378 MB/s
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.152665] raid6: .... xor() 8571 MB/s, rmw enabled
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.156452] raid6: using ssse3x2 recovery algorithm
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.161626] xor: automatically using best checksumming function:
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.202390]    avx       : 26719.000 MB/sec
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.220001] Btrfs loaded
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.291958] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.296811] EXT4-fs (sda1): write access will be enabled during recovery
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.397077] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.408997] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.413337] EXT4-fs (sda1): recovery complete
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.423087] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.671325] random: init: uninitialized urandom read (12 bytes read, 23 bits of entropy available)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.814774] random: mountall: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    8.876207] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    9.091195] random: cloud-init: uninitialized urandom read (32 bytes read, 33 bits of entropy available)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    9.717455] random: cloud-init: uninitialized urandom read (32 bytes read, 40 bits of entropy available)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    9.863278] systemd-udevd[703]: starting version 204
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [    9.990761] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   10.061414] intel_rapl: no valid rapl domains found in package 0
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   10.109781] ppdev: user-space parallel port driver
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   10.223515] random: mktemp: uninitialized urandom read (6 bytes read, 50 bits of entropy available)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   10.283031] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   10.360352] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   10.539878] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   10.845840] random: mktemp: uninitialized urandom read (12 bytes read, 54 bits of entropy available)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   10.930654] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   11.016755] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   11.073634] EXT4-fs (sda1): resized filesystem to 7864064
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   11.577173] init: failsafe main process (1094) killed by TERM signal
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO Running set_multiqueue.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO Set channels for eth0 to 4.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 15 05:11:11 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Starting Google Accounts daemon.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Creating a new user account for me.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Created user account me.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-clock-skew: INFO Synced system time with hardware clock.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Creating a new user account for henrik.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Created user account henrik.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Creating a new user account for emma.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Created user account emma.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Creating a new user account for igor.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Created user account igor.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Created user account konstantinhaase.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Creating a new user account for aj.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Created user account aj.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Creating a new user account for solarce.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Created user account solarce.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Creating a new user account for asari.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   13.012131] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   13.016622] Bridge firewalling registered
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   13.029755] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Created user account asari.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Creating a new user account for bogdana.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   13.065232] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   13.074651] floppy0: no floppy controllers found
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Created user account bogdana.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Creating a new user account for konstantin.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   13.161324] random: nonblocking pool is initialized
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   13.174942] Initializing XFRM netlink socket
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   13.184761] Netfilter messages via NETLINK v0.30.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   13.188259] ctnetlink v0.93: registering with nfnetlink.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Created user account konstantin.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Creating a new user account for carmen.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Created user account carmen.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Creating a new user account for maria.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Created user account maria.
Aug 15 05:11:12 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 google-accounts: INFO Removing user packer.
Aug 15 05:11:15 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 05:11:15 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 05:11:15 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 cron[1709]: (CRON) INFO (pidfile fd = 3)
Aug 15 05:11:15 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 cron[1746]: (CRON) STARTUP (fork ok)
Aug 15 05:11:15 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 cron[1746]: (CRON) INFO (Running @reboot jobs)
Aug 15 05:11:15 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 acpid: starting up with netlink and the input layer
Aug 15 05:11:15 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 acpid: 1 rule loaded
Aug 15 05:11:15 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 acpid: waiting for events: event logging is off
Aug 15 05:11:15 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 haveged: haveged starting up
Aug 15 05:11:15 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   16.071872] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1843]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1844]: proto: precision = 0.111 usec
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1844]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1844]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1844]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1844]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1844]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1844]: Listen normally on 3 eth0 10.20.1.27 UDP 123
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1844]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1844]: peers refreshed
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1844]: Listening on routing socket on fd #21 for interface updates
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   21.270240] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 startup-script: INFO Found startup-script in metadata.
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 startup-script: INFO startup-script: job 1 at Wed Aug 15 08:21:00 2018
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 startup-script: INFO startup-script: Return code 0.
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 startup-script: INFO startup-script: Return code 0.
Aug 15 05:11:20 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 startup-script: INFO Finished running startup scripts.
Aug 15 05:11:21 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ec2: 
Aug 15 05:11:21 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ec2: #############################################################
Aug 15 05:11:21 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 15 05:11:21 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ec2: 1024 52:e8:57:7d:4b:d5:e6:ec:ee:78:af:fa:b7:8e:33:da  root@travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 (DSA)
Aug 15 05:11:21 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ec2: 256 24:ec:07:fd:4e:45:fd:c5:32:58:07:bb:4d:ea:9a:9d  root@travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 (ECDSA)
Aug 15 05:11:21 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ec2: 256 86:d5:f3:0e:b5:6f:cb:b9:99:0e:6d:64:63:e2:81:c5  root@travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 (ED25519)
Aug 15 05:11:21 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ec2: 2048 39:65:08:a1:69:eb:aa:ba:e1:76:19:58:ce:2a:d9:22  root@travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 (RSA)
Aug 15 05:11:21 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 15 05:11:21 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ec2: #############################################################
Aug 15 05:11:27 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpdate[2248]: the NTP socket is in use, exiting
Aug 15 05:12:06 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [   66.574431] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 15 05:13:48 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [  169.214166] device veth5f1ca62 entered promiscuous mode
Aug 15 05:13:48 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [  169.214252] docker0: port 1(veth5f1ca62) entered forwarding state
Aug 15 05:13:48 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [  169.214261] docker0: port 1(veth5f1ca62) entered forwarding state
Aug 15 05:13:48 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [  169.214731] docker0: port 1(veth5f1ca62) entered disabled state
Aug 15 05:13:48 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [  169.295743] cgroup: docker-runc (4885) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 15 05:13:48 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [  169.295746] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 15 05:13:48 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [  169.367788] eth0: renamed from vethc4b92a5
Aug 15 05:13:48 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [  169.401619] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 15 05:13:48 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [  169.402608] docker0: port 1(veth5f1ca62) entered forwarding state
Aug 15 05:13:48 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [  169.402626] docker0: port 1(veth5f1ca62) entered forwarding state
Aug 15 05:13:48 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [  169.402649] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 15 05:13:52 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1844]: Listen normally on 5 docker0 fe80::42:1ff:fe2d:f936 UDP 123
Aug 15 05:13:52 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1844]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 15 05:13:52 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1844]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 15 05:13:52 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1844]: peers refreshed
Aug 15 05:13:52 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1844]: new interface(s) found: waking up resolver
Aug 15 05:14:03 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [  184.426692] docker0: port 1(veth5f1ca62) entered forwarding state
Aug 15 05:17:01 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 CRON[6797]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 15 06:17:01 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 CRON[8313]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 15 06:18:23 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [ 4043.870825] traps: a[9337] trap invalid opcode ip:55a176012b0b sp:7ffcb12ab990 error:0 in a[55a17600f000+6000]
Aug 15 06:18:44 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [ 4065.028330] traps: a[12171] trap invalid opcode ip:7f5ba2988ad1 sp:7ffc9f162410 error:0 in libstd-41f43a30bc296e4f.so[7f5ba292a000+166000]
Aug 15 06:18:44 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [ 4065.056428] traps: a[12172] trap invalid opcode ip:7f4a1994cad1 sp:7ffc69fa6860 error:0 in libstd-41f43a30bc296e4f.so[7f4a198ee000+166000]
Aug 15 06:20:44 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [ 4184.727969] traps: a[27046] trap invalid opcode ip:5597ae3a2e29 sp:7ffd482bd300 error:0 in a[5597ae3a0000+4000]
Aug 15 06:24:34 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [ 4414.991928] a[23140]: segfault at 0 ip 0000561dac2623ef sp 00007ffe2ca1c8c0 error 6 in a[561dac25f000+5000]
Aug 15 06:24:47 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [ 4427.801212] a[23885]: segfault at 1 ip 000055917a495bed sp 00007ffcdbbd0d50 error 6 in a[55917a493000+4000]
Aug 15 06:24:53 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [ 4434.208715] traps: a[24331] trap invalid opcode ip:5563c81ed4bc sp:7fff312db410 error:0 in a[5563c81ea000+7000]
Aug 15 06:25:01 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 CRON[25281]: (root) CMD (test -x /usr/sbin/anacron || ( cd / && run-parts --report /etc/cron.daily ))
Aug 15 06:43:37 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 upstart-socket-bridge[842]: Disconnected from Upstart
Aug 15 06:43:37 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 upstart-udev-bridge[697]: Disconnected from Upstart
Aug 15 06:43:38 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [ 5558.524815] init: upstart-udev-bridge main process (697) terminated with status 1
Aug 15 06:43:38 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [ 5558.524829] init: upstart-udev-bridge main process ended, respawning
Aug 15 06:43:38 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [ 5558.524952] init: upstart-socket-bridge main process (842) terminated with status 1
Aug 15 06:43:38 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [ 5558.524961] init: upstart-socket-bridge main process ended, respawning
Aug 15 06:43:38 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [ 5558.525064] init: upstart-file-bridge main process (1195) terminated with status 1
Aug 15 06:43:38 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [ 5558.525073] init: upstart-file-bridge main process ended, respawning
Aug 15 06:44:09 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 dbus[1155]: [system] Reloaded configuration
Aug 15 06:44:09 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 dbus[1155]: message repeated 10 times: [ [system] Reloaded configuration]
Aug 15 06:44:16 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[1844]: ntpd exiting on signal 15
Aug 15 06:44:33 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [ 5613.696404] init: apport post-stop process (20647) terminated with status 1
Aug 15 06:44:35 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 dbus[1155]: [system] Reloaded configuration
Aug 15 06:44:35 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 dbus[1155]: message repeated 5 times: [ [system] Reloaded configuration]
Aug 15 06:44:40 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 kernel: [ 5621.159185] systemd-udevd[22624]: starting version 204
Aug 15 06:44:40 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 dbus[1155]: [system] Reloaded configuration
Aug 15 06:44:40 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 dbus[1155]: message repeated 3 times: [ [system] Reloaded configuration]
Aug 15 06:44:42 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[22809]: ntpd 4.2.6p5@1.2349-o Fri Jul  6 20:19:54 UTC 2018 (1)
Aug 15 06:44:42 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[22810]: ntp_io: estimated max descriptors: 72000, initial socket boundary: 16
Aug 15 06:44:42 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[22810]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 15 06:44:42 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[22810]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 15 06:44:42 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[22810]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 15 06:44:42 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[22810]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 15 06:44:42 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[22810]: Listen normally on 3 eth0 10.20.1.27 UDP 123
Aug 15 06:44:42 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[22810]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 15 06:44:42 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[22810]: peers refreshed
Aug 15 06:44:42 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 ntpd[22810]: Listening on routing socket on fd #21 for interface updates
Aug 15 06:44:44 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 06:44:44 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 06:44:46 travis-job-054d663b-5c11-414a-be89-d596b3c1a5d3 dbus[1155]: [system] Reloaded configuration
---
travis_time:end:0079d0de:start=1534315942334371964,finish=1534315942342453642,duration=8081678
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:093cb8f8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00a2d380
travis_time:start:00a2d380
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:003fed3c
$ dmesg | grep -i kill
