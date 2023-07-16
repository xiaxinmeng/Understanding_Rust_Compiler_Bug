plain

[00:09:31] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:09:31] tidy error: /checkout/src/test/ui/dropck/dropck-union.rs: incorrect license
[00:09:32] some tidy checks failed
[00:09:32] 
[00:09:32] 
[00:09:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:09:32] 
[00:09:32] 
[00:09:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:09:32] Build completed unsuccessfully in 0:00:55
[00:09:32] Build completed unsuccessfully in 0:00:55
[00:09:32] make: *** [tidy] Error 1
[00:09:32] Makefile:79: recipe for target 'tidy' failed
8cde7d2091f kernel: [    0.000000]   Device   empty
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] Movable zone start for each node
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] Early memory node ranges
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 12 18:34:53 travis-job-e8891504 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 12 18:34:53 travis-job-e8891504-1d66-g Calgary via BIOS EBDA area
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] Hierarchical RCU implementation.
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] console [ttyS0] enabled
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    08891504-1d66-40db-974c-98cde7d2091f kernel: [    0.557562] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.561735]  #3
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.562153] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.566216] x86: Booted up 1 node, 4 CPUs
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.566894] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.569354] devtmpfs: initialized
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.573342] evm: security.selinux
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.573871] evm: security.SMACK64
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.574405] evm: security.SMACK64EXEC
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.574911] evm: security.SMACK64TRANSMUTE
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.575531] evm: security.SMACK64MMAP
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.576058] evm: security.ima
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.576480] evm: security.capability
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.577302] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffdb-974c-98cde7d2091f kernel: [    0.686692] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.687770] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.688846] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.690000] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.691419] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.692308] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.692745] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.707463] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.720364] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.721785] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.726531] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.730667] pci0db-974c-98cde7d2091f kernel: [    0.796116] ACPI: bus type USB registered
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.796942] usbcore: registered new interface driver usbfs
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.797781] usbcore: registered new interface driver hub
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.798563] usbcore: registered new device driver usb
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.799470] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.800510] dmi: Firmware registration failed.
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.801305] PCI: Using ACPI for IRQ routing
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.802234] PCI: pci_cache_line_size set to 64 bytes
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.802336] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.802338] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.802459] NetLabel: Initializing
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.802972] NetLabel:  domain hash size = 128
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.803574] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 12 18:38:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.821624] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.822951] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.822953] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.822954] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.822956] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.822995] NET: Registered protocol family 2
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.823846] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.825094] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.826139] TCP: Hash tables configured (established 131072 bind 65536)
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.827344] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    0.828226] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 12 18:34:53 travis-job-e889150891504-1d66-40db-974c-98cde7d2091f kernel: [    2.901608] Asymmetric key parser 'x509' registered
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    2.902373] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    2.903518] io scheduler noop registered
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    2.904117] io scheduler deadline registered (default)
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    2.904872] io scheduler cfq registered
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    2.905498] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    2.906319] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    2.907259] intel_idle: does not run on family 6 model 63
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    2.907360] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    2.908386] ACPI: Power Button [PWRF]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    2.909053] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    2.910093] ACPI: Sleep Button [SLPF]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [d66-40db-974c-98cde7d2091f kernel: [    3.021679] libphy: Fixed MDIO Bus: probed
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.022456] tun: Universal TUN/TAP device driver, 1.6
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.023307] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.055524] PPP generic driver version 2.4.2
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.056361] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.057450] ehci-pci: EHCI PCI platform driver
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.058236] ehci-platform: EHCI generic platform driver
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.058999] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.059869] ohci-pci: OHCI PCI platform driver
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.060719] ohci-platform: OHCI generic platform driver
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.061714] uhci_hcd: USB Universal Host Controller Interface driver
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.062794] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kern2 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.082171] Key type dns_resolver registered
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.083087] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.083907] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.084843] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.086719] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.087927] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.089441] registered taskstats version 1
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.090159] Loading compiled-in X.509 certificates
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.091655] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.093295] zswap: loaded using pool lzo/zbud
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.096097] Key type trusted registered
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    3.099718] Key type encrypted registered
Aug 12 18:34:53 travis-j4-1d66-40db-974c-98cde7d2091f kernel: [    7.890125] raid6: avx2x4   gen() 22379 MB/s
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    7.892590] raid6: using algorithm avx2x4 gen() 22379 MB/s
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    7.895232] raid6: using avx2x2 recovery algorithm
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    7.899570] xor: automatically using best checksumming function:
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    7.942150]    avx       : 26643.000 MB/sec
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    7.958717] Btrfs loaded
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    8.009698] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    8.012768] EXT4-fs (sda1): write access will be enabled during recovery
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    8.107921] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    8.116466] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    8.118790] EXT4-fs (sda1): recovery complete
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    8.125918] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    8.354847] random: ini2 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [    9.938389] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [   10.008026] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [   10.175501] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [   10.455691] random: mktemp: uninitialized urandom read (12 bytes read, 56 bits of entropy available)
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [   10.530230] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [   10.610655] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [   10.659652] EXT4-fs (sda1): resized filesystem to 7864064
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [   11.075834] init: failsafe main process (1094) killed by TERM signal
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f instance-setup: INFO Running set_multiqueue.
Aug 12 18:34:53 travis-job-e8891504-1 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 12 18:34:53 travis-job-e8891504-1d66-40db-974c-98cde7d2091f instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 12 18:34:54 travis-job-e8891504-1d66-40db-974c-98cde7d2091f google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 12 18:34:54 travis-job-e8891504-1d66-40db-974c-98cde7d2091f google-accounts: INFO Starting Google Accounts daemon.
Aug 12 18:34:54 travis-job-e8891504-1d66-40db-974c-98cde7d2091f google-accounts: INFO Creating a new user account for me.
Aug 12 18:34:55 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [   12.562338] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 12 18:34:55 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [   12.598409] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 12 18:34:55 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [   12.611882] random: nonblocking pool is initialized
Aug 12 18:34:55 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [   12.665318] Initializing XFRM netlink socket
Aug 12 18:34:55 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [   12.673659] Netfilter messages via NETLINK v0.30.
Aug 12 18:34:55 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [   12.676820] ctnetlink v0.93: registering with nfnetlink.
Aug 12 18:34:55 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [   12.718156] floppy0: no floppy controllers found
Aug 12 18:34:55 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [   12.718396] work still pending
Aug 12 18:34:55 travis-job-e8891504-1d66-40db-974c-98cde7d2091f pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 12 18:34:55 travis-job-e8891504-1d66-40db-974c-98cde7d2091f pollinate: To re-seed this system again, use the -r|--reseed option
Aug 12 18:34:56 travis-job-e8891504-1d66-40db-974c-98cde7d2091f cron[1629]: (CRON) INFO (pidfile fd = 3)
Aug 12 18:34:56 travis-job-e8891504-1d66-40db-974c-98cde7d2091f cron[1658]: (CRON) STARTUP (fork ok)
Aug 12 18:34:56 travis-job-e8891504-1d66-40db-974c-98cde7d2091f pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 12 1cker0: port 1(vethcebf9dd) entered forwarding state
Aug 12 18:40:07 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [  324.655189] docker0: port 1(vethcebf9dd) entered disabled state
Aug 12 18:40:07 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [  324.750856] cgroup: docker-runc (4740) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 12 18:40:07 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [  324.750859] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 12 18:40:07 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [  324.834508] eth0: renamed from veth9bf217c
Aug 12 18:40:07 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [  324.873444] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 12 18:40:07 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [  324.874556] docker0: port 1(vethcebf9dd) entered forwarding state
Aug 12 18:40:07 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [  324.874571] docker0: port 1(vethcebf9dd) entered forwarding state
Aug 12 18:40:07 travis-job-e8891504-1d66-40db-974c-98cde7d2091f kernel: [  324.874597] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 12 18:40:10 travis-job-e8891504-1d66-40db-974c-98cde7d2091f ntpd[1768]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 12 18:40:10 travis-job-e8891504-1d66-40db-974c-98cde7d2091f ntpd[1768]: Listen normally on 6 docker0 fe80::42:6aff:fefc:ecab UDP 123
Aug 12 18:40:10 travis-job-e8891504-1d66-40db-974c-98cde7d2091f ntpd[1768]: Listen normally on 7 docker0 fd64-unknown-linux-gnu/bin
43692 ./src/libcompiler_builtins
42624 ./src/libcompiler_builtins/compiler-rt
40920 ./src/llvm/lib/Target
36080 ./.git/modules/src/libcompiler_builtins
