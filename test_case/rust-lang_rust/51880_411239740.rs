plain
[00:45:25] ....................................................................................................
[00:45:28] ....................................................................................................
[00:45:30] ....................................................................................................
[00:45:33] ....................................................................................................
[00:45:36] ............iiiiiiiii...............................................................................
[00:45:42] ....................................................................................................
[00:45:46] .................i..................................................................................
[00:45:49] ..........................i.........................................................................
[00:45:52] ....................................................................................................
---
[00:52:30] ...................................................................................i................
[00:52:33] .............................i......................................................................
[00:52:38] ....................................................................................................
[00:52:42] ............................................................i.......................................
[00:52:45] ........................................F..............i..ii........................................
[00:52:52] ....................................................................................................
[00:52:55] ....................................................................................................
[00:52:58] .......................................i............................................................
[00:53:01] ....................................................................................................
[00:53:01] ....................................................................................................
[00:53:05] .................F..................................................................................
[00:53:10] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:53:10] ......................................................................................
[00:53:10] failures:
[00:53:10] 
---
[00:53:10] error: /checkout/src/test/compile-fail/method-call-lifetime-args.rs:28: expected error not found: expected at most 2 lifetime parameters, found 3 lifetime parameters
[00:53:10] 
[00:53:10] error: 2 unexpected errors found, 2 expected errors not found
[00:53:10] status: exit code: 1
[00:53:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/method-call-lifetime-args.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/method-call-lifetime-args/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/method-call-lifetime-args/auxiliary" "-A" "unused"
[00:53:10]     Error {
[00:53:10]         line_num: 26,
[00:53:10]         kind: Some(
[00:53:10]             Error
---
[00:53:10] thread '[compile-fail] compile-fail/method-call-lifetime-args.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1285:13
[00:53:10] 
[00:53:10] ---- [compile-fail] compile-fail/trait-test-2.rs stdout ----
[00:53:10] 
[00:53:10] error: /checkout/src/test/compile-fail/trait-test-2.rs:18: unexpected error: '18:8: 18:11: wrong number of type arguments: expected 0, found 1 [E0244]'
[00:53:10] 
[00:53:10] error: /checkout/src/test/compile-fail/trait-test-2.rs:19: unexpected error: '19:8: 19:12: wrong number of type arguments: expected 1, found 2 [E0244]'
[00:53:10] 
[00:53:10] error: /checkout/src/test/compile-fail/trait-test-2.rs:18: expected error not found: expected at most 0 type parameters, found 1 type parameter
[00:53:10] 
[00:53:10] error: /checkout/src/test/compile-fail/trait-test-2.rs:19: expected error not found: expected at most 1 type parameter, found 2 type parameters
[00:53:10] error: 2 unexpected errors found, 2 expected errors not found
[00:53:10] status: exit code: 1
[00:53:10] status: exit code: 1
[00:53:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/trait-test-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/trait-test-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/trait-test-2/auxiliary" "-A" "unused"
[00:53:10]     Error {
[00:53:10]         line_num: 18,
[00:53:10]         kind: Some(
[00:53:10]             Error
[00:53:10]             Error
[00:53:10]         ),
[00:53:10]         msg: "18:8: 18:11: wrong number of type arguments: expected 0, found 1 [E0244]"
[00:53:10]     Error {
[00:53:10]         line_num: 19,
[00:53:10]         kind: Some(
[00:53:10]             Error
[00:53:10]             Error
[00:53:10]         ),
[00:53:10]         msg: "19:8: 19:12: wrong number of type arguments: expected 1, found 2 [E0244]"
[00:53:10] ]
[00:53:10] 
[00:53:10] not found errors (from test file): [
[00:53:10]     Error {
[00:53:10]     Error {
[00:53:10]         line_num: 18,
[00:53:10]         kind: Some(
[00:53:10]             Error
[00:53:10]         ),
[00:53:10]         msg: "expected at most 0 type parameters, found 1 type parameter"
[00:53:10]     Error {
[00:53:10]         line_num: 19,
[00:53:10]         kind: Some(
[00:53:10]             Error
[00:53:10]             Error
[00:53:10]         ),
[00:53:10]         msg: "expected at most 1 type parameter, found 2 tTue, 07 Aug 2018 23:57:28 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_fold:start:after_failure.1
travis_time:start:03ce4a80
$ sudo tail -n 500 /var/log/syslog
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] kvm-clock: using sched offset of 1428015734 cycles
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] Zone ranges:
Au5647 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Au105-bf04-57109ca95647 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] console [ttyS0] enabled
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.313698] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.314869] pid_max: default: 32768 minimum: 301
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.315624] ACPI: Core revision 20150930
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.323129] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.324660] Security Framework initialized
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.325284] Yama: becoming mindful.
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.326086] AppArmor: AppArmor disabled by boot time parameter
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.328863] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.338361] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.343237] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.344266] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.345611] Initializing cgroup subsys io
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.346187] Initializing cgroup subsys memory
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.346818] Initializing cgroup subsys devices
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.347462] Initializing cgroup subsys freezer
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.348078] Initializing cgroup subsys net_cls
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.348690] Initializing cgroup subsys perf_event
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.349538] Initializing cgroup subsys net_prio
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.350166] Initializing cgroup subsys hugetlb
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.350776] Initializing cgroup subsys pids
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.351439] CPU: Physical Processor ID: 0
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.352011] CPU: Processor Core ID: 0
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.352555] mce: CPU supports 32 MCE banks
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.353295] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.354218] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.356996] Freeing SMP alternatives memory: 32K
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.365937] ftrace: allocating 32185 entries in 126 pages
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.413956] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.415206] smpboot: Max logical packages: 2
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.416454] x2apic enabled
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.418418] Switched APIC routing to physical x2apic.
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.422329] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.528967] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.530766] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.534061] x86: Booting SMP configuration:
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.534707] .... node  #0, CPUs:      #1
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.535926] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.539230]  #2
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.539837] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.543145]  #3
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.543779] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.546983] x86: Booted up 1 node, 4 CPUs
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.547808] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.550198] devtmpfs: initialized
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.555008] evm: security.selinux
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.555777] evm: security.SMACK64
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.556324] evm: security.SMACK64EXEC
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.556874] evm: security.SMACK64TRANSMUTE
Aug  7 09ca95647 kernel: [    0.710287] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.712078] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.718289] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.723675] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.737194] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.742523] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.747289] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.760797] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.763136] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.765323] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.767470] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.769611] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.790349] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.791502] vgaarb: loaded
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.792095] SCSI subsystem initialized
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.792753] libata version 3.00 loaded.
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.792774] ACPI: bus type USB registered
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.793373] usbcore: registered new interface driver usbfs
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.794168] usbcore: registered new interface driver hub
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.794963] usbcore: registered new device driver usb
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.795875] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.796937] dmi: Firmware registration failed.
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.797791] PCI: Using ACPI for IRQ routing
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.798484] PCI: pci_cache_line_size set to 64 bytes
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.798591] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.798594] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.798730] NetLabel: Initializing
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.799294] NetLabel:  domain hash size = 128
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.799907] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.800647] NetLabel:  unlabeled traffic allowed by default
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.801566] amd_nb: Cannot enumerate AMD northbridges
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.802315] clocksource: Switched to clocksource kvm-clock
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.810303] pnp: PnP ACPI init
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.811008] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.811084] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.811128] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.811181] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.811226] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.811267] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.811310] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.811496] pnp: PnP ACPI: found 7 devices
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.819032] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.820689] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.820693] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.820695] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.820697] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.820742] NET: Registered protocol family 2
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.821679] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    0.823869] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug/input0
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    2.993302] ACPI: Power Button [PWRF]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    2.994013] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    2.995239] ACPI: Sleep Button [SLPF]
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    2.996268] GHES: HEST is not enabled!
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    2.999013] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.000004] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.004851] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.005776] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.010766] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.033618] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.057935] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.081127] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.105044] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.109092] Linux agpgart interface v0.103
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.113199] loop: module loaded
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.114229] libphy: Fixed MDIO Bus: probed
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.115240] tun: Universal TUN/TAP device driver, 1.6
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.116413] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.151356] PPP generic driver version 2.4.2
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.153024] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.154377] ehci-pci: EHCI PCI platform driver
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.155554] ehci-platform: EHCI generic platform driver
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.156986] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.158829] ohci-pci: OHCI PCI platform driver
Aug  7 23:02:24 travis-job-0a4bff32-e075-4mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.180019] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.182156] NET: Registered protocol family 10
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.183317] NET: Registered protocol family 17
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.184354] Key type dns_resolver registered
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.185832] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.187245] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.188757] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.190257] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.191811] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.194067] registered taskstats version 1
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.195096] Loading compiled-in X.509 certificates
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.196616] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.199062] zswap: loaded using pool lzo/zbud
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.201587] Key type trusted registered
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.205775] Key type encrypted registered
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.206642] ima: No TPM chip found, activating TPM-bypass!
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.207836] evm: HMAC attrs: 0x1
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.208746]   Magic number: 14:938:48
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.209612] memory memory80: hash matches
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.210648] rtc_cmos 00:00: setting system clock to 2018-08-07 23:02:17 UTC (1533682937)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.212887] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.214216] EDD information not available.
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.215300] PM: Hibernation image not present or could not be loaded.
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.217074] Freeing unused kernel memory: 1496K
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.217845] Write protecting the kernel read-only data: 14336k
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.219511] Freeing unused kernel memory: 1956K
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.220440] Freeing unused kernel memory: 92K
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.234893] systemd-udevd[119]: starting version 204
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.302865] scsi host0: Virtio SCSI HBA
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.310505] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.314916] AVX version of gcm_enc/dec engaged.
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.315924] AES CTR mode by8 optimization enabled
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.347303] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.348248] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.349320] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.351269] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.352008] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.352177] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.355250]  sda: sda1
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.356337] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.374736] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.966431] tsc: Refined TSC clocksource calibration: 2599.801 MHz
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    3.967585] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2579804ba50, max_idle_ns: 440795283714 ns
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    4.207481] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    6.282487] floppy0: no floppy controllers found
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    7.446346] raid6: sse2x1   gen()  9079 MB/s
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    7.514346] raid6: sse2x1   xor()  6618 MB/s
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    7.582361] raid6: sse2x2   gen() 11273 MB/s
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    7.650412] raid6: sse2x2   xor()  7771 MB/s
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    7.718352] raid6: sse2x4   gen() 11621 MB/s
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    7.786367] raid6: sse2x4   xor()  8542 MB/s
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    7.787244] raid6: using algorithm sse2x4 gen() 11621 MB/s
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    7.788139] raid6: .... xor() 8542 MB/s, rmw enabled
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    7.789098] raid6: using ssse3x2 recovery algorithm
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    7.792207] xor: automatically using best checksumming function:
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    7.830335]    avx       : 22225.000 MB/sec
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    7.844582] Btrfs loaded
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    7.885687] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    7.886787] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    7.972739] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 23:02:24 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [    7.979568] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 23:02:24 travis-job-0a4NFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Starting Google Accounts daemon.
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Creating a new user account for me.
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [   11.139439] random: nonblocking pool is initialized
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Created user account me.
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Creating a new user account for henrik.
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Created user account henrik.
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Creating a new user account for emma.
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Created user account emma.
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Creating a new user account for igor.
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Created user account igor.
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  7 23:02:25 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Created user accouravis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Created user account asari.
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [   11.643056] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Creating a new user account for bogdana.
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [   11.655906] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Created user account bogdana.
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Creating a new user account for konstantin.
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Created user account konstantin.
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Creating a new user account for carmen.
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Created user account carmen.
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Creating a new user account for maria.
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [   11.850656] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [   11.853725] Bridge firewalling registered
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Created user account maria.
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [   11.864591] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 google-accounts: INFO Removing user packer.
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [   11.925494] Initializing XFRM netlink socket
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [   11.933300] Netfilter messages via NETLINK v0.30.
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [   11.936465] ctnetlink v0.93: registering with nfnetlink.
Aug  7 23:02:26 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [   12.362483] floppy0: no floppy controllers found
Aug  7 23:02:49 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ntpdate[1847]: adjust time server 169.254.169.254 offset 0.005993 sec
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ntpd[1882]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ntpd[1883]: proto: precision = 0.101 usec
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ntpd[1883]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ntpd[1883]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ntpd[1883]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ntpd[1883]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ntpd[1883]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ntpd[1883]: Listen normally on 3 eth0 10.20.2.130 UDP 123
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ntpd[1883]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ntpd[1883]: peers refreshed
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ntpd[1883]: Listening on routing socket on fd #21 for interface updates
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [   41.832257] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 startup-script: INFO Found startup-script in metadata.
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 startup-script: INFO startup-script: job 1 at Wed Aug  8 02:12:00 2018
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 startup-script: INFO startup-script: Return code 0.
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 startup-script: INFO startup-script: Return code 0.
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 startup-script: INFO Finished running startup scripts.
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ec2: 
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ec2: #############################################################
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ec2: 1024 5d:03:4e:ec:02:1d:28:61:82:f3:5c:40:d6:a1:55:9a  root@travis-job-0a4bff32-e075-4105-bf04-57109ca95647 (DSA)
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ec2: 256 d2:82:5d:0e:c5:1b:47:11:b3:76:b9:0e:27:99:db:7a  root@travis-job-0a4bff32-e075-4105-bf04-57109ca95647 (ECDSA)
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ec2: 256 d8:34:d4:a0:ec:28:e1:b9:35:d0:42:18:67:a3:66:ae  root@travis-job-0a4bff32-e075-4105-bf04-57109ca95647 (ED25519)
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ec2: 2048 d8:92:98:4d:27:58:5a:78:23:b6:c3:76:76:b8:40:fe  root@travis-job-0a4bff32-e075-4105-bf04-57109ca95647 (RSA)
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 23:02:56 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ec2: #############################################################
Aug  7 23:04:15 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [  121.484374] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 23:05:28 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [  194.228126] device veth3ccc8ee entered promiscuous mode
Aug  7 23:05:28 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [  194.319259] cgroup: docker-runc (4875) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 23:05:28 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [  194.319263] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 23:05:28 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [  194.390681] eth0: renamed from veth6e8187f
Aug  7 23:05:28 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [  194.432374] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 23:05:28 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [  194.433695] docker0: port 1(veth3ccc8ee) entered forwarding state
Aug  7 23:05:28 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [  194.433721] docker0: port 1(veth3ccc8ee) entered forwarding state
Aug  7 23:05:28 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 kernel: [  194.433748] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 23:05:32 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ntpd[1883]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  7 23:05:32 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ntpd[1883]: Listen normally on 6 docker0 fe80::42:88ff:fee1:e5b7 UDP 123
Aug  7 23:05:32 travis-job-0a4bff32-e075-4105-bf04-57109ca95647 ntpd[1883]: Listen normally on 7 dotive/jemalloc/lib
34496 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build
34344 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects
34336 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects/pack
34196 ./obj/build/x86_64-unknown-linux-gnu/doc/core/arch
---
travis_time:end:1bbdba30:start=1533686249994643836,finish=1533686250004859979,duration=10216143
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1df9195d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2da6e480
travis_time:start:2da6e480
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:16fb5cb0
$ dmesg | grep -i kill
