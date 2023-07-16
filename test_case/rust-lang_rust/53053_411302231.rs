plain
[00:59:26] [RUSTC-TIMING] error_chain test:false 1.239
[00:59:53]    Compiling cargo_metadata v0.5.8
[01:00:11]    Compiling clippy_lints v0.0.212 (file:///checkout/src/tools/clippy/clippy_lints)
[01:00:11] [RUSTC-TIMING] cargo_metadata test:false 18.009
[01:00:15] error[E0532]: expected unit struct/variant or constant, found tuple variant `Def::NonMacroAttr`
[01:00:15]     |
[01:00:15]     |
[01:00:15] 945 |         Def::ToolMod | Def::NonMacroAttr | Def::Err => None,
[01:00:15]     |                        ^^^^^^^^^^^^^^^^^ not a unit struct/variant or constant
[01:00:15] 
[01:00:15] error[E0532]: expected unit struct/variant or constant, found tuple variant `Def::NonMacroAttr`
[01:00:15]     |
[01:00:15]     |
[01:00:15] 945 |         Def::ToolMod | Def::NonMacroAttr | Def::Err => None,
[01:00:15]     |                        ^^^^^^^^^^^^^^^^^ not a unit struct/variant or constant
[01:00:16] error[E0261]: use of undeclared lifetime name `'a`
[01:00:16]    --> tools/clippy/clippy_lints/src/write.rs:220:32
[01:00:16]     |
[01:00:16] 220 | fn check_tts(cx: &EarlyContext<'a>, tts: &ThinTokenStream, is_write: bool) -> Option<String> {
---
[01:07:12] [RUSTC-TIMING] rustc_data_structures test:false 5.658
[01:07:12]    Compiling rls-analysis v0.14.0
[01:07:14]    Compiling rustc-ap-arena v209.0.0
[01:07:16] [RUSTC-TIMING] arena test:false 1.296
[01:07:16] error[E0532]: expected unit struct/variant or constant, found tuple variant `Def::NonMacroAttr`
[01:07:16]     |
[01:07:16]     |
[01:07:16] 945 |         Def::ToolMod | Def::NonMacroAttr | Def::Err => None,
[01:07:16]     |                        ^^^^^^^^^^^^^^^^^ not a unit struct/variant or constant
[01:07:17] error[E0261]: use of undeclared lifetime name `'a`
[01:07:17]    --> tools/clippy/clippy_lints/src/write.rs:220:32
[01:07:17]     |
[01:07:17] 220 | fn check_tts(cx: &EarlyContext<'a>, tts: &ThinTokenStream, is_write: bool) -> Option<String> {
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0ec92cc8
$ sudo tail -n 500 /var/log/syslog
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] kvm-clock: using sched offset of 1726271026 cycles
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] Zone ranges:
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000]   Device   empty
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] Movable zone start for each node
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] Early memory node ranges
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] Policy zone: Normal
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] console [ttyS0] enabled
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.501001] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.504781] pid_max: default: 32768 minimum: 301
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.506786] ACPI: Core revision 20150930
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.515024] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.518454] Security Framework initialized
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.520817] Yama: becoming mindful.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.522987] AppArmor: AppArmor disabled by boot time parameter
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.527290] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.540530] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.547035] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.549655] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.553270] Initializing cgroup subsys io
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.554817] Initializing cgroup subsys memory
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.556868] Initializing cgroup subsys devices
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.558680] Initializing cgroup subsys freezer
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.560429] Initializing cgroup subsys net_cls
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.562236] Initializing cgroup subsys perf_event
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.564606] Initializing cgroup subsys net_prio
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.566254] Initializing cgroup subsys hugetlb
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.568194] Initializing cgroup subsys pids
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.569930] CPU: Physical Processor ID: 0
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.571538] CPU: Processor Core ID: 0
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.574149] mce: CPU supports 32 MCE banks
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.575726] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.578318] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.583098] Freeing SMP alternatives memory: 32K
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.593670] ftrace: allocating 32185 entries in 126 pages
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.646552] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.650001] smpboot: Max logical packages: 2
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.652272] x2apic enabled
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.654657] Switched APIC routing to physical x2apic.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.660130] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.767238] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.771961] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.778295] x86: Booting SMP configuration:
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.780082] .... node  #0, CPUs:      #1
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.781967] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.788727]  #2
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.789699] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.795471]  #3
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.796529] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.802816] x86: Booted up 1 node, 4 CPUs
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.804472] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.809945] devtmpfs: initialized
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.816369] evm: security.selinux
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.817986] evm: security.SMACK64
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.819944] evm: security.SMACK64EXEC
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.821398] evm: security.SMACK64TRANSMUTE
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.823207] evm: security.SMACK64MMAP
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.824825] evm: security.ima
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.825712] evm: security.capability
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.827708] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.831789] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.834355] pinctrl core: initialized pinctrl subsystem
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.837054] RTC time:  5:26:14, date: 08/08/18
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.844068] NET: Registered protocol family 16
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.855304] cpuidle: using governor ladder
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.867343] cpuidle: using governor menu
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.869622] PCCT header not found.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.871414] ACPI: bus type PCI registered
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.873462] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.876273] PCI: Using configuration type 1 for base access
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.892836] ACPI: Added _OSI(Module Device)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.895222] ACPI: Added _OSI(Processor Device)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.896967] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.898997] ACPI: Added _OSI(Processor Aggregator Device)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.904317] ACPI: Executed 2 blocks of module-level executable AML code
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.930864] ACPI: Interpreter enabled
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.932264] ACPI: (supports S0 S3 S4 S5)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.934260] ACPI: Using IOAPIC for interrupt routing
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.936567] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.968633] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.971347] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.974558] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.977702] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.985016] PCI host bridge to bus 0000:00
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.986626] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.988941] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.991430] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.994054] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.996801] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.998662] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    0.999121] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.024853] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.049254] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.052448] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.063118] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.070956] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.092607] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.102023] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.109500] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.130704] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.135331] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.139542] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.144266] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.148382] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.170616] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.173039] vgaarb: loaded
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.174265] SCSI subsystem initialized
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.175649] libata version 3.00 loaded.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.175676] ACPI: bus type USB registered
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.177027] usbcore: registered new interface driver usbfs
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.179155] usbcore: registered new interface driver hub
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.181041] usbcore: registered new device driver usb
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.183230] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.185605] dmi: Firmware registration failed.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.187350] PCI: Using ACPI for IRQ routing
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.188926] PCI: pci_cache_line_size set to 64 bytes
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.189034] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.189036] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.189175] NetLabel: Initializing
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.190455] NetLabel:  domain hash size = 128
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.191938] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.193557] NetLabel:  unlabeled traffic allowed by default
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.195822] amd_nb: Cannot enumerate AMD northbridges
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.197522] clocksource: Switched to clocksource kvm-clock
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.207356] pnp: PnP ACPI init
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.208914] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.208990] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.209034] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.209082] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.209122] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.209160] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.209199] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.209426] pnp: PnP ACPI: found 7 devices
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.218666] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.221703] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.221706] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.221708] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.221709] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.221759] NET: Registered protocol family 2
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.223607] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.226527] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.229217] TCP: Hash tables configured (established 131072 bind 65536)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.231763] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.234091] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.237333] NET: Registered protocol family 1
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.239046] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.241547] PCI: CLS 0 bytes, default 64
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    1.241623] Unpacking initramfs...
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.428596] Freeing initrd memory: 21432K
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.429794] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.431204] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.433014] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.434664] hw unit of domain pp0-core 2^-0 Joules
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.435408] hw unit of domain package 2^-0 Joules
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.436116] hw unit of domain dram 2^-16 Joules
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.437106] Scanning for low memory corruption every 60 seconds
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.438950] audit: initializing netlink subsys (disabled)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.439989] audit: type=2000 audit(1533705976.668:1): initialized
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.441421] Initialise system trusted keyring
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.442428] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.443563] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.445645] zbud: loaded
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.446395] VFS: Disk quotas dquot_6.6.0
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.447135] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.448603] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.449928] fuse init (API version 7.23)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.450833] Key type big_key registered
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.451510] Allocating IMA MOK and blacklist keyrings.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.453776] Key type asymmetric registered
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.454459] Asymmetric key parser 'x509' registered
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.455365] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.456549] io scheduler noop registered
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.457095] io scheduler deadline registered (default)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.457922] io scheduler cfq registered
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.458667] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.459654] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.460636] intel_idle: does not run on family 6 model 63
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.460743] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.461890] ACPI: Power Button [PWRF]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.462571] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.463691] ACPI: Sleep Button [SLPF]
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.464683] GHES: HEST is not enabled!
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.468198] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.469174] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.475367] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.476310] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.482192] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.504574] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.528225] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.551565] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.575116] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.578819] Linux agpgart interface v0.103
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.582124] loop: module loaded
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.583375] libphy: Fixed MDIO Bus: probed
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.584740] tun: Universal TUN/TAP device driver, 1.6
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.586209] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.626309] PPP generic driver version 2.4.2
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.627683] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.629753] ehci-pci: EHCI PCI platform driver
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.631067] ehci-platform: EHCI generic platform driver
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.632483] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.634291] ohci-pci: OHCI PCI platform driver
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.635469] ohci-platform: OHCI generic platform driver
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.636870] uhci_hcd: USB Universal Host Controller Interface driver
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.638427] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.640282] i8042: Warning: Keylock active
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.642299] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.643290] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.644533] mousedev: PS/2 mouse device common for all mice
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.646059] rtc_cmos 00:00: RTC can wake from S4
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.647703] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.649784] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.651235] i2c /dev entries driver
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.652024] device-mapper: uevent: version 1.0.3
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.653289] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.655548] ledtrig-cpu: registered to indicate activity on CPUs
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.657854] NET: Registered protocol family 10
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.659384] NET: Registered protocol family 17
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.660533] Key type dns_resolver registered
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.661861] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.663547] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.664905] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.666501] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.668211] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.670898] registered taskstats version 1
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.672011] Loading compiled-in X.509 certificates
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.673667] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.676250] zswap: loaded using pool lzo/zbud
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.679746] Key type trusted registered
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.684072] Key type encrypted registered
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.685221] ima: No TPM chip found, activating TPM-bypass!
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.686782] evm: HMAC attrs: 0x1
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.688368]   Magic number: 14:514:415
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.689286] acpi LNXCPU:da: hash matches
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.690889] rtc_cmos 00:00: setting system clock to 2018-08-08 05:26:17 UTC (1533705977)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.693237] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.695089] EDD information not available.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.696312] PM: Hibernation image not present or could not be loaded.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.697862] Freeing unused kernel memory: 1496K
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.698867] Write protecting the kernel read-only data: 14336k
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.700624] Freeing unused kernel memory: 1956K
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.701533] Freeing unused kernel memory: 92K
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.717756] systemd-udevd[118]: starting version 204
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.778586] scsi host0: Virtio SCSI HBA
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.787642] AVX2 version of gcm_enc/dec engaged.
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.787743] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.791559] AES CTR mode by8 optimization enabled
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.818289] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.818321] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.820723] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.822100] sd 0:0:1:0: [sda] Write Protect is off
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.823038] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.823095] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.825838]  sda: sda1
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.827079] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    3.850402] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    4.433715] tsc: Refined TSC clocksource calibration: 2299.814 MHz
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    4.434793] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212685622b2, max_idle_ns: 440795265736 ns
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    4.687772] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    6.765775] floppy0: no floppy controllers found
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    7.933536] raid6: sse2x1   gen()  8931 MB/s
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.001537] raid6: sse2x1   xor()  6670 MB/s
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.069544] raid6: sse2x2   gen() 10987 MB/s
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.137543] raid6: sse2x2   xor()  7515 MB/s
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.205543] raid6: sse2x4   gen() 12107 MB/s
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.273538] raid6: sse2x4   xor()  8017 MB/s
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.341545] raid6: avx2x1   gen() 16715 MB/s
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.409541] raid6: avx2x2   gen() 19318 MB/s
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.477541] raid6: avx2x4   gen() 19678 MB/s
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.478872] raid6: using algorithm avx2x4 gen() 19678 MB/s
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.481313] raid6: using avx2x2 recovery algorithm
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.485023] xor: automatically using best checksumming function:
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.525562]    avx       : 21705.000 MB/sec
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.541490] Btrfs loaded
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.606940] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.609767] EXT4-fs (sda1): write access will be enabled during recovery
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.692429] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.703058] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.705061] EXT4-fs (sda1): recovery complete
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    8.717061] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    9.037992] random: init: uninitialized urandom read (12 bytes read, 23 bits of entropy available)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    9.193687] random: mountall: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    9.254172] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [    9.502883] random: cloud-init: uninitialized urandom read (32 bytes read, 32 bits of entropy available)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   10.211293] random: cloud-init: uninitialized urandom read (32 bytes read, 40 bits of entropy available)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   10.366815] systemd-udevd[701]: starting version 204
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   10.499579] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   10.552763] intel_rapl: no valid rapl domains found in package 0
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   10.611455] intel_rapl: no valid rapl domains found in package 0
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   10.620998] ppdev: user-space parallel port driver
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   10.738080] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   10.803380] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   10.879764] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   11.055942] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   11.353902] random: mktemp: uninitialized urandom read (12 bytes read, 55 bits of entropy available)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   11.444504] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   11.536004] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   11.586658] EXT4-fs (sda1): resized filesystem to 7864064
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   11.980892] init: failsafe main process (1093) killed by TERM signal
Aug  8 05:26:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO Running set_multiqueue.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO Set channels for eth0 to 4.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 google-accounts: INFO Starting Google Accounts daemon.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 google-accounts: INFO Creating a new user account for me.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 google-accounts: INFO Created user account me.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 google-accounts: INFO Creating a new user account for bogdana.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 google-accounts: INFO Created user account bogdana.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 google-accounts: INFO Creating a new user account for aj.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 google-accounts: INFO Created user account aj.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 google-accounts: INFO Creating a new user account for asari.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 google-accounts: INFO Created user account asari.
Aug  8 05:26:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 google-accounts: INFO Removing user packer.
Aug  8 05:26:27 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 google-clock-skew: INFO Synced system time with hardware clock.
Aug  8 05:26:27 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   13.372516] random: nonblocking pool is initialized
Aug  8 05:26:27 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   13.423568] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  8 05:26:27 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   13.429125] Bridge firewalling registered
Aug  8 05:26:27 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   13.444445] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  8 05:26:27 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   13.486843] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 05:26:27 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   13.565813] floppy0: no floppy controllers found
Aug  8 05:26:27 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   13.566000] work still pending
Aug  8 05:26:27 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   13.571891] Initializing XFRM netlink socket
Aug  8 05:26:27 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   13.585494] Netfilter messages via NETLINK v0.30.
Aug  8 05:26:27 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   13.589712] ctnetlink v0.93: registering with nfnetlink.
Aug  8 05:26:27 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 05:26:27 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 05:26:28 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 cron[1627]: (CRON) INFO (pidfile fd = 3)
Aug  8 05:26:28 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 cron[1659]: (CRON) STARTUP (fork ok)
Aug  8 05:26:28 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 cron[1659]: (CRON) INFO (Running @reboot jobs)
Aug  8 05:26:28 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 05:26:28 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 05:26:28 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 acpid: starting up with netlink and the input layer
Aug  8 05:26:28 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 acpid: 1 rule loaded
Aug  8 05:26:28 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 acpid: waiting for events: event logging is off
Aug  8 05:26:28 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 haveged: haveged starting up
Aug  8 05:26:28 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   15.350247] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1761]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1762]: proto: precision = 0.100 usec
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1762]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1762]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1762]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1762]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1762]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1762]: Listen normally on 3 eth0 10.20.0.184 UDP 123
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1762]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1762]: peers refreshed
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1762]: Listening on routing socket on fd #21 for interface updates
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   20.531838] init: plymouth-upstart-bridge main process ended, respawning
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 startup-script: INFO Found startup-script in metadata.
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 startup-script: INFO startup-script: job 1 at Wed Aug  8 08:36:00 2018
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 startup-script: INFO startup-script: Return code 0.
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 startup-script: INFO startup-script: Return code 0.
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 startup-script: INFO Finished running startup scripts.
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ec2: 
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ec2: #############################################################
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ec2: 1024 08:f4:8a:d5:90:41:7c:c8:d8:a7:01:f7:50:ca:9d:f6  root@travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 (DSA)
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ec2: 256 88:33:98:44:c9:12:fe:c5:b9:a6:97:60:72:56:e7:13  root@travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 (ECDSA)
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ec2: 256 8d:e8:0e:75:8b:5f:89:65:0b:4d:a4:f4:f2:52:58:ef  root@travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 (ED25519)
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ec2: 2048 1b:9c:ac:13:0e:25:bf:e2:c9:6c:94:c8:fc:37:68:3c  root@travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 (RSA)
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  8 05:26:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ec2: #############################################################
Aug  8 05:26:42 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpdate[2158]: the NTP socket is in use, exiting
Aug  8 05:27:25 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [   71.573355] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  8 05:28:15 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [  121.527173] device veth8f68c8c entered promiscuous mode
Aug  8 05:28:15 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [  121.527219] docker0: port 1(veth8f68c8c) entered forwarding state
Aug  8 05:28:15 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [  121.527224] docker0: port 1(veth8f68c8c) entered forwarding state
Aug  8 05:28:15 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [  121.528224] docker0: port 1(veth8f68c8c) entered disabled state
Aug  8 05:28:15 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [  121.639447] cgroup: docker-runc (4775) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  8 05:28:15 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [  121.639451] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  8 05:28:15 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [  121.719113] eth0: renamed from vetha5189b6
Aug  8 05:28:15 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [  121.753977] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  8 05:28:15 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [  121.755419] docker0: port 1(veth8f68c8c) entered forwarding state
Aug  8 05:28:15 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [  121.755446] docker0: port 1(veth8f68c8c) entered forwarding state
Aug  8 05:28:15 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [  121.755468] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  8 05:28:18 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1762]: Listen normally on 5 docker0 fe80::42:d7ff:fea9:6e7d UDP 123
Aug  8 05:28:18 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1762]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  8 05:28:18 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1762]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  8 05:28:18 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1762]: peers refreshed
Aug  8 05:28:18 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1762]: new interface(s) found: waking up resolver
Aug  8 05:28:30 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [  136.794321] docker0: port 1(veth8f68c8c) entered forwarding state
Aug  8 06:17:01 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 CRON[3386]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  8 06:25:01 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 CRON[1394]: (root) CMD (test -x /usr/sbin/anacron || ( cd / && run-parts --report /etc/cron.daily ))
Aug  8 06:29:00 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 upstart-udev-bridge[695]: Disconnected from Upstart
Aug  8 06:29:00 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 upstart-socket-bridge[850]: Disconnected from Upstart
Aug  8 06:29:00 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [ 3766.492715] init: upstart-udev-bridge main process (695) terminated with status 1
Aug  8 06:29:00 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [ 3766.492725] init: upstart-udev-bridge main process ended, respawning
Aug  8 06:29:00 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [ 3766.492829] init: upstart-socket-bridge main process (850) terminated with status 1
Aug  8 06:29:00 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [ 3766.492837] init: upstart-socket-bridge main process ended, respawning
Aug  8 06:29:00 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [ 3766.492919] init: upstart-file-bridge main process (1202) terminated with status 1
Aug  8 06:29:00 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [ 3766.492926] init: upstart-file-bridge main process ended, respawning
Aug  8 06:29:15 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 dbus[1153]: [system] Reloaded configuration
Aug  8 06:29:15 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 dbus[1153]: message repeated 9 times: [ [system] Reloaded configuration]
Aug  8 06:29:19 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[1762]: ntpd exiting on signal 15
Aug  8 06:29:26 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [ 3792.166419] init: apport post-stop process (15294) terminated with status 1
Aug  8 06:29:27 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 dbus[1153]: [system] Reloaded configuration
Aug  8 06:29:28 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 dbus[1153]: message repeated 4 times: [ [system] Reloaded configuration]
Aug  8 06:29:32 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 kernel: [ 3798.956268] systemd-udevd[18250]: starting version 204
Aug  8 06:29:33 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 dbus[1153]: [system] Reloaded configuration
Aug  8 06:29:33 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 dbus[1153]: message repeated 3 times: [ [system] Reloaded configuration]
Aug  8 06:29:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[18802]: ntpd 4.2.6p5@1.2349-o Fri Jul  6 20:19:54 UTC 2018 (1)
Aug  8 06:29:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[18803]: ntp_io: estimated max descriptors: 72000, initial socket boundary: 16
Aug  8 06:29:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[18803]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  8 06:29:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[18803]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  8 06:29:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[18803]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  8 06:29:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[18803]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  8 06:29:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[18803]: Listen normally on 3 eth0 10.20.0.184 UDP 123
Aug  8 06:29:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[18803]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  8 06:29:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[18803]: peers refreshed
Aug  8 06:29:34 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 ntpd[18803]: Listening on routing socket on fd #21 for interface updates
Aug  8 06:29:35 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 06:29:35 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 06:29:36 travis-job-1a9620be-e4c7-4d46-9d58-62c3682a5c98 dbus[1153]: [system] Reloaded configuration
---
travis_time:end:0178c443:start=1533710225382661549,finish=1533710225391049958,duration=8388409
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:058304d2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09b6721f
travis_time:start:09b6721f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0389d34d
$ dmesg | grep -i kill
