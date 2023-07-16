plain
[00:57:24] ....................................................................................................
[00:57:27] ....................................................................................................
[00:57:29] ....................................................................................................
[00:57:32] ....................................................................................................
[00:57:35] ................iiiiiiiii...........................................................................
[00:57:41] ....................................................................................................
[00:57:45] .....................i..............................................................................
[00:57:48] ................................i...................................................................
[00:57:51] ....................................................................................................
---
[01:04:05] ....................................................................................................
[01:04:08] ....................................................................................................
[01:04:12] ...........................................................................................i........
[01:04:15] ...................................................ii.iii...........................................
[01:04:18] .............................................................F.....................i................
[01:04:26] ....................................................................................................
[01:04:29] ............................................................i.......................................
[01:04:32] .......................................................i..ii........................................
[01:04:35] ....................................................................................................
---
[01:04:55] failures:
[01:04:55] 
[01:04:55] ---- [compile-fail] compile-fail/extern-wrong-value-type.rs stdout ----
[01:04:55] 
[01:04:55] error: /checkout/src/test/compile-fail/extern-wrong-value-type.rs:19: unexpected error: '19:5: 19:10: expected a `std::ops::Fn<()>` closure, found `extern "C" fn() {f}` [E0277]'
[01:04:55] 
[01:04:55] error: /checkout/src/test/compile-fail/extern-wrong-value-type.rs:19: expected error not found: expected a `std::ops::Fn<()>` closure, found `extern \"C\" fn() {f}`
[01:04:55] error: 1 unexpected errors found, 1 expected errors not found
[01:04:55] status: exit code: 1
[01:04:55] status: exit code: 1
[01:04:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/extern-wrong-value-type.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/extern-wrong-value-type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/extern-wrong-value-type/auxiliary" "-A" "unused"
[01:04:55]     Error {
[01:04:55]         line_num: 19,
[01:04:55]         kind: Some(
[01:04:55]             Error
[01:04:55]             Error
[01:04:55]         ),
[01:04:55]         msg: "19:5: 19:10: expected a `std::ops::Fn<()>` closure, found `extern \"C\" fn() {f}` [E0277]"
[01:04:55] ]
[01:04:55] 
[01:04:55] not found errors (from test file): [
[01:04:55]     Error {
[01:04:55]     Error {
[01:04:55]         line_num: 19,
[01:04:55]         kind: Some(
[01:04:55]             Error
[01:04:55]         ),
[01:04:55]         msg: "expected a `std::ops::Fn<()>` closure, found `extern \\\"C\\\" fn() {f}`"
[01:04:55] ]
[01:04:55] 
[01:04:55] thread '[compile-fail] compile-fail/extern-wrong-value-type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1285:13
[01:04:55] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:55] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:55] 
[01:04:55] 
[01:04:55] failures:
[01:04:55]     [compile-fail] compile-fail/extern-wrong-value-type.rs
[01:04:55] 
[01:04:55] test resernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] kvm-clock: using sched offset of 1880159775 cycles
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] Zone ranges:
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000]   Device   empty
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] Movable zone start for each node
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] Early memory node ranges
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0fb073a4 kernel: [    0.852078] x86: Booted up 1 node, 4 CPUs
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.854677] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.859760] devtmpfs: initialized
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.865401] evm: security.selinux
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.866644] evm: security.SMACK64
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.868189] evm: security.SMACK64EXEC
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.869519] evm: security.SMACK64TRANSMUTE
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.871121] evm: security.SMACK64MMAP
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.872534] evm: security.ima
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.873465] evm: security.capability
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.875426] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.879169] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.881760] pinctrl core: initialized pinctrl subsystem
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.884446] erpreter enabled
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.972699] ACPI: (supports S0 S3 S4 S5)
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.974560] ACPI: Using IOAPIC for interrupt routing
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    0.976985] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.010004] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.013137] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.016027] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.018402] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.023593] PCI host bridge to bus 0000:00
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.025102] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.027361] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.030099] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.032908] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.035886] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.037949] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.038463] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.070786] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.104466] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.108444] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.120393] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.129701] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.157690] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    1.169206] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [4 kernel: [    3.629859] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.631120] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.633599] zbud: loaded
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.634795] VFS: Disk quotas dquot_6.6.0
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.636159] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.638602] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.640420] fuse init (API version 7.23)
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.642876] Key type big_key registered
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.643929] Allocating IMA MOK and blacklist keyrings.
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.647395] Key type asymmetric registered
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.648348] Asymmetric key parser 'x509' registered
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.649418] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.651066] io scheduler noop registered
Aug 12 21:39:16 travis-job-53b29   3.670258] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.671231] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.676759] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.699949] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.724436] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.748643] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.772271] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.775969] Linux agpgart interface v0.103
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.780310] loop: module loaded
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.781756] libphy: Fixed MDIO Bus: probed
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.783197] tun: Universal TUN/TAP device driver, 1.6
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.784855] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.829611] PPP generic driver version 2.4.2
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.831110] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.833187] ehci-pci: EHCI PCI platform driver
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.834342] ehci-platform: EHCI generic platform driver
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.835880] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.838217] ohci-pci: OHCI PCI platform driver
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.839675] ohci-platform: OHCI generic platform driver
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.841029] uhci_hcd: USB Universal Host Controller Interface driver
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.843109] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.848238] i8042: Warning: Keylock active
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.850963] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.852350] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.854239] mousedev: PS/2 mouse device common for all mice
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.856558] rtc_cmos 00:00: RTC can wake from S4
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.858069] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.860164] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.862455] i2c /dev entries driver
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.863906] device-mapper: uevent: version 1.0.3
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.865494] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.868248] ledtrig-cpu: registered to indicate activity on CPUs
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.870607] NET: Registered protocol family 10
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.872322] NET: Registered protocol family 17
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.873445] Key type dns_resolver registered
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.874870] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [    3.876631] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 12 21:39:-f82b-4423-bc41-0fb38fb073a4 kernel: [   11.622640] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [   11.678208] EXT4-fs (sda1): resized filesystem to 7864064
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [   12.166165] init: failsafe main process (1096) killed by TERM signal
Aug 12 21:39:16 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 instance-setup: INFO Running set_multiqueue.
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 instance-setup: INFO Set channels for eth0 to 4.
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affiINFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 google-accounts: INFO Starting Google Accounts daemon.
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 google-accounts: INFO Creating a new user account for me.
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 google-accounts: INFO Created user account me.
Aug 12 21:39:17 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 google-accounts: INFO Creating a new user account for bogdana.
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 google-clock-skew: INFO Synced system time with hardware clock.
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 google-accounts: INFO Created user account bogdana.
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 google-accounts: INFO Creating a new user account for aj.
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 google-accounts: INFO Created user account aj.
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 google-accounts: INFO Creating a new user account for asari.
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 google-accounts: INFO Created user account asari.
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 google-accounts: INFO Removing user packer.
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [   13.590473] random: nonblocking pool is initialized
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [   13.604729] floppy0: no floppy controllers found
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [   13.660008] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [   13.665180] Bridge firewalling registered
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [   13.682773] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [   13.731225] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [   13.817437] Initializing XFRM netlink socket
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [   13.826327] Netfilter messages via NETLINK v0.30.
Aug 12 21:39:18 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [   13.829779] ctnetlink v0.93: registering with nfnetlink.
Aug 12 21:39:20 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 cron[1634]: (CRON) INFO (pidfile fd = 3)
Aug 12 21:39:20 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 12 21:39:20 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 12 21:39:20 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 cron[1668]: (CRON) STARTUP (fork ok)
Aug 12 21:39:20 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 cron[1668]: (CRON) INFO (Running @reboot jobs)
Aug 12 21:39:20 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 acpid: starting up with netlink and the input layer
Aug 12 21:39:20 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 acpid: 1 rule loaded
Aug 12 21:39:20 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 acpid: waiting for events: event logging is off
Aug 12 21:39:20 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 haveged: haveged starting up
Aug 12 21:39:20 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 kernel: [   15.816375] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 12 21:39:25 travis-job-53b294ed-f82b-4423-bc41-0fb38fb073a4 ntpd[1771]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:2
