plain
[00:49:01] ....................................................................................................
[00:49:03] ....................................................................................................
[00:49:06] ....................................................................................................
[00:49:09] ....................................................................................................
[00:49:12] ......iiiiiiiii.....................................................................................
[00:49:18] ....................................................................................................
[00:49:22] ...........i........................................................................................
[00:49:25] ....................i...............................................................................
[00:49:28] ....................................................................................................
---
[01:19:06] 
[01:19:06] failures:
[01:19:06] 
[01:19:06] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0712 (line 11385) stdout ----
[01:19:06] error[E0597]: borrowed value does not live long enough
[01:19:06]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11392:14
[01:19:06]    |
[01:19:06] 8  |     let a = &FOO; // error: thread-local variable borrowed past end of function
[01:19:06]    |              ^^^ temporary value does not live long enough
[01:19:06] 13 | }
[01:19:06] 13 | }
[01:19:06]    | - temporary value only lives until here
[01:19:06]    |
[01:19:06]    = note: borrowed value must be valid for the static lifetime...
[01:19:06] 
[01:19:06] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0712 (line 11385)' panicked at 'Some expected error codes were not found: ["E0627"]', librustdoc/test.rs:338:9
[01:19:06] 
[01:19:06] 
[01:19:06] failures:
[01:19:06]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0712 (line 11385)
---
[01:19:06] 
[01:19:06] 
[01:19:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:06] Build completed unsuccessfully in 0:32:54
[01:19:06] make: *** [check] Error 1
[01:19:06] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0710e830
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:127ac660
$ sudo tail -n 500 /var/log/syslog
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] Using GB pages for direct mapping
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] ACPI: RSDP 0x00000000000F27D0 000014 (v00 Google)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  -job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000]   Device   empty
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] Movable zone start for each node
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] Early memory node ranges
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] Policy zone: Normal
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9R_IRQS:33024 nr_irqs:456 16
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] console [ttyS0] enabled
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.338379] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.339795] pid_max: default: 32768 minimum: 301
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.340554] ACPI: Core revision 20150930
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.346802] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.347926] Security Framework initialized
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.348547] Yama: becoming mindful.
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.349066] AppArmor: AppArmor disabled by boot time parameter
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.352766] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.362266] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.367952] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.369288] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.371799] Initializing cgroup subsys io
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.372730] Initializing cgroup subsys memory
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.373639] Initializing cgroup subsys devices
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.374659] Initializing cgroup subsys freezer
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.375336] Initializing cgroup subsys net_cls
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.376228] Initializing cgroup subsys perf_event
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.377206] Initializing cgroup subsys net_prio
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.378182] Initializing cgroup subsys hugetlb
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.378810] Initializing cgroup subsys pids
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.379641] CPU: Physical Processor ID: 0
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.380438] CPU: Processor Core ID: 0
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.381113] mce: CPU supports 32 MCE banks
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.382111] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.383557] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.386859] Freeing SMP alternatives memory: 32K
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.395453] ftrace: allocating 32185 entries in 126 pages
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.441806] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.443206] smpboot: Max logical packages: 2
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.444544] x2apic enabled
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.446453] Switched APIC routing to physical x2apic.
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.450537] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.557555] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.559354] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.561843] x86: Booting SMP configuration:
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.562531] .... node  #0, CPUs:      #1
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.563382] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.566952]  #2
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.567413] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.571029]  #3
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.571537] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.575234] x86: Booted up 1 node, 4 CPUs
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.576004] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.578399] devtmpfs: initialized
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.583185] evm: security.selinux
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.583786] evm: security.SMACK64
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.584655] evm: security.SMACK64EXEC
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.585320] evm: security.SMACK64TRANSMUTE
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.585994] evm: security.SMACK64MMAP
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.586544] evm: security.ima
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.587029] evm: security.capability
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.587881] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.589440] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.590911] pinctrl core: initialized pinctrl subsystem
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.592022] RTC time:  8:26:56, date: 08/07/18
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.593602] NET: Registered protocol family 16
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.605587] cpuidle: using governor ladder
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.617591] cpuidle: using governor menu
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.618308] PCCT header not found.
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.618929] ACPI: bus type PCI registered
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.619520] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.620553] PCI: Using configuration type 1 for base access
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.634760] ACPI: Added _OSI(Module Device)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.635641] ACPI: Added _OSI(Processor Device)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.636343] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.637073] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.640522] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.663590] ACPI: Interpreter enabled
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.664348] ACPI: (supports S0 S3 S4 S5)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.665107] ACPI: Using IOAPIC for interrupt routing
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.666099] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.695580] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.696566] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.697926] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.699121] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.701617] PCI host bridge to bus 0000:00
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.702238] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.703567] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.704825] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.705907] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.707036] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.708032] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.708444] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.723605] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.739105] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.740701] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.747004] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.752156] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.766183] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.772270] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.777074] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.792283] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.794539] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.796726] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.799264] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.801480] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.821480] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.822724] vgaarb: loaded
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.823492] SCSI subsystem initialized
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.824260] libata version 3.00 loaded.
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.824284] ACPI: bus type USB registered
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.824968] usbcore: registered new interface driver usbfs
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.825856] usbcore: registered new interface driver hub
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.826712] usbcore: registered new device driver usb
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.827665] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.828761] dmi: Firmware registration failed.
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.829636] PCI: Using ACPI for IRQ routing
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.830300] PCI: pci_cache_line_size set to 64 bytes
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.830393] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.830395] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.830523] NetLabel: Initializing
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.831090] NetLabel:  domain hash size = 128
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.831839] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.832984] NetLabel:  unlabeled traffic allowed by default
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.833972] amd_nb: Cannot enumerate AMD northbridges
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.834843] clocksource: Switched to clocksource kvm-clock
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.842885] pnp: PnP ACPI init
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.843487] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.843562] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.843607] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.843657] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.843704] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.843746] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.843787] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.843953] pnp: PnP ACPI: found 7 devices
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.851749] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.853144] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.853146] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.853148] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.853149] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.853189] NET: Registered protocol family 2
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.854194] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    0.855756] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 08:27:06 travis-s
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.865255] hw unit of domain package 2^-0 Joules
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.865954] hw unit of domain dram 2^-0 Joules
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.866801] Scanning for low memory corruption every 60 seconds
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.868357] audit: initializing netlink subsys (disabled)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.869339] audit: type=2000 audit(1533630418.818:1): initialized
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.870581] Initialise system trusted keyring
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.871831] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.872880] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.875120] zbud: loaded
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.875801] VFS: Disk quotas dquot_6.6.0
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.876522] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.877834] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f k7:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.894377] ACPI: Power Button [PWRF]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.895036] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.896267] ACPI: Sleep Button [SLPF]
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.897396] GHES: HEST is not enabled!
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.899850] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.901475] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.906332] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.907663] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.913073] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.935808] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.959819] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    2.984320] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.008715] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.013599] Linux agpgart interface v0.103
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.017281] loop: module loaded
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.018005] libphy: Fixed MDIO Bus: probed
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.019399] tun: Universal TUN/TAP device driver, 1.6
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.020593] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.056299] PPP generic driver version 2.4.2
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.057751] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.059998] ehci-pci: EHCI PCI platform driver
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.061542] ehci-platform: EHCI generic platform driver
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.063316] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.065341] ohci-pci: OHCI PCI platform driver
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.066373] ohci-platform: OHCI generic platform driver
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.068220] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.070563] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.073131] i8042: Warning: Keylock active
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.075320] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.077000] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.078786] mousedev: PS/2 mouse device common for all mice
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.080880] rtc_cmos 00:00: RTC can wake from S4
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.082770] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.084655] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.086373] i2c /dev entries driver
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.087398] device-mapper: uevent: version 1.0.3
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.089041] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.091859] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.094406] NET: Registered protocol family 10
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.095814] NET: Registered protocol family 17
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.097327] Key type dns_resolver registered
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.098994] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.100763] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.101912] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.103133] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.104213] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.106745] registered taskstats version 1
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.107529] Loading compiled-in X.509 certificates
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.109432] Loaded X.509 cert 'Build time aude9-44dc-bba8-f6f21f652c9f kernel: [    3.132627] Freeing unused kernel memory: 1956K
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.133750] Freeing unused kernel memory: 92K
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.147905] systemd-udevd[118]: starting version 204
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.198767] scsi host0: Virtio SCSI HBA
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.203209] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.212374] AVX version of gcm_enc/dec engaged.
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.213481] AES CTR mode by8 optimization enabled
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.249242] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.249275] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.249277] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.249529] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.249531] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    3.249688] sd 0:0:1:0: [sda-bba8-f6f21f652c9f kernel: [    7.916039] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    8.104918] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    8.215044] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    8.262210] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    8.549255] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    9.044286] random: cloud-init: uninitialized urandom read (32 bytes read, 46 bits of entropy available)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    9.162752] systemd-udevd[701]: starting version 204
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    9.257121] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    9.330466] intel_rapl: no valid rapl domains found in package 0
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    9.386734] ppdev: user-space parallel port driver
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    9.460570] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    9.504221] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    9.559716] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    9.721494] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [    9.961059] random: mktemp: uninitialized urandom read (12 bytes read, 61 bits of entropy available)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [   10.029389] random: mktemp: uninitialized urandom read (6 bytes read, 62 bits of entropy available)
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [   10.096552] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [   10.132431] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [   10.550952] init: failsafe main process (1093) killed by TERM signal
Aug  7 08:27:06 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f instance-se Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f google-accounts: INFO Starting Google Accounts daemon.
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [   11.251724] random: nonblocking pool is initialized
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f google-accounts: INFO Creating a new user account for me.
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f google-accounts: INFO Created user account me.
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f google-accounts: INFO Creating a new user account for bogdana.
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f google-accounts: INFO Created user account bogdana.
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f google-accounts: INFO Creating a new user account for aj.
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f google-accounts: INFO Created user account aj.
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f google-accounts: INFO Creating a new user account for asari.
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f google-accounts: INFO Created user account asari.
Aug  7 08:27:07 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f google-accounts: INFO Removing user packer.
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f cron[1434]: (CRON) INFO (pidfile fd = 3)
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f cron[1476]: (CRON) STARTUP (fork ok)
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f cron[1476]: (CRON) INFO (Running @reboot jobs)
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f acpid: starting up with netlink and the input layer
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f acpid: 1 rule loaded
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f acpid: waiting for events: event logging is off
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f haveged: haveged starting up
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [   11.781041] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [   11.793720] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [   11.889804] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [   11.893838] Bridge firewalling registered
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [   11.905542] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [   11.972246] Initializing XFRM netlink socket
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [   11.978927] Netfilter messages via NETLINK v0.30.
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [   11.981443] ctnetlink v0.93: registering with nfnetlink.
Aug  7 08:27:08 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [   12.338988] floppy0: no floppy controllers found
Aug  7 08:27:31 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpdate[1779]: adjust time server 169.254.169.254 offset 0.005431 sec
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpd[1814]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpd[1815]: proto: precision = 0.117 usec
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpd[1815]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpd[1815]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpd[1815]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpd[1815]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpd[1815]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpd[1815]: Listen normally on 3 eth0 10.20.255.13 UDP 123
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpd[1815]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpd[1815]: peers refreshed
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpd[1815]: Listening on routing socket on fd #21 for interface updates
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [   41.936022] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f startup-script: INFO Found startup-script in metadata.
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f startup-script: INFO startup-script: job 1 at Tue Aug  7 11:37:00 2018
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f startup-script: INFO startup-script: Return code 0.
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f startup-script: INFO startup-script: Return code 0.
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f startup-script: INFO Finished running startup scripts.
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ec2: 
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ec2: #############################################################
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ec2: 1024 5e:36:1d:a5:31:e6:41:c4:16:13:a6:4c:8b:52:0c:9f  root@travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f (DSA)
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ec2: 256 44:4a:64:63:41:d9:e3:a3:06:fc:77:1b:eb:b5:2c:b3  root@travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f (ECDSA)
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ec2: 256 b6:e1:4f:fb:4b:f9:51:7f:a6:c4:8a:52:82:d4:42:0f  root@travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f (ED25519)
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ec2: 2048 4d:6f:57:58:cf:71:39:fa:35:f5:bb:bd:d6:cd:eb:eb  root@travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f (RSA)
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 08:27:38 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ec2: #############################################################
Aug  7 08:28:58 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [  122.165911] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 08:30:00 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [  184.458171] device vethf70b42a entered promiscuous mode
Aug  7 08:30:00 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [  184.458263] docker0: port 1(vethf70b42a) entered forwarding state
Aug  7 08:30:00 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [  184.458272] docker0: port 1(vethf70b42a) entered forwarding state
Aug  7 08:30:00 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [  184.458676] docker0: port 1(vethf70b42a) entered disabled state
Aug  7 08:30:00 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [  184.557066] cgroup: docker-runc (4803) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 08:30:00 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [  184.557069] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 08:30:00 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [  184.636634] eth0: renamed from veth1461511
Aug  7 08:30:00 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [  184.683055] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 08:30:00 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [  184.684193] docker0: port 1(vethf70b42a) entered forwarding state
Aug  7 08:30:00 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [  184.684209] docker0: port 1(vethf70b42a) entered forwarding state
Aug  7 08:30:00 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [  184.684233] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 08:30:04 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpd[1815]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  7 08:30:04 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpd[1815]: Listen normally on 6 docker0 fe80::42:ceff:fe81:9ed4 UDP 123
Aug  7 08:30:04 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpd[1815]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 08:30:04 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpd[1815]: peers refreshed
Aug  7 08:30:04 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f ntpd[1815]: new interface(s) found: waking up resolver
Aug  7 08:30:15 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [  199.724261] docker0: port 1(vethf70b42a) entered forwarding state
Aug  7 09:17:01 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f CRON[26463]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  7 09:18:37 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [ 3101.588257] traps: a[5571] trap invalid opcode ip:55a237b35c1b sp:7ffc6c58c4f0 error:0 in a[55a237b32000+6000]
Aug  7 09:18:53 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [ 3116.884670] traps: a[8542] trap invalid opcode ip:7fb755fdbef1 sp:7fffe3f163a0 error:0 in libstd-e054c7a28f8831a7.so[7fb755f80000+172000]
Aug  7 09:18:53 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [ 3116.921626] traps: a[8544] trap invalid opcode ip:7ff502593ef1 sp:7ffde4cc6b90 error:0 in libstd-e054c7a28f8831a7.so[7ff502538000+172000]
Aug  7 09:20:20 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [ 3204.239209] traps: a[24339] trap invalid opcode ip:55c85777ae68 sp:7ffddd094180 error:0 in a[55c857778000+4000]
Aug  7 09:23:13 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [ 3377.763489] a[22146]: segfault at 0 ip 0000562b6a678658 sp 00007ffdca6f5f90 error 6 in a[562b6a675000+5000]
Aug  7 09:23:23 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [ 3387.673460] a[22945]: segfault at 1 ip 0000563d5d194c6c sp 00007ffc963f1a00 error 6 in a[563d5d192000+4000]
Aug  7 09:23:28 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [ 3392.039347] traps: a[23360] trap invalid opcode ip:55b767a265bc sp:7ffd86da4e50 error:0 in a[55b767a23000+7000]
Aug  7 09:48:05 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [ 4869.343059] docker0: port 1(vethf70b42a) entered disabled state
Aug  7 09:48:05 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [ 4869.343128] veth1461511: renamed from eth0
Aug  7 09:48:05 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [ 4869.405507] docker0: port 1(vethf70b42a) entered disabled state
Aug  7 09:48:05 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [ 4869.407213] device vethf70b42a left promiscuous mode
Aug  7 09:48:05 travis-job-8b00f5ac-4de9-44dc-bba8-f6f21f652c9f kernel: [ 4869.407216] docker0: port 1(vethf70b42a) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:01a3f498
