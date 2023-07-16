\n# struct TheDarkKnight;\n# impl TheDarkKnight { fn nothing_is_true(self) {} }\n# struct Batcave { knirendered":null},{"message":"consider removing the `*`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/nll/move-errors.rs","byte_start":2201,"byte_end":2203,"line_start":123,"line_end":123,"column_start":11,"column_end":13,"is_primary":true,"text":[{"text":"    match *x {","highlight_start":11,"highlight_end":13}],"label":null,"suggested_replacement":"x","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0507]: cannot move out of borrowed content\n  --> /checkout/src/test/ui/nll/move-errors.rs:123:11\n   |\nLL |     match *x {\n   |           ^^\n   |           |\n   |           cannot move out of borrowed content\n   |           help: consider removing the `*`: `x`\nLL |     //~^ ERROR\nLL |         Ok(s) | Err(s) => (),\n   |            - data moved here\n   |\nnote: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait\n  --> /checkout/src/test/ui/nll/move-errors.rs:125:12\n   |\nLL |         Ok(s) | Err(s) => (),\n   |            ^\n\n"}
[00:49:50] {"message":"aborting due to 14 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 14 previous errors\n\n"}
[00:49:50] {"message":"Some errors occurred: E0507, E0508, E0509.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0507, E0508, E0509.\n"}
[00:49:50] {"message":"For more information about an error, try `rustc --explain E0507`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0507`.\n"}
[00:49:50] ------------------------------------------
[00:49:50] 
[00:49:50] thread '[ui] ui/nll/move-errors.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:49:50] 
[00:49:50] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:49:50] 
[00:49:50] 
[00:49:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" is-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x001b156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] Movable zone start for each node
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] Early memory node ranges
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000]   Normal zone: 49152 pages used9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] Policy zone: Normal
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a0156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.662231] Initializing cgroup subsys devices
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.664892] Initializing cgroup subsys freezer
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.668003] Initializing cgroup subsys net_cls
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.671203] Initializing cgroup subsys perf_event
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.673429] Initializing cgroup subsys net_prio
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.675941] Initializing cgroup subsys hugetlb
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.678383] Initializing cgroup subsys pids
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.680531] CPU: Physical Processor ID: 0
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.682656] CPU: Processor Core ID: 0
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.685370] mce: CPU supports 32 MCE banks
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.687760] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.690609] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    0.696213] Freeing SMP alternatives memory: 32K
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081ac7ea2 kernel: [    1.027603] ACPI: Added _OSI(Processor Aggregator Device)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.033453] ACPI: Executed 2 blocks of module-level executable AML code
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.059989] ACPI: Interpreter enabled
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.062824] ACPI: (supports S0 S3 S4 S5)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.064824] ACPI: Using IOAPIC for interrupt routing
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.067600] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.101929] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.105309] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.108800] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.112271] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.118838] PCI host bridge to bus 0000:00
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.120991] pci_bus 0000:00: root bus resource [io ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.236009] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.246667] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.255348] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.280299] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.285995] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.291238] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.296845] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.302150] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.324050] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.327643] vgaarb: loaded
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.329634] SCSI subsystem initialized
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.332722] libata version 3.00 loaded.
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.332749] ACPI: bus type USB registered
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.335187] usbcore: registered new interface driver usbfs
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.337654] usbcore: registered new interface driver hub
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.340472] usbcore: registered new device driver usb
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.344674] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.349662] dmi: Firmware registration failed.
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.352568] PCI: Using ACPI for IRQ routing
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.354615] PCI: pci_cache_line_size set to 64 bytes
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.354719] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.354721] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.354839] NetLabel: Initializing
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.356590] NetLabel:  domain hash size = 128
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.359044] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.361912] NetLabeksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.395172] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.395175] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.395177] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.395178] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.395216] NET: Registered protocol family 2
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.397793] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.401623] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.405269] TCP: Hash tables configured (established 131072 bind 65536)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.408526] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.411614] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.415690] NET: Registered protocol family 1
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.417551] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.420254] PCI: CLS 0 bytes, default 64
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    1.420326] Unpacking initramfs...
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.460112] Freeing initrd memory: 21432K
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.462463] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.465630] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.471029] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.475758] hw unit of domain pp0-core 2^-0 Joules
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.478113] hw unit of domain package 2^-0 Joules
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.480334] hw unit of domain dram 2^-16 Joules
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.482794] Scanning for low memory corruption every 60 seconds
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.486716] audit: initializing netlink subsys (disabled)
Aug ug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.715628] tun: Universal TUN/TAP device driver, 1.6
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.718382] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.772388] PPP generic driver version 2.4.2
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.774927] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.780072] ehci-pci: EHCI PCI platform driver
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.782962] ehci-platform: EHCI generic platform driver
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.786805] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.790529] ohci-pci: OHCI PCI platform driver
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.793764] ohci-platform: OHCI generic platform driver
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.796983] uhci_hcd: USB Universal Host Controller Interface driver
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.800478] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.804782] i8042: Warning: Keylock active
Aug 13 10:34:27 travis-job-b9ab1ey type dns_resolver registered
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.850459] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.853596] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.856697] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.860444] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.863495] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.869230] registered taskstats version 1
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.872002] Loading compiled-in X.509 certificates
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.875139] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.880797] zswap: loaded using pool lzo/zbud
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.887136] Key type trusted registered
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.895318] Key type encrypted registered
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    3.897365] ima: No TPM chip foul: [    4.089458] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    4.480309] tsc: Refined TSC clocksource calibration: 2300.006 MHz
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    4.483824] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x21273ac4d85, max_idle_ns: 440795266965 ns
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    4.870853] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    7.000453] floppy0: no floppy controllers found
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    8.180191] raid6: sse2x1   gen()  8761 MB/s
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    8.248158] raid6: sse2x1   xor()  6514 MB/s
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    8.316130] raid6: sse2x2   gen() 10850 MB/s
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    8.384122] raid6: sse2x2   xor()  7126 MB/s
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    8.452132] raid6: sse2x4   gen() 12576 MB/s
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    8.520129] raid6: sse2x4   xor()  8761 MB/s
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    8.588121] raid6: avx2x1   gen() 16884 MB/s
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [    8.656115] raid6: avx2x2   gen() 20311 MB/s
b156c-96ca-42a7-b8f3-a081dcac7ea2 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 13 10:34:27 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 instance-setup: INFO /proc/irq/30/smp_affinity_likew daemon.
Aug 13 10:34:28 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 google-accounts: INFO Created user account me.
Aug 13 10:34:28 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 google-accounts: INFO Created user account me.
Aug 13 10:34:28 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 google-accounts: INFO Creating a new user account for henrik.
Aug 13 10:34:28 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 google-accounts: INFO Created user account henrik.
Aug 13 10:34:28 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 google-accounts: INFO Creating a new user account for emma.
Aug 13 10:34:28 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 google-accounts: INFO Created user account emma.
Aug 13 10:34:28 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 google-accounts: INFO Creating a new user account for igor.
Aug 13 10:34:28 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 google-accounts: INFO Created user account igor.
Aug 13 10:34:28 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 13 10:34:28 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 google-accounts: INFO Created user account konstantinhaase.
Aug 13 10:34:28 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 google-accounts: INFO Creating a new user account for aj.
Aug 13 10:34:28 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 google-accounts: INFO Created user account aj.
Aug 13 10:34:28 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 google-accounts: INFO Creating a new user account for solarce.
Aug 13 10:34:28 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 10:34:31 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 cron[1747]: (CRON) STARTUP (fork ok)
Aug 13 10:34:31 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 cron[1747]: (CRON) INFO (Running @reboot jobs)
Aug 13 10:34:31 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 acpid: starting up with netlink and the input layer
Aug 13 10:34:31 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 acpid: 1 rule loaded
Aug 13 10:34:31 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 acpid: waiting for events: event logging is off
Aug 13 10:34:31 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 haveged: haveged starting up
Aug 13 10:34:31 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [   16.178353] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 10:34:36 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 ntpd[1844]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 13 10:34:36 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 ntpd[1845]: proto: precision = 0.100 usec
Aug 13 10:34:36 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 ntpd[1845]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 13 10:34:36 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 ntpd[1845]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 10:34:36 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 ntpd[1845]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 10:34:36 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 ntpd[1845]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 13 10:34: docker0: port 1(veth4a41280) entered forwarding state
Aug 13 11:17:01 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 CRON[17954]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 13 11:27:00 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [ 3164.588181] veth661cb97: renamed from eth0
Aug 13 11:27:00 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [ 3164.621624] docker0: port 1(veth4a41280) entered disabled state
Aug 13 11:27:00 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [ 3164.652822] docker0: port 1(veth4a41280) entered disabled state
Aug 13 11:27:00 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [ 3164.654553] device veth4a41280 left promiscuous mode
Aug 13 11:27:00 travis-job-b9ab156c-96ca-42a7-b8f3-a081dcac7ea2 kernel: [ 3164.654556] docker0: port 1(veth4a41280) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:034e214e
