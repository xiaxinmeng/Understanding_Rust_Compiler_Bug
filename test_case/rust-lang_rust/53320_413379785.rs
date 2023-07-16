plain
[00:51:45] ....................................................................................................
[00:51:48] ...............................................................................................i....
[00:51:51] ....................................................................................................
[00:51:54] ....................................................................................................
[00:51:57] ............................................iiiiiiiii...............................................
[00:52:02] ....................................................................................................
[00:52:06] ....................................................................................................
[00:52:09] .......................i............................................................................
[00:52:12] ..........................i...............................................i.i..ii...................
---
[01:20:44] travis_fold:end:stage0-linkchecker

[01:20:44] travis_time:end:stage0-linkchecker:start=1534378402874086686,finish=1534378405277425241,duration=2403338555

] reference/print.html:931: absolute path - /expressions/operator-expr.html
[01:20:58] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:20:58] 
[01:20:58] 
[01:20:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:20:58] expected success, got: exit code: 101
[01:20:58] expected success, got: exit code: 101
[01:20:58] 
[01:20:58] 
[01:20:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:58] Build completed unsuccessfully in 0:33:24
[01:20:58] make: *** [check] Error 1
[01:20:58] Makefile:58: recipe for target 'check' failed
job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000] ACPI: INT_SRC_OVR : 3870588
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000] Policy zone: Normal
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3l Processor ID: 0
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.362636] CPU: Processor Core ID: 0
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.364072] mce: CPU supports 32 MCE banks
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.364877] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.366427] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.370295] Freeing SMP alternatives memory: 32K
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.380499] ftrace: allocating 32185 entries in 126 pages
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.435503] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.436807] smpboot: Max logical packages: 2
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.437985] x2apic enabled
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.439525] Switched APIC routing to physical x2apic.
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.443283] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.549503] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 15 22:50:43 travis-job-63a96b-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.686844] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.687918] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.689462] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.691969] PCI host bridge to bus 0000:00
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.692592] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.693674] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.694611] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.695903] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.697106] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.697984] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.698390] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.712871] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.727511] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.729184] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.734498] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.739092] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.752971] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.758579] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.763142] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.776090] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.778353] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.780530] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.782633] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10   0.828369] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.828410] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.828454] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.828494] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.828661] pnp: PnP ACPI: found 7 devices
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.836291] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.837868] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.837871] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.837872] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.837874] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.837911] NET: Registered protocol family 2
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    0.838890] TCP established hash table entries: 131072 (order:I unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.880386] hw unit of domain pp0-core 2^-0 Joules
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.881456] hw unit of domain package 2^-0 Joules
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.882517] hw unit of domain dram 2^-16 Joules
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.883278] Scanning for low memory corruption every 60 seconds
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.885069] audit: initializing netlink subsys (disabled)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.886091] audit: type=2000 audit(1534373435.448:1): initialized
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.887237] Initialise system trusted keyring
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.888205] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.889280] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.891280] zbud: loaded
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.892370] VFS: Disk quotas dquot_6.6.0
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.893059] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 15 22:5un on family 6 model 63
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.907092] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.908161] ACPI: Power Button [PWRF]
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.908835] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.909997] ACPI: Sleep Button [SLPF]
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.911122] GHES: HEST is not enabled!
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.913362] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.914401] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.919212] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.920280] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.925363] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.948282] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    2.972129] 00:04Driver
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.073604] ohci-pci: OHCI PCI platform driver
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.074717] ohci-platform: OHCI generic platform driver
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.076157] uhci_hcd: USB Universal Host Controller Interface driver
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.077902] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.081182] i8042: Warning: Keylock active
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.083500] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.084970] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.086707] mousedev: PS/2 mouse device common for all mice
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.088793] rtc_cmos 00:00: RTC can wake from S4
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.090485] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.092650] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.094641] i2c /dev entries driver
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.095826] device-mapper: uevent: version 1.0.3
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.097494] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.099567] ledtrig-cpu: registered to indicate activity on CPUs
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.101731] NET: Registered protocol family 10
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.103204] NET: Registered protocol family 17
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.104444] Key type dns_resolver registered
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.106147] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.108396] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.110208] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.111830] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.113179] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.116190] registered taskstats version 1
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-791-4bcc-8779-792c3b0af9d8 kernel: [    3.251635] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.251665] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.252877]  sda: sda1
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.253600] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.288197] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.880018] tsc: Refined TSC clocksource calibration: 2300.001 MHz
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    3.882187] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735f0517, max_idle_ns: 440795237604 ns
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    4.120994] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    6.212052] floppy0: no floppy controllers found
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    7.363881] raid6: sse2x1   gen()  8967 MB/s
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    7.431882] raid6: sse2x1   xor()  6590 MB/s
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    7.499881] raid6: sse2x2   gen() 11103 MB/s
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    7.567880] raid6: sse2x2   xor()  7222 MB/s
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    7.635878] raid6: sse2x4   gen() 12896 MB/s
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    7.703877] raid6: sse2x4   xor()  9039 MB/s
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    7.771877] raid6: avx2x1   gen() 17343 MB/s
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    7.839880] raid6: avx2x2   gen() 20361 MB/s
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    7.907879] raid6: avx2x4   gen() 23309 MB/s
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    7.908733] raid6: using algorithm avx2x4 gen() 23309 MB/s
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    7.909646] raid6: using avx2x2 recovery algorithm
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    7.911963] xor: automatically using best checksumming function:
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    7.951877]    avx       : 27547.000 MB/sec
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    7.965904] Btrfs loaded
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    8.007917] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    8.009092] EXT4-fs (sda1): write access will be enabled during recovery
Autialized - upgrade BIOS or use force_addr=0xaddr
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    9.475239] intel_rapl: no valid rapl domains found in package 0
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    9.521835] ppdev: user-space parallel port driver
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    9.627445] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    9.677599] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    9.743930] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [    9.907628] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [   10.181618] random: mktemp: uninitialized urandom read (12 bytes read, 61 bits of entropy available)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [   10.251633] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [   10.322883] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [   10.362922] EXT4-fs (sda1): resized fijob-63a96e32-3b11-4bcc-8779-792c3b0af9d8 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 google-accounts: INFO Starting Google Accounts daemon.
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 google-accounts: INFO Creating a new user account for me.
Aug 15 22:50:43 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 15 22:50:44 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 google-accounts: INFO Created user account me.
Aug 15 22:50:44 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 google-accounts: INFO Creating a new user account for henrik.
Aug 15 22:50:44 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 google-accounts: INFO Created user account henrik.
Aug 15 22:50:44 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 google-accounts: INFO Creating a new user account for emma.
Aug 15 22:50:44 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 google-accounts: INFO Created user account emma.
Aug 15 22:50:44 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 google-accounts: INFO Creating a new user account for igor.
Aug 15 22:50:44 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 google-accounts: INFO Created user account igor.
Aug 15 22:50:44 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 google-account9-792c3b0af9d8 google-accounts: INFO Removing user packer.
Aug 15 22:50:45 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 22:50:45 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 22:50:47 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 22:50:47 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 22:50:47 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 cron[1716]: (CRON) INFO (pidfile fd = 3)
Aug 15 22:50:47 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 cron[1752]: (CRON) STARTUP (fork ok)
Aug 15 22:50:47 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 cron[1752]: (CRON) INFO (Running @reboot jobs)
Aug 15 22:50:47 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 acpid: starting up with netlink and the input layer
Aug 15 22:50:47 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 acpid: 1 rule loaded
Aug 15 22:50:47 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 acpid: waiting for events: event logging is off
Aug 15 22:50:47 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 haveged: haveged starting up
Aug 15 22:50:47 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [   15.014599] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 22:50:50 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 ntpdate[975]: adjust time server 169.254.169.254 offset 0.013352 sec
Aug 15 22:50:52 travis-job-63a96e32-3y on 5 docker0 fe80::1 UDP 123
Aug 15 22:54:29 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 ntpd[1857]: Listen normally on 6 docker0 fe80::42:d5ff:fed0:b4fd UDP 123
Aug 15 22:54:29 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 ntpd[1857]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 15 22:54:29 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 ntpd[1857]: peers refreshed
Aug 15 22:54:29 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 ntpd[1857]: new interface(s) found: waking up resolver
Aug 15 22:54:41 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [  248.666717] docker0: port 1(veth7db4853) entered forwarding state
Aug 15 23:17:01 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 CRON[16374]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 15 23:45:34 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [ 3301.537081] traps: a[15640] trap invalid opcode ip:55f0abc08a9b sp:7fffa74aeac0 error:0 in a[55f0abc05000+6000]
Aug 15 23:45:49 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [ 3316.774305] traps: a[18462] trap invalid opcode ip:7f8eb6701381 sp:7fff079242e0 error:0 in libstd-2339b911e3c09de8.so[7f8eb66a1000+16f000]
Aug 15 23:45:49 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [ 3316.809990] traps: a[18476] trap invalid opcode ip:7f9a6147d381 sp:7ffc71fd9bd0 error:0 in libstd-2339b911e3c09de8.so[7f9a6141d000+16f000]
Aug 15 23:47:14 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [ 3401.476449] traps: a[859] trap invalid opcode ip:55f54d7fbd98 sp:7ffd680cc060 error:0 in a[55f54d7f9000+4000]
Aug 15 23:50:04 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [ 3572.172991] a[29457]: segfault at 0 ip 000055ef71205463 sp 00007fffd4007f30 error 6 in a[55ef71202000+5000]
Aug 15 23:50:14 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [ 3581.630939] a[30196]: segfault at 1 ip 000055c4d40c2b8c sp 00007ffc24e69df0 error 6 in a[55c4d40c0000+4000]
Aug 15 23:50:18 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [ 3585.714727] traps: a[30572] trap invalid opcode ip:56336420842c sp:7ffc1b0ca210 error:0 in a[563364205000+7000]
Aug 16 00:13:39 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [ 4986.714396] docker0: port 1(veth7db4853) entered disabled state
Aug 16 00:13:39 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [ 4986.714451] veth9c6fa2a: renamed from eth0
Aug 16 00:13:39 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [ 4986.771396] docker0: port 1(veth7db4853) entered disabled state
Aug 16 00:13:39 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [ 4986.773186] device veth7db4853 left promiscuous mode
Aug 16 00:13:39 travis-job-63a96e32-3b11-4bcc-8779-792c3b0af9d8 kernel: [ 4986.773188] docker0: port 1(veth7db4853) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:082dc8a6
