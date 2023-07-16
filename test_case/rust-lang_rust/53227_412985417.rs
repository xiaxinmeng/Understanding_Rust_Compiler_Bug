plain
[00:48:55]     Checking unwind v0.0.0 (file:///checkout/src/libunwind)
[00:48:55]     Checking alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:48:55]     Checking alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:48:55]     Checking panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:48:58] warning: `[swap]` cannot be resolved, ignoring it...
[00:48:58]   --> liballoc/pin.rs:27:73
[00:48:58]    |
[00:48:58] 27 | //! A type may be moved out of a reference to it using a function like [swap],
[00:48:58]    |
[00:48:58]    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[00:48:58]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:48:58] 
---
[00:51:09] ....................................................................................................
[00:51:12] ....................................................................................................
[00:51:15] ....................................................................................................
[00:51:18] .......i............................................................................................
[00:51:23] .................................................................................F...........i......
[00:51:26] .i..................................................................................................
[00:51:32] ....................................................................................................
[00:51:34] ....................................................................................................
[00:51:36] ....................................................................................................
[00:51:39] ....................................................................................................
---
[00:53:06] ---- [ui] ui/consts/const-size_of-cycle.rs stdout ----
[00:53:06] diff of stderr:
[00:53:06] 
[00:53:06] 2    |
[00:53:06] 3 note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...
[00:53:06] 4 note: ...which requires const-evaluating `Foo::bytes::{{constant}}`...
[00:53:06] -   --> $SRC_DIR/libcore/mem.rs:323:14
[00:53:06] +   --> $SRC_DIR/libcore/mem.rs:321:14
[00:53:06] 6    |
[00:53:06] 7 LL |     unsafe { intrinsics::size_of::<T>() }
[00:53:06] 
[00:53:06] 
[00:53:06] 9    = note: ...which again requires computing layout of `Foo`, completing the cycle
[00:53:06] 10 note: cycle used when const-evaluating `Foo::bytes::{{constant}}`
[00:53:06] -   --> $SRC_DIR/libcore/mem.rs:323:14
[00:53:06] +   --> $SRC_DIR/libcore/mem.rs:321:14
[00:53:06] 12    |
[00:53:06] 13 LL |     unsafe { intrinsics::size_of::<T>() }
[00:53:06] 
[00:53:06] 
[00:53:06] The actual stderr differed from the expected stderr.
[00:53:06] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
[00:53:06] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
[00:53:06] To upda"// Copyright 2017 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/consts/const-size_of-cycle.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2017 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which requires const-evaluating `Foo::bytes::{{constant}}`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/libcore/mem.rs","byte_start":10182,"byte_end":10208,"line_start":321,"line_end":321,"column_start":14,"column_end":40,"is_primary":true,"text":[{"text":"    unsafe { intrinsics::size_of::<T>() }","highlight_start":14,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires computing layout of `Foo`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when const-evaluating `Foo::bytes::{{constant}}`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/libcore/mem.rs","byte_start":10182,"byte_end":10208,"liith `RUST_BACKTRACE=1` for a backtrace.
[00:53:06] 
[00:53:06] failures:
[00:53:06]     [ui] ui/consts/const-size_of-cycle.rs
[00:53:06] 
[00:53:06] 
[00:53:06] test result: FAILED. 4106 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out
[00:53:06] 
[00:53:06] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:53:06] 
[00:53:06] 
[00:53:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:53:06] expected success, got: exit code: 101
4 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680  (18400.00 BogoMIPS)
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.854014] devtmpfs: initialized
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.859729] evm: security.selinux
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.861286] evm: security.SMACK64
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.863047] evm: security.SMACK64EXEC
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.864889] evm: security.SMACK64TRANSMUTE
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.866619] evm: security.SMACK64MMAP
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.868465] evm: security.ima
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.869895] evm: security.capability
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.871888] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.876746] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.880071] pinctrl core: initialized pinctrl subsystem
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.882779] RTC time: 18:23:04, date: 08/14/18
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.886134] NET: Registered protocol family 16
Aug 14 18:23:15 travis-job-ed7e96422e72 kernel: [    0.979556] ACPI: Using IOAPIC for interrupt routing
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    0.981800] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.015372] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.018353] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.021555] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.024897] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.030374] PCI host bridge to bus 0000:00
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.032385] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.035462] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.038256] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.041982] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 14 18:23:15 trav window]
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.269631] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.269663] NET: Registered protocol family 2
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.272149] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.275902] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.279017] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.281985] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.284980] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.288667] NET: Registered protocol family 1
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.291506] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.293984] PCI: CLS 0 bytes, default 64
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    1.294038] Unpacking initramfs...
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.327189] Freeing initrd memory: 21432K
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.329450] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.332742] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.337957] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.342225] hw unit of domain pp0-core 2^-0 Joules
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.344565] hw unit of domain package 2^-0 Joules
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.346481] hw unit of domain dram 2^-16 Joules
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.348538] Scanning for low memory corruption every 60 seconds
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.352171] audit: initializing netlink subsys (disabled)
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.354805] audit: type=2000 audit(1534270986.516:1): initialized
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.358162] Initialise system trusted keyring
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.360958] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.364357] HugeTLB registeredleaving for legacy driver
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.451692] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.476571] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.501857] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.527998] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.554203] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.559806] Linux agpgart interface v0.103
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.565027] loop: module loaded
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.566944] libphy: Fixed MDIO Bus: probed
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.569068] tun: Universal TUN/TAP device driver, 1.6
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.571393] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.617237] PPP generic driver version 2.4.2
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    3.619677] ehci_hcd: USB 2.0 'Enhanced' He as /devices/platform/i8042/serio1/input/input4
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    6.817346] floppy0: no floppy controllers found
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    7.977203] raid6: sse2x1   gen()  8997 MB/s
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    8.045202] raid6: sse2x1   xor()  6684 MB/s
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    8.113195] raid6: sse2x2   gen() 11113 MB/s
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    8.181195] raid6: sse2x2   xor()  7273 MB/s
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    8.249200] raid6: sse2x4   gen() 12876 MB/s
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    8.317195] raid6: sse2x4   xor()  9026 MB/s
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    8.385196] raid6: avx2x1   gen() 17375 MB/s
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    8.453200] raid6: avx2x2   gen() 20740 MB/s
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    8.521195] raid6: avx2x4   gen() 23009 MB/s
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    8.522056] raid6: using algorithm avx2x4 gen() 23009 MB/s
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    8.522824] raid6: using avx2x2 recovery algorithm
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [    8.524947] xor: automatically using best checksummiom: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   10.136756] random: cloud-init: uninitialized urandom read (32 bytes read, 44 bits of entropy available)
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   10.286119] systemd-udevd[702]: starting version 204
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   10.413156] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   10.486933] intel_rapl: no valid rapl domains found in package 0
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   10.571352] ppdev: user-space parallel port driver
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   10.648527] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   10.703772] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   10.776270] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   10.939285] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 14 18:23:15 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   11.19154O Creating a new user account for bogdana.
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 google-accounts: INFO Created user account bogdana.
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 google-accounts: INFO Creating a new user account for konstantin.
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 google-accounts: INFO Created user account konstantin.
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   13.331786] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   13.335896] Bridge firewalling registered
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 google-accounts: INFO Creating a new user account for carmen.
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   13.350568] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 google-clock-skew: INFO Synced system time with hardware clock.
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   13.387070] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 google-accounts: INFO Created user account carmen.
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 google-accounts: INFO Creating a new user account for maria.
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   13.458778] Initializing XFRM netlink socket
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   13.465363] Netfilter messages via NETLINK v0.30.
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   13.468835] ctnetlink v0.93: registering with nfnetlink.
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 google-accounts: INFO Created user account maria.
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 google-accounts: INFO Removing user packer.
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   13.513284] floppy0: no floppy controllers found
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 18:23:17 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 18:23:19 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 18:23:19 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 18:23:19 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 cron[1710]: (CRON) INFO (pidfile fd = 3)
Aug 14 18:23:19 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 cron[1747]: (CRON) STARTUP (fork ok)
Aug 14 18:23:19 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 cron[1747]: (CRON) INFO (Running @reboot jobs)
Aug 14 18:23:19 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 acpid: starting up with netlink and the input layer
Aug 14 18:23:19 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 acpid: 1 rule loaded
Aug 14 18:23:19 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 acpid: waiting for events: event logging is off
Aug 14 18:23:19 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 haveged: haveged starting up
Aug 14 18:23:19 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [   15.815696] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 18:23:24 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ntpd[1854]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 14 18:23:24 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ntpd[1855]: proto: precision = 0.110 usec
Aug 14 18:23:24 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ntpd[1855]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 18:23:24 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ntpd[1855]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 18:23:24 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ntpd[1855]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 18:23:24 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ntpd[1855]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 14 18:23:24 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ntpd[1855]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 14 18:23:24 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ntpd[1855]: Listen normally on 3 eth0 10.20.2.2 UDP 123
Aug 14 18:23:24 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ntpd[1855]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 14 18:23:24 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ntpd[1855]: peers refreshed
Aug 14 18:23:24 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 nDSA)
Aug 14 18:23:24 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ec2: 256 bf:c7:0e:be:f2:b4:7c:7e:25:cf:89:3b:0a:21:31:07  root@travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 (ED25519)
Aug 14 18:23:24 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ec2: 2048 2b:e1:5b:21:19:bf:5b:72:fe:17:4f:5c:2d:52:03:e3  root@travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 (RSA)
Aug 14 18:23:24 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 14 18:23:24 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ec2: #############################################################
Aug 14 18:23:32 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ntpdate[2987]: the NTP socket is in use, exiting
Aug 14 18:25:00 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [  116.484257] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 14 18:28:25 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [  321.475403] device vethb658c98 entered promiscuous mode
Aug 14 18:28:25 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [  321.475480] docker0: port 1(vethb658c98) entered forwarding state
Aug 14 18:28:25 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [  321.475492] docker0: port 1(vethb658c98) entered forwarding state
Aug 14 18:28:25 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [  321.476931] docker0: port 1(vethb658c98) entered disabled state
Aug 14 18:28:25 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [  321.582118] cgroup: docker-runc (4864) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 18:28:25 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [  321.582121] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 18:28:25 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [  321.657857] eth0: renamed from vethf494a9c
Aug 14 18:28:25 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [  321.697112] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 18:28:25 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [  321.698400] docker0: port 1(vethb658c98) entered forwarding state
Aug 14 18:28:25 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [  321.698427] docker0: port 1(vethb658c98) entered forwarding state
Aug 14 18:28:25 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [  321.698452] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 14 18:28:28 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ntpd[1855]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 14 18:28:28 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ntpd[1855]: Listen normally on 6 docker0 fe80::42:a7ff:fe79:1b82 UDP 123
Aug 14 18:28:28 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ntpd[1855]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 14 18:28:28 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ntpd[1855]: peers refreshed
Aug 14 18:28:28 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 ntpd[1855]: new interface(s) found: waking up resolver
Aug 14 18:28:40 travis-job-ed7fa9f4-2d27-4bb6-bf2b-43be96422e72 kernel: [  336.729183] docker0: port 1(vethb658c98) entered forwarding state
