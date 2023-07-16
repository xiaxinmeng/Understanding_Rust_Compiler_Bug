\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-size_of-cycle.rs","byte_start":0,"byte_end":0,"line_star[{"file_name":"/checkout/src/libcore/mem.rs","byte_start":10182,"byte_end":10208,"line_start":321,"line_end":321,"column_start":14,"column_end":40,"is_primary":true,"text":[{"text":"    unsafe { intrinsics::size_of::<T>() }","highlight_start":14,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when computing layout of `Foo`\n   |\nnote: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...\nnote: ...which requires const-evaluating `Foo::bytes::{{constant}}`...\n  --> /checkout/src/libcore/mem.rs:321:14\n   |\nLL |     unsafe { intrinsics::size_of::<T>() }\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = note: ...which again requires computing layout of `Foo`, completing the cycle\nnote: cycle used when const-evaluating `Foo::bytes::{{constant}}`\n  --> /checkout/src/libcore/mem.rs:321:14\n   |\nLL |     unsafe { intrinsics::size_of::<T>() }\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:52:43] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:52:43] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[00:52:43] ------------------------------------------
[00:52:43] 
[00:52:43] thread '[ui] ui/consts/const-size_of-cycle.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:52:43] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:52:43] 
[00:52:43] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:52:43] 
[00:52:43] 
[00:52:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-c0] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] kvm-clock: using sched offset of 1744232322 cycles
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] Zone ranges:
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000: [    0.000000] ACPI: IRQ9 used by override.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.tes)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] console [ttyS0] enabled
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.000000] tsc: D0.753557] Initializing cgroup subsys io
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.756193] Initializing cgroup subsys memory
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.758828] Initializing cgroup subsys devices
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.761379] Initializing cgroup subsys freezer
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.763941] Initializing cgroup subsys net_cls
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.767286] Initializing cgroup subsys perf_event
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.770431] Initializing cgroup subsys net_prio
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.773046] Initializing cgroup subsys hugetlb
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.775833] Initializing cgroup subsys pids
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.778352] CPU: Physical Processor ID: 0
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.781078] CPU: Processor Core ID: 0
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.783219] mce: CPU supports 32 MCE banks
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.785776] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.790459] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.797030] Freeing SMP alternatives memory: 32K
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.809916] ftrace: allocating 32185 entries in 126 pages
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.871998] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.875501] smpboot: Max logical packages: 2
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.878533] x2apic enabled
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.880824] Switched APIC routing to physical x2apic.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.886933] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    0.995538] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.000941] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.006491] x86: Booting SMP configuration:
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.008922] .... node  #0, CPUs:      #1
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.011977] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.018588]  #2
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.020298] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.026654]  #3
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.027937] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.033708] x86: Booted up 1 node, 4 CPUs
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.037139] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.043181] devtmpfs: initialized
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.048858] evm: security.selinux
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.051892] evm: security.SMACK64
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.054011] evm: security.SMACK64EXEC
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.056675] evm: security.SMACK64TRANSMUTE
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.059251] evm: security.SMACK64MMAP
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.061791] evm: security.ima
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.063802] evm: security.capability
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40ug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.142414] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.145611] ACPI: Added _OSI(Processor Aggregator Device)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.152484] ACPI: Executed 2 blocks of module-level executable AML code
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.179356] ACPI: Interpreter enabled
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.182001] ACPI: (supports S0 S3 S4 S5)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.184283] ACPI: Using IOAPIC for interrupt routing
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.187840] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.222323] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.226565] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.230842] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.234732] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e999424670xc03f]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.346006] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.368410] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.378531] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.388037] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.411379] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.417450] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.423630] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.429321] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.436753] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.459769] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.463015] vgaarb: loaded
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.464990] SCSI subsystem initialized
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99945379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.517281] pnp: PnP ACPI: found 7 devices
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.528055] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.533234] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.533237] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.533238] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.533240] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.533277] NET: Registered protocol family 2
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.536161] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.540994] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.545864] TCP: Hash tables configured (established 131072 bind 65536)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.549836] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.553703] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.558277] NET: Registered protocol family 1
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.561095] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.564388] PCI: CLS 0 bytes, default 64
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    1.564450] Unpacking initramfs...
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.650589] Freeing initrd memory: 21432K
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.653216] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.656817] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.663534] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.669426] hw unit of domain pp0-core 2^-0 Joules
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.672782] hw unit of domain package 2^-0 Joules
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.675829] hw unit of domain dram 2^-0 Joules
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.679577] Scanning fPI: Sleep Button [SLPF]
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.770113] GHES: HEST is not enabled!
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.775566] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.779443] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.789570] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.794258] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.805985] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.832363] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.859210] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.885437] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.912479] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    3.920058] Linux agpgart interface v0.103
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-83:MOU] at 0x60,0x64 irq 1,12
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.017007] i8042: Warning: Keylock active
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.020333] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.023000] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.026097] mousedev: PS/2 mouse device common for all mice
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.029518] rtc_cmos 00:00: RTC can wake from S4
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.032827] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.035816] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.038908] i2c /dev entries driver
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.041081] device-mapper: uevent: version 1.0.3
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.043694] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.047814] ledtrig-cpu: registered to indicate activity on CPUs
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.052481] NET: Registered protocol family 10
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.055977] NET: Registered protocol family 17
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.059548] Key type dns_resolver registered
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.063752] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.066683] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.070753] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.074557] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.077761] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.083623] registered taskstats version 1
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.086534] Loading compiled-in X.509 certificates
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.090205] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.096000] zswap: loaded using pool lzo/zbud
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.102039] Key type trusted registered
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40d6:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.228194] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.233795] scsi host0: Virtio SCSI HBA
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.248713] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.252691] AVX version of gcm_enc/dec engaged.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.256316] AES CTR mode by8 optimization enabled
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.326256] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.329996] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.335147] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.339469] sd 0:0:1:0: [sda] Write Protect is off
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.342049] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.342410] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.352171]  sda: sda1
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.356692] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.676120] tsc: Refined TSC clocksource calibration: 2600.000 MHz
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    4.679701] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3c3232d, max_idle_ns: 440795236700 ns
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    5.081198] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    7.224212] floppy0: no floppy controllers found
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    8.392026] raid6: sse2x1   gen()  8659 MB/s
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    8.460001] raid6: sse2x1   xor()  6456 MB/s
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    8.527993] raid6: sse2x2   gen() 10684 MB/s
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    8.596063] raid6: sse2x2   xor()  7066 MB/s
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    8.664098] raid6: sse2x4   gen() 12236 MB/s
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    8.732008] raid6: sse2x4   xor()  8625 MB/s
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    8.734652] raid6: using algorithm sse2x4 gen() 12236 MB/s
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40d2467 kernel: [    9.446871] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    9.510264] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [    9.751056] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   10.436542] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   10.592675] systemd-udevd[701]: starting version 204
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   10.739799] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   10.798574] intel_rapl: no valid rapl domains found in package 0
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   10.869085] ppdev: user-space parallel port driver
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   10.995791] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   11.056180] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   11.129725] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   11.300913] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   11.615293] random: mktemp: uninitialized urandom read (12 bytes read, 59 bits of entropy available)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   11.702619] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   11.790442] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   11.851438] EXT4-fs (sda1): resized filesystem to 7864064
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   12.108800] init: failsafe main process (1096) killed by TERM signal
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 instance-setup: INFO Running set_multiqueue.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 instance-setup: INFO Set channels for eth0 to 4.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 16 11:36:42 travis-job-05379ad1-f9a3-40dc-8683- is initialized
Aug 16 11:36:43 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 16 11:36:43 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Created user account henrik.
Aug 16 11:36:43 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Creating a new user account for emma.
Aug 16 11:36:43 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Created user account emma.
Aug 16 11:36:43 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Creating a new user account for igor.
Aug 16 11:36:43 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Created user account igor.
Aug 16 11:36:43 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 16 11:36:43 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Created user account konstantinhaase.
Aug 16 11:36:43 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Creating a new user account for aj.
Aug 16 11:36:43 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Created user account aj.
Aug 16 11:36:43 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Creating a new user account for solarce.
Aug 16 11:36:43 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Created user account solarce.
Aug 16 11:36:43 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Creating a new user account for asari.
Aug 16 11:36:43 travis-job-05379ad1-f9a3-40dcdc-8683-135e99942467 haveged: haveged starting up
Aug 16 11:36:44 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 11:36:44 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 11:36:44 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Creating a new user account for carmen.
Aug 16 11:36:44 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Created user account carmen.
Aug 16 11:36:44 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   13.350076] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 16 11:36:44 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Creating a new user account for maria.
Aug 16 11:36:44 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   13.362854] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 16 11:36:44 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Created user account maria.
Aug 16 11:36:44 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 google-accounts: INFO Removing user packer.
Aug 16 11:36:44 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   13.469934] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 16 11:36:44 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   13.473850] Bridge firewalling registered
Aug 16 11:36:44 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   13.484376] nf_conntrack version 0.5.0 (65536 buckets024 6e:4f:44:d7:2d:92:bd:50:33:cd:2f:d3:09:dd:73:25  root@travis-job-05379ad1-f9a3-40dc-8683-135e99942467 (DSA)
Aug 16 11:37:14 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 ec2: 256 89:7c:fa:ef:36:58:fd:dc:20:2e:c3:27:61:f9:9d:d9  root@travis-job-05379ad1-f9a3-40dc-8683-135e99942467 (ECDSA)
Aug 16 11:37:14 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 ec2: 256 6f:2d:1e:c6:02:a2:6e:c7:28:ca:37:a5:98:b5:9f:d6  root@travis-job-05379ad1-f9a3-40dc-8683-135e99942467 (ED25519)
Aug 16 11:37:14 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 ec2: 2048 ff:d9:94:02:61:66:a6:06:89:e4:de:f7:bf:66:eb:35  root@travis-job-05379ad1-f9a3-40dc-8683-135e99942467 (RSA)
Aug 16 11:37:14 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 16 11:37:14 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 ec2: #############################################################
Aug 16 11:37:45 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [   74.984677] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 16 11:39:37 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [  186.928058] device vethbdced3b entered promiscuous mode
Aug 16 11:39:37 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [  187.028605] cgroup: docker-runc (4953) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 16 11:39:37 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [  187.028608] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 16 11:39:37 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [  187.104345] eth0: renamed from veth9d5c71a
Aug 16 11:39:37 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [  187.150112] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 16 11:39:37 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [  187.151409] docker0: port 1(vethbdced3b) entered forwarding state
Aug 16 11:39:37 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [  187.151427] docker0: port 1(vethbdced3b) entered forwarding state
Aug 16 11:39:37 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [  187.151457] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 16 11:39:41 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 ntpd[1864]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 16 11:39:41 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 ntpd[1864]: Listen normally on 6 docker0 fe80::42:e7ff:fe71:15ed UDP 123
Aug 16 11:39:41 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 ntpd[1864]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 16 11:39:41 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 ntpd[1864]: peers refreshed
Aug 16 11:39:41 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 ntpd[1864]: new interface(s) found: waking up resolver
Aug 16 11:39:53 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [  202.179956] docker0: port 1(vethbdced3b) entered forwarding state
Aug 16 12:17:01 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 CRON[17072]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 16 12:30:29 travis-job-05379ad1-f9a3-40dc-8683-135e99942467 kernel: [ 3238.768077] docker0: port 1(vethbdced3b) entered disabled state
A8748 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
125256 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
122492 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
121660 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
112464 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
