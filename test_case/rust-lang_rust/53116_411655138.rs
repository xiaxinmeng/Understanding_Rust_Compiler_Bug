plain
[01:48:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[01:48:46] expected success, got: exit code: 101
[01:48:46] 
[01:48:46] 
[01:48:46] thread 'main' panicked at 'assertion failed: status.success()', tools/cargotest/main.rs:128:13
[01:48:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:48:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:48:46] Build completed unsuccessfully in 0:48:13
[01:48:46] make: *** [check-aux] Error 1
[01:48:46] Makefile:60: recipe for target 'check-aux' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:172f066e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:265579b0
$ sudo tail -n 500 /var/log/syslog
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] kvm-clock: using sched offset of 1573285819 cycles
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] Zone ranges:
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000]   Device   empty
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] Movable zone start for each node
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] Early memory node ranges
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] Policy zone: Normal
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] Hierarchical RCU implementation.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] console [ttyS0] enabled
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.460559] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.464983] pid_max: default: 32768 minimum: 301
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.467125] ACPI: Core revision 20150930
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.475559] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.477961] Security Framework initialized
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.479571] Yama: becoming mindful.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.480859] AppArmor: AppArmor disabled by boot time parameter
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.484352] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.496773] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.503876] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.506851] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.510146] Initializing cgroup subsys io
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.511791] Initializing cgroup subsys memory
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.514069] Initializing cgroup subsys devices
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.515889] Initializing cgroup subsys freezer
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.517321] Initializing cgroup subsys net_cls
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.519396] Initializing cgroup subsys perf_event
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.521204] Initializing cgroup subsys net_prio
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.523583] Initializing cgroup subsys hugetlb
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.525497] Initializing cgroup subsys pids
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.527868] CPU: Physical Processor ID: 0
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.529030] CPU: Processor Core ID: 0
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.531607] mce: CPU supports 32 MCE banks
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.533531] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.535535] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.540567] Freeing SMP alternatives memory: 32K
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.551117] ftrace: allocating 32185 entries in 126 pages
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.605918] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.609213] smpboot: Max logical packages: 2
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.611823] x2apic enabled
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.614824] Switched APIC routing to physical x2apic.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.621141] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.731035] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.736273] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.742292] x86: Booting SMP configuration:
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.744168] .... node  #0, CPUs:      #1
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.746381] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.752716]  #2
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.753537] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.760188]  #3
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.761277] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.767721] x86: Booted up 1 node, 4 CPUs
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.769958] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.775623] devtmpfs: initialized
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.781383] evm: security.selinux
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.783465] evm: security.SMACK64
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.785343] evm: security.SMACK64EXEC
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.786821] evm: security.SMACK64TRANSMUTE
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.788955] evm: security.SMACK64MMAP
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.790959] evm: security.ima
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.792812] evm: security.capability
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.795425] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.800176] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.803368] pinctrl core: initialized pinctrl subsystem
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.806851] RTC time:  4:56:57, date: 08/09/18
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.810160] NET: Registered protocol family 16
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.823127] cpuidle: using governor ladder
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.835159] cpuidle: using governor menu
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.838617] PCCT header not found.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.840434] ACPI: bus type PCI registered
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.842137] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.845255] PCI: Using configuration type 1 for base access
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.861180] ACPI: Added _OSI(Module Device)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.863746] ACPI: Added _OSI(Processor Device)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.866146] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.868822] ACPI: Added _OSI(Processor Aggregator Device)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.874194] ACPI: Executed 2 blocks of module-level executable AML code
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.901333] ACPI: Interpreter enabled
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.903759] ACPI: (supports S0 S3 S4 S5)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.906540] ACPI: Using IOAPIC for interrupt routing
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.909429] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.944741] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.949345] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.952723] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.956474] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.964238] PCI host bridge to bus 0000:00
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.966336] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.970019] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.973563] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.977120] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.981178] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.984110] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    0.984597] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.008998] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.037693] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.040941] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.051326] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.059156] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.088328] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.099427] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.107676] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.129535] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.135248] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.140734] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.146154] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.151518] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.175874] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.179053] vgaarb: loaded
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.180692] SCSI subsystem initialized
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.183495] libata version 3.00 loaded.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.183528] ACPI: bus type USB registered
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.185482] usbcore: registered new interface driver usbfs
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.187994] usbcore: registered new interface driver hub
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.190940] usbcore: registered new device driver usb
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.193941] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.197279] dmi: Firmware registration failed.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.199802] PCI: Using ACPI for IRQ routing
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.201722] PCI: pci_cache_line_size set to 64 bytes
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.201869] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.201871] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.202021] NetLabel: Initializing
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.203884] NetLabel:  domain hash size = 128
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.206491] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.209169] NetLabel:  unlabeled traffic allowed by default
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.211810] amd_nb: Cannot enumerate AMD northbridges
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.214410] clocksource: Switched to clocksource kvm-clock
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.224448] pnp: PnP ACPI init
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.226557] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.226629] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.226671] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.226721] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.226806] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.226845] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.226886] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.227054] pnp: PnP ACPI: found 7 devices
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.236617] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.241531] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.241534] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.241536] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.241537] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.241585] NET: Registered protocol family 2
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.245205] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.249513] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.253443] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.257299] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.260115] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.263852] NET: Registered protocol family 1
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.266140] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.269293] PCI: CLS 0 bytes, default 64
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    1.270140] Unpacking initramfs...
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.477155] Freeing initrd memory: 21432K
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.479878] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.483270] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.487571] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.492061] hw unit of domain pp0-core 2^-0 Joules
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.494813] hw unit of domain package 2^-0 Joules
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.497719] hw unit of domain dram 2^-0 Joules
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.499701] Scanning for low memory corruption every 60 seconds
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.503313] audit: initializing netlink subsys (disabled)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.505912] audit: type=2000 audit(1533790620.048:1): initialized
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.509221] Initialise system trusted keyring
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.512665] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.515395] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.519506] zbud: loaded
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.521316] VFS: Disk quotas dquot_6.6.0
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.523121] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.526697] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.530034] fuse init (API version 7.23)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.532803] Key type big_key registered
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.535736] Allocating IMA MOK and blacklist keyrings.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.543013] Key type asymmetric registered
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.544899] Asymmetric key parser 'x509' registered
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.547157] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.550935] io scheduler noop registered
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.553165] io scheduler deadline registered (default)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.557170] io scheduler cfq registered
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.559618] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.561970] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.566475] intel_idle: does not run on family 6 model 62
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.566625] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.571290] ACPI: Power Button [PWRF]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.573907] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.578956] ACPI: Sleep Button [SLPF]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.581658] GHES: HEST is not enabled!
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.586662] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.590195] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.601148] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.604898] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.616861] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.641915] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.668095] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.693186] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.720010] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.728424] Linux agpgart interface v0.103
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.734888] loop: module loaded
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.737196] libphy: Fixed MDIO Bus: probed
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.739012] tun: Universal TUN/TAP device driver, 1.6
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.741104] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.814720] PPP generic driver version 2.4.2
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.817254] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.820167] ehci-pci: EHCI PCI platform driver
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.822538] ehci-platform: EHCI generic platform driver
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.825467] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.828835] ohci-pci: OHCI PCI platform driver
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.830946] ohci-platform: OHCI generic platform driver
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.833172] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.835837] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.839435] i8042: Warning: Keylock active
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.842876] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.844724] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.847222] mousedev: PS/2 mouse device common for all mice
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.850610] rtc_cmos 00:00: RTC can wake from S4
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.852900] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.855573] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.858095] i2c /dev entries driver
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.859820] device-mapper: uevent: version 1.0.3
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.861750] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.865611] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.869269] NET: Registered protocol family 10
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.871746] NET: Registered protocol family 17
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.873791] Key type dns_resolver registered
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.876179] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.879622] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.882693] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.885608] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.888009] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.892700] registered taskstats version 1
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.894715] Loading compiled-in X.509 certificates
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.897762] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.902242] zswap: loaded using pool lzo/zbud
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.907009] Key type trusted registered
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.914940] Key type encrypted registered
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.917542] ima: No TPM chip found, activating TPM-bypass!
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.919993] evm: HMAC attrs: 0x1
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.922093]   Magic number: 14:1:919
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.924272] rtc_cmos 00:00: setting system clock to 2018-08-09 04:57:00 UTC (1533790620)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.927926] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.931080] EDD information not available.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.933676] PM: Hibernation image not present or could not be loaded.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.935305] Freeing unused kernel memory: 1496K
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.937345] Write protecting the kernel read-only data: 14336k
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.941476] Freeing unused kernel memory: 1956K
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.943922] Freeing unused kernel memory: 92K
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    3.964701] systemd-udevd[120]: starting version 204
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    4.049773] scsi host0: Virtio SCSI HBA
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    4.052053] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    4.066204] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    4.071083] AVX version of gcm_enc/dec engaged.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    4.073761] AES CTR mode by8 optimization enabled
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    4.163416] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    4.163580] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    4.163582] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    4.164234] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    4.164237] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    4.164336] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    4.168011]  sda: sda1
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    4.170327] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    4.498630] tsc: Refined TSC clocksource calibration: 2499.768 MHz
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    4.501553] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24085ee3cf2, max_idle_ns: 440795333760 ns
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    4.893263] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    7.026820] floppy0: no floppy controllers found
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.222418] raid6: sse2x1   gen()  8549 MB/s
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.290426] raid6: sse2x1   xor()  6631 MB/s
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.358423] raid6: sse2x2   gen() 10863 MB/s
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.426422] raid6: sse2x2   xor()  7632 MB/s
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.494420] raid6: sse2x4   gen() 12073 MB/s
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.562421] raid6: sse2x4   xor()  8146 MB/s
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.565230] raid6: using algorithm sse2x4 gen() 12073 MB/s
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.568989] raid6: .... xor() 8146 MB/s, rmw enabled
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.571395] raid6: using ssse3x2 recovery algorithm
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.575784] xor: automatically using best checksumming function:
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.618496]    avx       : 21757.000 MB/sec
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.634823] Btrfs loaded
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.697628] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.700504] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.781471] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.798866] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.801350] EXT4-fs (sda1): recovery complete
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    8.809624] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    9.026313] random: init: uninitialized urandom read (12 bytes read, 22 bits of entropy available)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    9.149166] random: mountall: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    9.203700] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    9.394739] random: cloud-init: uninitialized urandom read (32 bytes read, 31 bits of entropy available)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [    9.936539] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   10.089009] systemd-udevd[701]: starting version 204
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   10.196876] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   10.302107] ppdev: user-space parallel port driver
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   10.412587] random: mktemp: uninitialized urandom read (6 bytes read, 48 bits of entropy available)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   10.473817] random: mktemp: uninitialized urandom read (6 bytes read, 49 bits of entropy available)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   10.552549] random: cloud-init: uninitialized urandom read (32 bytes read, 49 bits of entropy available)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   10.727544] random: cloud-init: uninitialized urandom read (32 bytes read, 49 bits of entropy available)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   11.002156] random: mktemp: uninitialized urandom read (12 bytes read, 52 bits of entropy available)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   11.071065] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   11.151580] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   11.193063] EXT4-fs (sda1): resized filesystem to 7864064
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   11.677612] init: failsafe main process (1092) killed by TERM signal
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO Running set_multiqueue.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO Set channels for eth0 to 4.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f google-accounts: INFO Starting Google Accounts daemon.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 04:57:08 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f google-accounts: INFO Creating a new user account for me.
Aug  9 04:57:09 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  9 04:57:09 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f google-accounts: INFO Created user account me.
Aug  9 04:57:09 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f google-accounts: INFO Creating a new user account for bogdana.
Aug  9 04:57:09 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f google-accounts: INFO Created user account bogdana.
Aug  9 04:57:09 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f google-accounts: INFO Creating a new user account for aj.
Aug  9 04:57:09 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f google-accounts: INFO Created user account aj.
Aug  9 04:57:09 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f google-accounts: INFO Creating a new user account for asari.
Aug  9 04:57:09 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f google-accounts: INFO Created user account asari.
Aug  9 04:57:09 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f google-accounts: INFO Removing user packer.
Aug  9 04:57:10 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 04:57:10 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   13.089898] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  9 04:57:10 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   13.095161] Bridge firewalling registered
Aug  9 04:57:10 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   13.107211] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 04:57:10 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   13.142231] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 04:57:10 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   13.191498] random: nonblocking pool is initialized
Aug  9 04:57:10 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   13.228741] Initializing XFRM netlink socket
Aug  9 04:57:10 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   13.238580] Netfilter messages via NETLINK v0.30.
Aug  9 04:57:10 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   13.241412] ctnetlink v0.93: registering with nfnetlink.
Aug  9 04:57:10 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   13.262508] floppy0: no floppy controllers found
Aug  9 04:57:10 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   13.262830] work still pending
Aug  9 04:57:10 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 04:57:10 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 04:57:11 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 04:57:11 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f cron[1628]: (CRON) INFO (pidfile fd = 3)
Aug  9 04:57:11 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f cron[1664]: (CRON) STARTUP (fork ok)
Aug  9 04:57:11 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f cron[1664]: (CRON) INFO (Running @reboot jobs)
Aug  9 04:57:11 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 04:57:11 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f acpid: starting up with netlink and the input layer
Aug  9 04:57:11 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f acpid: 1 rule loaded
Aug  9 04:57:11 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f acpid: waiting for events: event logging is off
Aug  9 04:57:11 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f haveged: haveged starting up
Aug  9 04:57:11 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   14.427016] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1762]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1763]: proto: precision = 0.101 usec
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1763]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1763]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1763]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1763]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1763]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1763]: Listen normally on 3 eth0 10.20.0.105 UDP 123
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1763]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1763]: peers refreshed
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1763]: Listening on routing socket on fd #21 for interface updates
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   19.602409] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f startup-script: INFO Found startup-script in metadata.
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f startup-script: INFO startup-script: job 1 at Thu Aug  9 08:07:00 2018
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f startup-script: INFO startup-script: Return code 0.
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f startup-script: INFO startup-script: Return code 0.
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f startup-script: INFO Finished running startup scripts.
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ec2: 
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ec2: #############################################################
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ec2: 1024 f9:c4:4f:ee:9a:62:cf:e0:03:42:40:87:93:f3:5c:ec  root@travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f (DSA)
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ec2: 256 8d:fa:eb:05:51:b0:dd:67:86:39:dd:56:0a:4a:24:c3  root@travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f (ECDSA)
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ec2: 256 fe:82:65:d2:b7:61:64:26:7d:20:cf:26:b9:70:90:58  root@travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f (ED25519)
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ec2: 2048 09:78:bc:67:a1:37:6e:f4:fa:05:57:a5:f4:85:84:1b  root@travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f (RSA)
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 04:57:16 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ec2: #############################################################
Aug  9 04:57:25 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpdate[1842]: the NTP socket is in use, exiting
Aug  9 04:57:54 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [   57.536238] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 04:59:02 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [  125.162992] device veth66eabac entered promiscuous mode
Aug  9 04:59:02 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [  125.760347] cgroup: docker-runc (4746) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 04:59:02 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [  125.760350] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  9 04:59:02 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [  125.827336] eth0: renamed from vethbe2bfc3
Aug  9 04:59:02 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [  125.873307] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  9 04:59:02 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [  125.874847] docker0: port 1(veth66eabac) entered forwarding state
Aug  9 04:59:02 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [  125.874878] docker0: port 1(veth66eabac) entered forwarding state
Aug  9 04:59:02 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [  125.874901] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  9 04:59:06 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1763]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  9 04:59:06 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1763]: Listen normally on 6 docker0 fe80::42:ecff:fe72:1503 UDP 123
Aug  9 04:59:06 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1763]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  9 04:59:06 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1763]: peers refreshed
Aug  9 04:59:06 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1763]: new interface(s) found: waking up resolver
Aug  9 04:59:17 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [  140.875108] docker0: port 1(veth66eabac) entered forwarding state
Aug  9 05:17:01 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f CRON[13051]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  9 05:59:38 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [ 3761.270307] traps: a[9115] trap invalid opcode ip:55f00483fbab sp:7ffe95f08150 error:0 in a[55f00483c000+6000]
Aug  9 05:59:59 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [ 3782.232222] traps: a[11973] trap invalid opcode ip:7f58e95288c1 sp:7ffe45883bd0 error:0 in libstd-41f43a30bc296e4f.so[7f58e94cf000+16a000]
Aug  9 05:59:59 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [ 3782.264280] traps: a[11974] trap invalid opcode ip:7f985696f8c1 sp:7ffcb105dee0 error:0 in libstd-41f43a30bc296e4f.so[7f9856916000+16a000]
Aug  9 06:01:54 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [ 3896.756500] traps: a[26869] trap invalid opcode ip:55d407f4be19 sp:7ffed204fff0 error:0 in a[55d407f49000+4000]
Aug  9 06:05:32 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [ 4114.991323] a[23044]: segfault at 0 ip 000055ea8e5243df sp 00007ffe10bc9e70 error 6 in a[55ea8e521000+5000]
Aug  9 06:05:44 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [ 4127.089087] a[23802]: segfault at 1 ip 0000564a26c03bdd sp 00007ffdf9239720 error 6 in a[564a26c01000+4000]
Aug  9 06:05:49 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [ 4132.340157] traps: a[24206] trap invalid opcode ip:562b706044bc sp:7ffefd0dabd0 error:0 in a[562b70601000+7000]
Aug  9 06:17:01 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f CRON[24663]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  9 06:25:01 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f CRON[11134]: (root) CMD (test -x /usr/sbin/anacron || ( cd / && run-parts --report /etc/cron.daily ))
Aug  9 06:44:35 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f upstart-socket-bridge[858]: Disconnected from Upstart
Aug  9 06:44:35 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f upstart-udev-bridge[696]: Disconnected from Upstart
Aug  9 06:44:35 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [ 6457.728214] init: upstart-udev-bridge main process (696) terminated with status 1
Aug  9 06:44:35 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [ 6457.728223] init: upstart-udev-bridge main process ended, respawning
Aug  9 06:44:35 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [ 6457.728311] init: upstart-socket-bridge main process (858) terminated with status 1
Aug  9 06:44:35 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [ 6457.728341] init: upstart-socket-bridge main process ended, respawning
Aug  9 06:44:35 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [ 6457.728436] init: upstart-file-bridge main process (1202) terminated with status 1
Aug  9 06:44:35 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [ 6457.728442] init: upstart-file-bridge main process ended, respawning
Aug  9 06:44:59 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f dbus[1144]: [system] Reloaded configuration
Aug  9 06:44:59 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f dbus[1144]: message repeated 8 times: [ [system] Reloaded configuration]
Aug  9 06:45:06 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[1763]: ntpd exiting on signal 15
Aug  9 06:45:23 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [ 6505.999718] init: apport post-stop process (10236) terminated with status 1
Aug  9 06:45:25 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f dbus[1144]: [system] Reloaded configuration
Aug  9 06:45:25 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f dbus[1144]: message repeated 6 times: [ [system] Reloaded configuration]
Aug  9 06:45:32 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f kernel: [ 6514.855723] systemd-udevd[12420]: starting version 204
Aug  9 06:45:32 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f dbus[1144]: [system] Reloaded configuration
Aug  9 06:45:32 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f dbus[1144]: message repeated 3 times: [ [system] Reloaded configuration]
Aug  9 06:45:33 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[12663]: ntpd 4.2.6p5@1.2349-o Fri Jul  6 20:19:54 UTC 2018 (1)
Aug  9 06:45:33 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[12664]: ntp_io: estimated max descriptors: 72000, initial socket boundary: 16
Aug  9 06:45:33 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[12664]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 06:45:33 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[12664]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 06:45:33 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[12664]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 06:45:33 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[12664]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 06:45:33 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[12664]: Listen normally on 3 eth0 10.20.0.105 UDP 123
Aug  9 06:45:33 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[12664]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 06:45:33 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[12664]: peers refreshed
Aug  9 06:45:33 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f ntpd[12664]: Listening on routing socket on fd #21 for interface updates
Aug  9 06:45:34 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 06:45:34 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 06:45:37 travis-job-f88328bb-2341-4cc9-a8af-50a160e8db6f dbus[1144]: [system] Reloaded configuration
---
travis_time:end:03e12d47:start=1533797205398210996,finish=1533797205405118503,duration=6907507
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2d0aede9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c912d75
travis_time:start:1c912d75
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0023074e
$ dmesg | grep -i kill
