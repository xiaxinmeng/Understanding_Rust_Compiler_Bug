plain
###########                                                               16.7%
######################################################################## 100.0%
[00:02:23] extracting /checkout/obj/build/cache/2018-08-01/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:23]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:48] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:48] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:48] make: *** [prepare] Error 1
[00:02:48] Makefile:81: recipe for target 'prepare' failed
[00:02:49] Command failed. Attempt 2/5:
[00:02:49] Command failed. Attempt 2/5:
[00:02:50] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:50] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:50] Makefile:81: recipe for target 'prepare' failed
[00:02:50] make: *** [prepare] Error 1
[00:02:52] Command failed. Attempt 3/5:
[00:02:52] Command failed. Attempt 3/5:
[00:02:52] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:52] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:52] make: *** [prepare] Error 1
[00:02:52] Makefile:81: recipe for target 'prepare' failed
[00:02:55] Command failed. Attempt 4/5:
[00:02:55] Command failed. Attempt 4/5:
[00:02:55] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:55] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:55] make: *** [prepare] Error 1
[00:02:55] Makefile:81: recipe for target 'prepare' failed
[00:02:59] Command failed. Attempt 5/5:
[00:02:59] Command failed. Attempt 5/5:
[00:03:00] error: the lock file needs to be updated but --locked was passed to prevent this
[00:03:00] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:00] Makefile:81: recipe for target 'prepare' failed
[00:03:00] make: *** [prepare] Error 1
[00:03:00] The command has failed after 5 attempts.
travis_time:end:181e7660:start=1534417165399673550,finish=1534417355032779450,duration=189633105900
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:01530cf9
$ sudo tail -n 500 /var/log/syslog
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] kvm-clock: using sched offset of 1852995249 cycles
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] Zone ranges:
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000]   Device   empty
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] Movable zone start for each node
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] Early memory node ranges
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] Policy zone: Normal
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] Hierarchical RCU implementation.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] console [ttyS0] enabled
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.533079] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.536903] pid_max: default: 32768 minimum: 301
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.539235] ACPI: Core revision 20150930
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.548446] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.551095] Security Framework initialized
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.552337] Yama: becoming mindful.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.553977] AppArmor: AppArmor disabled by boot time parameter
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.558744] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.571669] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.579289] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.581899] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.585624] Initializing cgroup subsys io
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.587514] Initializing cgroup subsys memory
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.589274] Initializing cgroup subsys devices
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.591202] Initializing cgroup subsys freezer
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.592741] Initializing cgroup subsys net_cls
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.594719] Initializing cgroup subsys perf_event
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.596529] Initializing cgroup subsys net_prio
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.598416] Initializing cgroup subsys hugetlb
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.600168] Initializing cgroup subsys pids
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.602099] CPU: Physical Processor ID: 0
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.603733] CPU: Processor Core ID: 0
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.606575] mce: CPU supports 32 MCE banks
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.608669] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.611314] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.616970] Freeing SMP alternatives memory: 32K
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.629856] ftrace: allocating 32185 entries in 126 pages
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.693089] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.696518] smpboot: Max logical packages: 2
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.699091] x2apic enabled
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.702344] Switched APIC routing to physical x2apic.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.707537] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.814876] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.818397] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.824330] x86: Booting SMP configuration:
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.825974] .... node  #0, CPUs:      #1
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.827713] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.834093]  #2
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.835651] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.841624]  #3
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.842470] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.848950] x86: Booted up 1 node, 4 CPUs
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.850300] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.855105] devtmpfs: initialized
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.861020] evm: security.selinux
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.862541] evm: security.SMACK64
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.863778] evm: security.SMACK64EXEC
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.864864] evm: security.SMACK64TRANSMUTE
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.866071] evm: security.SMACK64MMAP
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.867447] evm: security.ima
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.868945] evm: security.capability
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.871339] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.874751] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.877247] pinctrl core: initialized pinctrl subsystem
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.879405] RTC time: 10:58:20, date: 08/16/18
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.882005] NET: Registered protocol family 16
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.894972] cpuidle: using governor ladder
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.907007] cpuidle: using governor menu
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.908553] PCCT header not found.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.910180] ACPI: bus type PCI registered
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.912288] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.914684] PCI: Using configuration type 1 for base access
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.928596] ACPI: Added _OSI(Module Device)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.930343] ACPI: Added _OSI(Processor Device)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.932252] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.934268] ACPI: Added _OSI(Processor Aggregator Device)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.939055] ACPI: Executed 2 blocks of module-level executable AML code
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.965411] ACPI: Interpreter enabled
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.966966] ACPI: (supports S0 S3 S4 S5)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.968275] ACPI: Using IOAPIC for interrupt routing
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    0.970364] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.004441] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.006930] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.009164] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.011859] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.016474] PCI host bridge to bus 0000:00
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.018073] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.020314] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.023514] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.026471] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.029062] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.030952] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.031418] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.060387] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.088604] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.091470] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.103186] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.112265] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.136696] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.147941] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.156243] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.183516] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.187626] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.192421] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.197177] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.201263] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.224618] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.227200] vgaarb: loaded
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.228491] SCSI subsystem initialized
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.229789] libata version 3.00 loaded.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.229813] ACPI: bus type USB registered
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.231333] usbcore: registered new interface driver usbfs
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.232975] usbcore: registered new interface driver hub
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.234559] usbcore: registered new device driver usb
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.237051] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.239280] dmi: Firmware registration failed.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.241923] PCI: Using ACPI for IRQ routing
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.243516] PCI: pci_cache_line_size set to 64 bytes
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.243649] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.243651] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.243836] NetLabel: Initializing
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.245019] NetLabel:  domain hash size = 128
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.247327] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.249027] NetLabel:  unlabeled traffic allowed by default
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.250886] amd_nb: Cannot enumerate AMD northbridges
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.253046] clocksource: Switched to clocksource kvm-clock
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.262916] pnp: PnP ACPI init
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.264967] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.265035] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.265096] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.265146] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.265186] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.265226] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.265268] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.265443] pnp: PnP ACPI: found 7 devices
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.275835] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.278457] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.278459] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.278460] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.278462] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.278502] NET: Registered protocol family 2
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.279981] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.282825] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.285881] TCP: Hash tables configured (established 131072 bind 65536)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.288696] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.291455] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.294421] NET: Registered protocol family 1
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.295979] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.298018] PCI: CLS 0 bytes, default 64
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    1.298085] Unpacking initramfs...
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.553999] Freeing initrd memory: 21432K
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.556689] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.559915] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.563832] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.569052] hw unit of domain pp0-core 2^-0 Joules
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.571685] hw unit of domain package 2^-0 Joules
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.573674] hw unit of domain dram 2^-0 Joules
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.575520] Scanning for low memory corruption every 60 seconds
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.578223] audit: initializing netlink subsys (disabled)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.581228] audit: type=2000 audit(1534417102.596:1): initialized
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.584704] Initialise system trusted keyring
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.587015] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.589328] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.593560] zbud: loaded
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.595421] VFS: Disk quotas dquot_6.6.0
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.596884] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.599973] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.603444] fuse init (API version 7.23)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.605470] Key type big_key registered
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.606902] Allocating IMA MOK and blacklist keyrings.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.614568] Key type asymmetric registered
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.616852] Asymmetric key parser 'x509' registered
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.619550] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.622708] io scheduler noop registered
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.624135] io scheduler deadline registered (default)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.626254] io scheduler cfq registered
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.628023] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.629950] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.632179] intel_idle: does not run on family 6 model 62
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.632303] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.635667] ACPI: Power Button [PWRF]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.637313] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.640166] ACPI: Sleep Button [SLPF]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.642499] GHES: HEST is not enabled!
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.647360] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.650785] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.660316] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.662684] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.672281] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.696458] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.722426] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.747986] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.773434] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.779248] Linux agpgart interface v0.103
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.785235] loop: module loaded
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.787191] libphy: Fixed MDIO Bus: probed
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.788986] tun: Universal TUN/TAP device driver, 1.6
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.790572] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.853405] PPP generic driver version 2.4.2
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.857129] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.859993] ehci-pci: EHCI PCI platform driver
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.861811] ehci-platform: EHCI generic platform driver
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.863978] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.866302] ohci-pci: OHCI PCI platform driver
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.867836] ohci-platform: OHCI generic platform driver
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.869811] uhci_hcd: USB Universal Host Controller Interface driver
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.872245] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.876618] i8042: Warning: Keylock active
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.879432] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.881148] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.883741] mousedev: PS/2 mouse device common for all mice
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.886652] rtc_cmos 00:00: RTC can wake from S4
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.889072] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.892035] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.894660] i2c /dev entries driver
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.896077] device-mapper: uevent: version 1.0.3
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.898386] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.901552] ledtrig-cpu: registered to indicate activity on CPUs
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.906032] NET: Registered protocol family 10
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.907904] NET: Registered protocol family 17
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.910635] Key type dns_resolver registered
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.913107] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.915704] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.918331] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.920340] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.922433] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.926936] registered taskstats version 1
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.928507] Loading compiled-in X.509 certificates
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.930968] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.934854] zswap: loaded using pool lzo/zbud
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.939923] Key type trusted registered
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.946964] Key type encrypted registered
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.948617] ima: No TPM chip found, activating TPM-bypass!
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.950509] evm: HMAC attrs: 0x1
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.952341]   Magic number: 14:715:982
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.953894] rtc_cmos 00:00: setting system clock to 2018-08-16 10:58:23 UTC (1534417103)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.957738] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.960265] EDD information not available.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.962119] PM: Hibernation image not present or could not be loaded.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.964234] Freeing unused kernel memory: 1496K
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.966407] Write protecting the kernel read-only data: 14336k
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.970233] Freeing unused kernel memory: 1956K
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.972162] Freeing unused kernel memory: 92K
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    3.992889] systemd-udevd[119]: starting version 204
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    4.074884] scsi host0: Virtio SCSI HBA
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    4.083021] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    4.089469] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    4.094591] AVX version of gcm_enc/dec engaged.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    4.096811] AES CTR mode by8 optimization enabled
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    4.162609] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    4.162864] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    4.162865] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    4.163391] sd 0:0:1:0: [sda] Write Protect is off
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    4.163393] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    4.163469] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    4.168784]  sda: sda1
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    4.170479] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    4.573331] tsc: Refined TSC clocksource calibration: 2499.795 MHz
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    4.575950] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x240877ee81b, max_idle_ns: 440795211433 ns
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    4.926413] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    7.049259] floppy0: no floppy controllers found
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.237071] raid6: sse2x1   gen()  8827 MB/s
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.305062] raid6: sse2x1   xor()  6679 MB/s
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.373068] raid6: sse2x2   gen() 11000 MB/s
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.441055] raid6: sse2x2   xor()  7479 MB/s
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.509057] raid6: sse2x4   gen() 12624 MB/s
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.577060] raid6: sse2x4   xor()  8409 MB/s
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.579001] raid6: using algorithm sse2x4 gen() 12624 MB/s
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.580745] raid6: .... xor() 8409 MB/s, rmw enabled
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.582580] raid6: using ssse3x2 recovery algorithm
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.586209] xor: automatically using best checksumming function:
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.625101]    avx       : 22096.000 MB/sec
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.643219] Btrfs loaded
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.709201] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.713366] EXT4-fs (sda1): write access will be enabled during recovery
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.814140] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.828097] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.829336] EXT4-fs (sda1): recovery complete
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    8.835244] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    9.054412] random: init: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    9.186534] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    9.237144] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [    9.454657] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   10.131593] random: cloud-init: uninitialized urandom read (32 bytes read, 46 bits of entropy available)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   10.295638] systemd-udevd[702]: starting version 204
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   10.427064] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   10.533850] ppdev: user-space parallel port driver
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   10.657466] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   10.725956] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   10.798258] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   10.983848] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   11.308977] random: mktemp: uninitialized urandom read (12 bytes read, 60 bits of entropy available)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   11.401107] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   11.498691] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   11.558531] EXT4-fs (sda1): resized filesystem to 7864064
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   11.921820] init: failsafe main process (1093) killed by TERM signal
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO Running set_multiqueue.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO Set channels for eth0 to 4.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 16 10:58:31 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Starting Google Accounts daemon.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Creating a new user account for me.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Created user account me.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Created user account me.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Creating a new user account for henrik.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Created user account henrik.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Creating a new user account for emma.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Created user account emma.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Creating a new user account for igor.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   13.068033] random: nonblocking pool is initialized
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Created user account igor.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Created user account konstantinhaase.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Creating a new user account for aj.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Created user account aj.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Creating a new user account for solarce.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Created user account solarce.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Creating a new user account for asari.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Created user account asari.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Creating a new user account for bogdana.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   13.406276] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   13.413156] Bridge firewalling registered
Aug 16 10:58:32 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   13.427976] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Created user account bogdana.
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   13.471648] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Creating a new user account for konstantin.
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   13.481519] floppy0: no floppy controllers found
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Created user account konstantin.
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-clock-skew: INFO Synced system time with hardware clock.
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Creating a new user account for carmen.
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   13.607736] Initializing XFRM netlink socket
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   13.618115] Netfilter messages via NETLINK v0.30.
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   13.621294] ctnetlink v0.93: registering with nfnetlink.
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Created user account carmen.
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Creating a new user account for maria.
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Created user account maria.
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c google-accounts: INFO Removing user packer.
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c cron[1706]: (CRON) INFO (pidfile fd = 3)
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c cron[1740]: (CRON) STARTUP (fork ok)
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c cron[1740]: (CRON) INFO (Running @reboot jobs)
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c acpid: starting up with netlink and the input layer
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c acpid: 1 rule loaded
Aug 16 10:58:33 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c acpid: waiting for events: event logging is off
Aug 16 10:58:34 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c haveged: haveged starting up
Aug 16 10:58:34 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   14.847167] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpd[1843]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpd[1844]: proto: precision = 0.128 usec
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpd[1844]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpd[1844]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpd[1844]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpd[1844]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpd[1844]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpd[1844]: Listen normally on 3 eth0 10.20.1.164 UDP 123
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpd[1844]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpd[1844]: peers refreshed
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpd[1844]: Listening on routing socket on fd #21 for interface updates
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   20.037103] init: plymouth-upstart-bridge main process ended, respawning
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c startup-script: INFO Found startup-script in metadata.
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c startup-script: INFO startup-script: job 1 at Thu Aug 16 14:08:00 2018
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c startup-script: INFO startup-script: Return code 0.
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c startup-script: INFO startup-script: Return code 0.
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c startup-script: INFO Finished running startup scripts.
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ec2: 
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ec2: #############################################################
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ec2: 1024 92:e2:16:e8:43:27:18:b3:69:54:4b:fd:0e:f3:39:41  root@travis-job-0d60aded-ca1e-4096-813c-fb46a663160c (DSA)
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ec2: 256 c8:d8:42:60:18:24:e4:7b:e8:f7:9e:0c:ae:b4:e1:ab  root@travis-job-0d60aded-ca1e-4096-813c-fb46a663160c (ECDSA)
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ec2: 256 ff:11:67:08:5b:32:59:cc:70:0d:41:dd:e8:e5:ec:6c  root@travis-job-0d60aded-ca1e-4096-813c-fb46a663160c (ED25519)
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ec2: 2048 cb:27:7c:b1:44:33:d3:46:9b:60:bf:c6:5c:84:ef:e9  root@travis-job-0d60aded-ca1e-4096-813c-fb46a663160c (RSA)
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 16 10:58:39 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ec2: #############################################################
Aug 16 10:58:48 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpdate[2237]: the NTP socket is in use, exiting
Aug 16 10:59:24 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [   64.995934] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 16 11:01:34 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [  194.965533] device veth68af5be entered promiscuous mode
Aug 16 11:01:34 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [  194.965594] docker0: port 1(veth68af5be) entered forwarding state
Aug 16 11:01:34 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [  194.965601] docker0: port 1(veth68af5be) entered forwarding state
Aug 16 11:01:34 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [  194.966072] docker0: port 1(veth68af5be) entered disabled state
Aug 16 11:01:34 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [  195.077550] cgroup: docker-runc (4948) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 16 11:01:34 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [  195.077553] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 16 11:01:34 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [  195.169134] eth0: renamed from veth608c087
Aug 16 11:01:34 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [  195.223353] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 16 11:01:34 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [  195.225071] docker0: port 1(veth68af5be) entered forwarding state
Aug 16 11:01:34 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [  195.225091] docker0: port 1(veth68af5be) entered forwarding state
Aug 16 11:01:34 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [  195.225111] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 16 11:01:38 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpd[1844]: Listen normally on 5 docker0 fe80::42:34ff:feb6:52d0 UDP 123
Aug 16 11:01:38 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpd[1844]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 16 11:01:38 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpd[1844]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 16 11:01:38 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpd[1844]: peers refreshed
Aug 16 11:01:38 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c ntpd[1844]: new interface(s) found: waking up resolver
Aug 16 11:01:49 travis-job-0d60aded-ca1e-4096-813c-fb46a663160c kernel: [  210.232382] docker0: port 1(veth68af5be) entered forwarding state
---
travis_time:end:1bc62137:start=1534417355544035752,finish=1534417355554098121,duration=10062369
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:21dad4a0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:065ee740
travis_time:start:065ee740
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:08a49df0
$ dmesg | grep -i kill
