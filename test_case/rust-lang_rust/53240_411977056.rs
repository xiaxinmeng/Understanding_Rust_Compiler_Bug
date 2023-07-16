plain
[00:06:38]    Compiling cc v1.0.18
[00:06:38]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:06:38]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:06:38]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:06:38] error: incorrect close delimiter: `}`
[00:06:38]     |
[00:06:38] 404 |         }
[00:06:38]     |         ^
[00:06:38]     |
[00:06:38]     |
[00:06:38] note: unclosed delimiter
[00:06:38]    --> libcore/num/mod.rs:385:20
[00:06:38]     |
[00:06:38] 385 |             concat!("Reverses the byte order of the integer.
[00:06:38] 
[00:06:39] error: unexpected end of macro invocation
[00:06:39]    --> libcore/num/mod.rs:385:20
[00:06:39]     |
[00:06:39]     |
[00:06:39] 385 |               concat!("Reverses the byte order of the integer.
[00:06:39] 386 | |
[00:06:39] 387 | | # Examples
[00:06:39] 388 | |
[00:06:39] ...   |
[00:06:39] ...   |
[00:06:39] 403 | |             }
[00:06:39] 404 | |         }
[00:06:39]     | |_________^
[00:06:39] 
[00:06:39] error: expected expression, found `u8`
[00:06:39]     --> libcore/num/mod.rs:2277:22
[00:06:39]      |
[00:06:39] 2277 | let n = ", $swap_op, $SelfT, ";
[00:06:39] ...
[00:06:39] ...
[00:06:39] 3619 |     uint_impl! { u8, u8, 8, 255, "", "", 2, "0x82", "0xa", "0x12", "0x12", "0x48" }
[00:06:39] 
[00:06:39] error: aborting due to 3 previous errors
[00:06:39] 
[00:06:39] error: Could not compile `core`.
[00:06:39] error: Could not compile `core`.
[00:06:39] 
[00:06:39] Caused by:
[00:06:39]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=bd44783aadae9ca1 -C extra-filename=-bd44783aadae9ca1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 1)
[00:06:39] warning: build failed, waiting for other jobs to finish...
[00:06:44] error: build failed
[00:06:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:06:44] expected success, got: exit code: 101
[00:06:44] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:06:44] travis_fold:end:stage0-std

[00:06:44] travis_time:end:stage0-std:start=1533877762193100781,finish=1533877768327440363,duration=6134339582


[00:06:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:44] Build completed unsuccessfully in 0:00:07
[00:06:44] Makefile:28: recipe for target 'all' failed
[00:06:44] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e9920c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:2e3f954f
$ sudo tail -n 500 /var/log/syslog
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000]   4 disabled
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000]   5 disabled
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000]   6 disabled
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000]   7 disabled
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] Using GB pages for direct mapping
Aug 10 04:55:4956 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 k04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.399934] ftrace: allocating 32185 entries in 126 pages
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.446043] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.447583] smpboot: Max logical packages: 2
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.449170] x2apic enabled
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.450877] Switched APIC routing to physical x2apic.
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.454679] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.561558] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.563630] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.566021] x86: Booting SMP configuration:
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.566793] .... node  #0, CPUs:      #1
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.567749] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.571186]  #2
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.571991] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.576008]  #3
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.576701] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.580195] x86: Booted up 1 node, 4 CPUs
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.581139] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.583992] devtmpfs: initialized
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.588409] evm: security.selinux
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.589577] evm: security.SMACK64
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.590307] evm: security.SMACK64EXEC
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.591228] evm: security.SMACK64TRANSMUTE
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.591882] evm: security.SMACK64MMAP
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.592614] evm: security.ima
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.593121] evm: security.capability
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.594560] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 76450417851000fe000-0xfebfe07f]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.778825] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.784766] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.789341] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.803885] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.806646] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.809770] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.812305] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.815137] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.835806] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.836924] vgaarb: loaded
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.837757] SCSI subsystem initialized
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.838581] libata version 3.00 loaded.
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.838608] ACPI: bus type USB registered
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.839321] usbcore: registered new interface driver usbfs
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.840196] usbcore: registered new interface driver hub
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.841368] usbcore: registered new device driver usb
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.842585] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.844173] dmi: Firmware registration failed.
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.845266] PCI: Using ACPI for IRQ routing
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.846017] PCI: pci_cache_line_size set to 64 bytes
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.846121] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.846124] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.846274] NetLabel: Initializing
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.846827] NetLabel:  domain hash size = 128
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.847676] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-5f-b97e-fd43dd264956 kernel: [    0.868961] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.870402] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.870405] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.870406] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.870408] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.870452] NET: Registered protocol family 2
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.871486] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.873471] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.874752] TCP: Hash tables configured (established 131072 bind 65536)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.875981] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    0.876907] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernet: initializing netlink subsys (disabled)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    2.924568] audit: type=2000 audit(1533876906.554:1): initialized
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    2.925986] Initialise system trusted keyring
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    2.926964] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    2.928015] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    2.930279] zbud: loaded
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    2.931142] VFS: Disk quotas dquot_6.6.0
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    2.932122] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    2.933706] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    2.935019] fuse init (API version 7.23)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    2.936120] Key type big_key registered
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    2.937080] Allocating IMA MOK and blacklist keyrings.
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    2.939602] Key type asymmetric registered
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 keylock active
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.128080] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.129885] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.131946] mousedev: PS/2 mouse device common for all mice
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.134245] rtc_cmos 00:00: RTC can wake from S4
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.136062] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.137940] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.140528] i2c /dev entries driver
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.141759] device-mapper: uevent: version 1.0.3
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.143514] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.146568] ledtrig-cpu: registered to indicate activity on CPUs
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.148916] NET: Registered protocol family 10
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.150164] NET: Registered protocol family 17
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.150991] Key type dns_resolver registered
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.152232] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.153973] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.155422] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.156705] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.158547] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.160877] registered taskstats version 1
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.161710] Loading compiled-in X.509 certificates
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.163946] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.166029] zswap: loaded using pool lzo/zbud
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.168762] Key type trusted registered
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.174105] Key type encrypted registered
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.175598] ima: No TPM chip found, activating TPM-bypass!
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.177373] evm: HMAC attrs: 0x1
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.178746]   Magic number: 14:76:919
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.179753] rtc_cmos 00:00: setting system clock to 2018-08-10 04:55:07 UTC (1533876907)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.181970] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.183092] EDD information not available.
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.184271] PM: Hibernation image not present or could not be loaded.
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.185769] Freeing unused kernel memory: 1496K
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.186514] Write protecting the kernel read-only data: 14336k
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.188372] Freeing unused kernel memory: 1956K
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.189504] Freeing unused kernel memory: 92K
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.204571] systemd-udevd[120]: starting version 204
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    3.255625] scsi host0: Virtio SCSI HBA
Aug 10 04:55:14 travis-job-87c travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    8.293568] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    8.482381] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    9.030626] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    9.160277] systemd-udevd[703]: starting version 204
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    9.280510] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    9.340394] intel_rapl: no valid rapl domains found in package 0
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    9.404580] intel_rapl: no valid rapl domains found in package 0
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    9.416792] ppdev: user-space parallel port driver
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    9.500233] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    9.547150] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    9.616547] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [    9.779930] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [   10.049765] random: mktemp: uninitialized urandom read (12 bytes read, 58 bits of entropy available)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [   10.124105] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [   10.198150] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [   10.238241] EXT4-fs (sda1): resized filesystem to 7864064
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [   10.546566] init: failsafe main process (1097) killed by TERM signal
Aug 10 04:55:14 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 10 04:55:15 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 instance-setup: INFO Running set_multiqueue.
Aug 10 04:55:15 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 instance-setup: INFO Set channels for eth0 to 4.
Aug 10 04:55:15 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 10 04:55:15 travis-job-87c1a404-0207-4a5:16 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 acpid: starting up with netlink and the input layer
Aug 10 04:55:16 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 acpid: 1 rule loaded
Aug 10 04:55:16 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 acpid: waiting for events: event logging is off
Aug 10 04:55:16 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 04:55:16 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 haveged: haveged starting up
Aug 10 04:55:16 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 04:55:16 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [   11.906263] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 04:55:16 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [   11.923459] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 04:55:16 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [   11.971844] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 10 04:55:16 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [   11.975115] Bridge firewalling registered
Aug 10 04:55:16 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [   11.987192] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 10 04:55:16 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [   12.060129] Initializing XFRM netlink socket
Aug 10 04:55:16 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [   12.069027] Netfilter messages via NETLINK v0.30.
Aug 10 04:55:16 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [   12.072695] ctnetlink v0.93: registering with nfnetlink.
Aug 10 04:55:16 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 google-clock-skew: INFO Synced system time with hardware clock.
Aug 10 04:55:16 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [   12.351526] floppy0: no floppy controllers found
Aug 10 04:55:38 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 ntpdate[1771]: adjust time server 169.254.169.254 offset 0.016053 sec
Aug 10 04:55:45 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 ntpd[1807]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 10 04:55:45 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 ntpd[1808]: proto: precision = 0.134 usec
Aug 10 04:55:45 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 ntpd[1808]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 04:55:45 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 ntpd[1808]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 04:55:45 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 ntpd[1808]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 10 04:55:45 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 ntpd[1808]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 10 04:55:45 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 ntpd[1808]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 10 04:55:45 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 ntpd[1808]: Listen normally on 3 eth0 10.20.0.17 UDP 123
Aug 10 04:55:45 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 ntpd[1808]: Listen normally on 4 docker0 172.) entered forwarding state
Aug 10 05:09:28 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [  864.474207] docker0: port 1(veth5d4c5f9) entered disabled state
Aug 10 05:09:28 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [  864.476166] veth902a523: renamed from eth0
Aug 10 05:09:28 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [  864.546609] docker0: port 1(veth5d4c5f9) entered disabled state
Aug 10 05:09:28 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [  864.548320] device veth5d4c5f9 left promiscuous mode
Aug 10 05:09:28 travis-job-87c1a404-0207-4a5f-b97e-fd43dd264956 kernel: [  864.548323] docker0: port 1(veth5d4c5f9) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:1c587924
