plain
[00:04:10]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:04:16] error: This node does not have a stability attribute
[00:04:16]     --> libcore/option.rs:1067:1
[00:04:16]      |
[00:04:16] 1067 | / impl<'a, T> From<&'a Option<T>> for Option<&'a T> {
[00:04:16] 1068 | |     fn from(o: &'a Option<T>) -> Option<&'a T> {
[00:04:16] 1069 | |         o.as_ref()
[00:04:16] 1071 | | }
[00:04:16]      | |_^
[00:04:16] 
[00:04:17] error: aborting due to previous error
---
[00:04:17] warning: build failed, waiting for other jobs to finish...
[00:04:19] error: build failed
[00:04:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:19] expected success, got: exit code: 101
[00:04:19] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:04:19] travis_fold:end:stage0-std

[00:04:19] travis_time:end:stage0-std:start=1533807984028756803,finish=1533808007416551906,duration=23387795103


[00:04:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:19] Build completed unsuccessfully in 0:00:24
[00:04:19] make: *** [all] Error 1
[00:04:19] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13c2a494
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:062a0fc0
$ sudo tail -n 500 /var/log/syslog
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   2 disabled
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   3 disabled
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   4 disabled
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   5 disabled
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   6 disabled
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   7 disabled
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Using GB pages for direct mapping
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] kvm-clock: using sched offset of 1289945946 cycles
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Zone ranges:
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   Device   empty
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Movable zone start for each node
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Early memory node ranges
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]   DMA32 zone: 12224 pages used for 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Policy zone: Normal
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] console [ttyS0] enabled
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.317492] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.318862] pid_max: default: 32768 minimum: 301
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.319627] ACPI: Core revision 20150930
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.326037] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.327134] Security Framework initialized
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.327728] Yama: becoming mindful.
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.328335] AppArmor: AppArmor disabled by boot time parameter
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.331143] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.340351] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.344759] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.345751] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.347158] Initializing cgroup subsys io
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.347823] Initializing cgroup subsys memory
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.348491] Initializing cgroup subsys devices
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.349230] Initializing cgroup subsys freezer
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.349909] Initializing cgroup subsys net_cls
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.350668] Initializing cgroup subsys perf_event
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.351507] Initializing cgroup subsys net_prio
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.352477] Initializing cgroup subsys hugetlb
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.353257] Initializing cgroup subsys pids
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.354153] CPU: Physical Processor ID: 0
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.354939] CPU: Processor Core ID: 0
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.355474] mce: CPU supports 32 MCE banks
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.356224] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.357223] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.359862] Freeing SMP alternatives memory: 32K
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.368302] ftrace: allocating 32185 entries in 126 pages
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.414139] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.415252] smpboot: Max logical packages: 2
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.416427] x2apic enabled
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.418002] Switched APIC routing to physical x2apic.
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.421728] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.528719] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.530309] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.532916] x86: Booting SMP configuration:
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.533543] .... node  #0, CPUs:      #1
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.534320] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.537755]  #2
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.538305] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.541990]  #3
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.542441] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.545928] x86: Booted up 1 node, 4 CPUs
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.546649] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.549026] devtmpfs: initialized
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.553273] evm: security.selinux
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.553769] evm: security.SMACK64
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.554293] evm: security.SMACK64EXEC
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.554851] evm: security.SMACK64TRANSMUTE
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.555539] evm: security.SMACK64MMAP
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.556045] evm: security.ima
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.556533] evm: security.capability
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.557521] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.560232] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.561506] pinctrl core: initialized pinctrl subsystem
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.562832] RTC time:  9:41:16, date: 08/09/18
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.564479] NET: Registered protocol family 16
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.576760] cpuidle: using governor ladder
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.588755] cpuidle: using governor menu
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.589515] PCCT header not found.
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.590120] ACPI: bus type PCI registered
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.590705] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.591834] PCI: Using configuration type 1 for base access
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.605789] ACPI: Added _OSI(Module Device)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.671971] PCI host bridge to bus 0000:00
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.672576] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.673596] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.674604] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.675653] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.676815] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.677650] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.678074] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.692074] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.704239] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.705611] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567zed
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.781879] libata version 3.00 loaded.
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.781907] ACPI: bus type USB registered
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.782563] usbcore: registered new interface driver usbfs
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.783334] usbcore: registered new interface driver hub
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.784166] usbcore: registered new device driver usb
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.785271] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.786368] dmi: Firmware registration failed.
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.787219] PCI: Using ACPI for IRQ routing
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.787865] PCI: pci_cache_line_size set to 64 bytes
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.787957] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.787959] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.788069] NetLabel: Initializing
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.788582] NetLabel:  domain hash size = 128
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.789229] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.789968] NetLabel:  unlabeled traffic allowed by default
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.791031] amd_nb: Cannot enumerate AMD northbridges
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.791815] clocksource: Switched to clocksource kvm-clock
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.799201] pnp: PnP ACPI init
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.799866] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.799939] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.799983] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.800033] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.800082] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.800126] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.800167] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.800323] pnp: PnP ACPI: found 7 devices
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.807482] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.809024] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.809027] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.809028] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.809030] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.809078] NET: Registered protocol family 2
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.809943] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.811360] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.812428] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.813393] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.814546] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.816237] NET: Registered protocol family 1
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.817127] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.818142] PCI: CLS 0 bytes, default 64
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    0.818193] Unpacking initramfs...
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.776076] Freeing initrd memory: 21432K
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.776983] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.778230] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.779775] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.781060] hw unit of domain pp0-core 2^-0 Joules
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.782048] hw unit of domain package 2^-0 Joules
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.782711] hw unit of domain dram 2^-0 Joules
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.783437] Scanning for low memory corruption every 60 seconds
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.784729] audit: initializing netlink subsys (disabled)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.785518] audit: type=2000 audit(1533807678.816:1): initialized
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.786616] Initialise system trusted keyring
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.787566] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.788601] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.790781] zbud: loaded
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.791466] VFS: Disk quotas dquot_6.6.0
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.792172] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.793524] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.794894] fuse init (API version 7.23)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.795669] Key type big_key registered
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.796268] Allocating IMA MOK and blacklist keyrings.
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.798116] Key type asymmetric registered
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.798941] Asymmetric key parser 'x509' registered
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.799670] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.800915] io scheduler noop registered
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.801514] io scheduler deadline registered (default)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.802748] io scheduler cfq registered
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.803407] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.804305] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.805227] intel_idle: does not run on family 6 model 45
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.805318] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.806437] ACPI: Power Button [PWRF]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.807069] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.808454] ACPI: Sleep Button [SLPF]
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.809423] GHES: HEST is not enabled!
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.811596] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.812694] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.817033] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.818055] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.822140] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.844371] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.867625] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.890567] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.913276] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.916102] Linux agpgart interface v0.103
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.919581] loop: module loaded
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.920545] libphy: Fixed MDIO Bus: probed
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.921584] tun: Universal TUN/TAP device driver, 1.6
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.922616] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.950195] PPP generic driver version 2.4.2
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.950922] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.951812] ehci-pci: EHCI PCI platform driver
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.952706] ehci-platform: EHCI generic platform driver
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.953783] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.954681] ohci-pci: OHCI PCI platform driver
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.955495] ohci-platform: OHCI generic platform driver
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.956289] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernelT: Registered protocol family 10
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.974093] NET: Registered protocol family 17
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.974889] Key type dns_resolver registered
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.976036] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.977359] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.978800] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.979734] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.981184] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.983212] registered taskstats version 1
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.984011] Loading compiled-in X.509 certificates
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.985221] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.987674] zswap: loaded using pool lzo/zbud
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.990368] Key type trusted registered
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.994052] Key type encrypted registered
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.994798] ima: No TPM chip found, activating TPM-bypass!
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.996314] evm: HMAC attrs: 0x1
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.997467]   Magic number: 14:772:676
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.998236] acpi LNXCPU:21: hash matches
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    2.998954] rtc_cmos 00:00: setting system clock to 2018-08-09 09:41:19 UTC (1533807679)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.000412] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.001523] EDD information not available.
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.002662] PM: Hibernation image not present or could not be loaded.
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.004006] Freeing unused kernel memory: 1496K
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.004813] Write protecting the kernel read-only data: 14336k
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.006444] Freeing unused kernel memory: 1956K
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.007430] Freeing unused kernel memory: 92K
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.020360] systemd-udevd[119]: starting version 204
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.072842] scsi host0: Virtio SCSI HBA
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.075334] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.081127] AVX version of gcm_enc/dec engaged.
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.081950] AES CTR mode by8 optimization enabled
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.112977] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.113021] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.113023] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.113172] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.113174] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.113218] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.114309]  sda: sda1
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.114830] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.164578] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.780006] tsc: Refined TSC clocksource calibration: 2600.005 MHz
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    3.780893] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a40ff313, max_idle_ns: 440795330741 ns
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    4.002842] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    6.076074] floppy0: no floppy controllers found
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.231851] raid6: sse2x1   gen()  9029 MB/s
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.299850] raid6: sse2x1   xor()  6751 MB/s
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.367850] raid6: sse2x2   gen() 10962 MB/s
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.435848] raid6: sse2x2   xor()  7362 MB/s
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.503841] raid6: sse2x4   gen() 12819 MB/s
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.571881] raid6: sse2x4   xor()  8929 MB/s
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.572873] raid6: using algorithm sse2x4 gen() 12819 MB/s
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.573780] raid6: .... xor() 8929 MB/s, rmw enabled
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.574567] raid6: using ssse3x2 recovery algorithm
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.576988] xor: automatically using best checksumming function:
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.615823]    avx       : 27878.000 MB/sec
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.629253] Btrfs loaded
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.667181] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.668535] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.735769] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.742285] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.743322] EXT4-fs (sda1): recovery complete
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.748099] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    7.938417] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    8.041656] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    8.088465] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    8.264407] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    8.789770] random: cloud-init: uninitialized urandom read (32 bytes read, 45 bits of entropy available)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    8.909473] systemd-udevd[701]: starting version 204
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    9.006577] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    9.074349] intel_rapl: no valid rapl domains found in package 0
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    9.117918] ppdev: user-space parallel port driver
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    9.206885] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    9.251903] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    9.319256] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    9.478931] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    9.711345] random: mktemp: uninitialized urandom read (12 bytes read, 60 bits of entropy available)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    9.778666] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    9.846707] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [    9.882212] EXT4-fs (sda1): resized filesystem to 7864064
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [   10.273495] init: failsafe main process (1092) killed by TERM signal
Aug  9 09:41:26 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 instance-setup: INFO Running set_multiqueue.
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 instance-setup: INFO Set channels for eth0 to 4.
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 instance-setup: INFO /proc/irq/30/smp_oogle Accounts daemon.
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 cron[1337]: (CRON) INFO (pidfile fd = 3)
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 cron[1370]: (CRON) STARTUP (fork ok)
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 cron[1370]: (CRON) INFO (Running @reboot jobs)
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 google-accounts: INFO Creating a new user account for me.
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 acpid: starting up with netlink and the input layer
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 acpid: 1 rule loaded
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 acpid: waiting for events: event logging is off
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [   11.040619] random: nonblocking pool is initialized
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 google-accounts: INFO Created user account me.
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 google-accounts: INFO Creating a new user account for aj.
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 google-accounts: INFO Created user account aj.
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 google-accounts: INFO Creating a new user account for carmen.
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 google-accounts: INFO Created user account carmen.
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 google-accounts: INFO Removing user packer.
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 haveged: haveged starting up
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [   11.378564] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 09:41:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [   11.390327] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 09:41:28 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [   11.628305] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  9 09:41:28 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [   11.631347] Bridge firewalling registered
Aug  9 09:41:28 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [   11.637508] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 09:41:28 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [   11.700143] Initializing XFRM netlink socket
Aug  9 09:41:28 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [   11.706658] Netfilter messages via NETLINK v0.30.
Aug  9 09:41:28 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [   11.709128] ctnetlink v0.93: registering with nfnetlink.
Aug  9 09:41:28 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 09:41:28 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [   12.107908] floppy0: no floppy controllers found
Aug  9 09:41:50 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpdate[1748]: adjust time server 169.254.169.254 offset 0.013909 sec
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpd[1781]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpd[1782]: proto: precision = 0.104 usec
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpd[1782]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpd[1782]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpd[1782]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpd[1782]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpd[1782]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpd[1782]: Listen normally on 3 eth0 10.20.0.86 UDP 123
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpd[1782]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpd[1782]: peers refreshed
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpd[1782]: Listening on routing socket on fd #21 for interface updates
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [   41.560143] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 startup-script: INFO Found startup-script in metadata.
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 startup-script: INFO startup-script: job 1 at Thu Aug  9 12:51:00 2018
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 startup-script: INFO startup-script: Return code 0.
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 startup-script: INFO startup-script: Return code 0.
Aug  9 09:41:57 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 startup-script: INFO Finished running startup scripts.
Aug  9 09:41:58 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ec2: 
Aug  9 09:41:58 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ec2: #############################################################
Aug  9 09:41:58 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 09:41:58 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ec2: 1024 29:81:ba:74:a5:d2:44:c1:fe:14:45:86:0f:44:02:29  root@travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 (DSA)
Aug  9 09:41:58 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ec2: 256 00:65:03:36:50:0b:83:bb:cd:64:48:4f:8a:97:2c:25  root@travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 (ECDSA)
Aug  9 09:41:58 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ec2: 256 c5:eb:35:2c:06:20:79:e5:a0:93:9f:50:a0:6a:3c:41  root@travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 (ED25519)
Aug  9 09:41:58 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ec2: 2048 79:c9:16:2b:7a:63:ac:c7:7e:57:c3:06:3c:88:89:22  root@travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 (RSA)
Aug  9 09:41:58 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 09:41:58 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ec2: #############################################################
Aug  9 09:42:27 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [   70.823564] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 09:43:34 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  137.824863] device vetha379069 entered promiscuous mode
Aug  9 09:43:34 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  137.824962] docker0: port 1(vetha379069) entered forwarding state
Aug  9 09:43:34 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  137.824974] docker0: port 1(vetha379069) entered forwarding state
Aug  9 09:43:34 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  137.825284] docker0: port 1(vetha379069) entered disabled state
Aug  9 09:43:34 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  137.902522] cgroup: docker-runc (4762) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 09:43:34 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  137.902525] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  9 09:43:34 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  137.964694] eth0: renamed from vethdc0004f
Aug  9 09:43:34 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  137.996769] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  9 09:43:34 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  137.997664] docker0: port 1(vetha379069) entered forwarding state
Aug  9 09:43:34 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  137.997681] docker0: port 1(vetha379069) entered forwarding state
Aug  9 09:43:34 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  137.997711] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  9 09:43:37 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpd[1782]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  9 09:43:37 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpd[1782]: Listen normally on 6 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  9 09:43:37 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpd[1782]: Listen normally on 7 docker0 fe80::42:13ff:fe49:7fe4 UDP 123
Aug  9 09:43:37 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpd[1782]: peers refreshed
Aug  9 09:43:37 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 ntpd[1782]: new interface(s) found: waking up resolver
Aug  9 09:43:49 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  153.052026] docker0: port 1(vetha379069) entered forwarding state
Aug  9 09:46:47 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  331.198560] docker0: port 1(vetha379069) entered disabled state
Aug  9 09:46:47 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  331.198617] vethdc0004f: renamed from eth0
Aug  9 09:46:47 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  331.252889] docker0: port 1(vetha379069) entered disabled state
Aug  9 09:46:47 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  331.254448] device vetha379069 left promiscuous mode
Aug  9 09:46:47 travis-job-b154856f-2e42-4daa-8d1e-af98e9375567 kernel: [  331.254450] docker0: port 1(vetha379069) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:0cde8738
---
travis_time:end:0a72a940:start=1533808007908704203,finish=1533808007914051419,duration=5347216
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1bc829ee
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:079c33f0
travis_time:start:079c33f0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:05a1a1c6
$ dmesg | grep -i kill
