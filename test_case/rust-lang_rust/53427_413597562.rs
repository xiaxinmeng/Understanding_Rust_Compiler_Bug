plain
[00:50:22] ....................................................................................................
[00:50:25] ....................................................................................................
[00:50:27] ....................................................................................................
[00:50:30] ...................................................................................................i
[00:50:34] ............................................................................F.......................
[00:50:40] ....................................................................................................
[00:50:43] .............................i......................................................................
[00:50:47] ....................................................................................................
[00:50:50] ....................................................................................................
[00:50:50] ....................................................................................................
[00:50:52] ..........................................................................i.........................
3e-ab0db0d1a8c3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] kvm-clock: using sched offset of 1524297027 cycles
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] Zone ranges:
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000]   Device   empty
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] Movable zone start for each node
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] Early memory node ranges
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
1a8c3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.000000] P930] CPU: Processor Core ID: 0
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.380468] mce: CPU supports 32 MCE banks
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.381386] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.382387] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.385753] Freeing SMP alternatives memory: 32K
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.395687] ftrace: allocating 32185 entries in 126 pages
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.449204] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.450660] smpboot: Max logical packages: 2
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.452221] x2apic enabled
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.454191] Switched APIC routing to physical x2apic.
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.458056] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.567174] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    0.569139] Performance Events: unsupported p6 CPU model 62.926388] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    2.927707] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    2.929463] fuse init (API version 7.23)
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    2.930667] Key type big_key registered
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    2.931475] Allocating IMA MOK and blacklist keyrings.
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    2.933870] Key type asymmetric registered
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    2.934595] Asymmetric key parser 'x509' registered
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    2.935729] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    2.937305] io scheduler noop registered
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    2.937865] io scheduler deadline registered (default)
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    2.938651] io scheduler cfq registered
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    2.939427] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    2.940268] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 16 15:10:25 travis-j15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.008016] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.031794] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.056346] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.061040] Linux agpgart interface v0.103
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.064971] loop: module loaded
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.066198] libphy: Fixed MDIO Bus: probed
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.067451] tun: Universal TUN/TAP device driver, 1.6
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.068831] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.109859] PPP generic driver version 2.4.2
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.111358] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.113305] ehci-pci: EHCI PCI platform driver
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.114561] ehci-platform: EHCI generic platform driver
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.116480] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.118042] ohci-pci: OHCI PCI platform driver
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.119720] ohci-platform: OHCI generic platform driver
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.121302] uhci_hcd: USB Universal Host Controller Interface driver
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.123267] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.126610] i8042: Warning: Keylock active
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.129146] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.130752] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.132702] mousedev: PS/2 mouse device common for all mice
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.134797] rtc_cmos 00:00: RTC can wake from S4
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.136685] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.138686] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.140373] i2c /dev entries driver
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.141631] device-mapper: uevent: version 1.0.3
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.143115] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.145480] ledtrig-cpu: registered to indicate activity on CPUs
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.147738] NET: Registered protocol family 10
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.149418] NET: Registered protocol family 17
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.150651] Key type dns_resolver registered
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.152456] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.154306] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.156123] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.157182] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.159106] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.161890] -job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.189531] Freeing unused kernel memory: 1496K
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.190443] Write protecting the kernel read-only data: 14336k
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.192672] Freeing unused kernel memory: 1956K
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.193748] Freeing unused kernel memory: 92K
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.208420] systemd-udevd[118]: starting version 204
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.272793] scsi host0: Virtio SCSI HBA
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.276752] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.286090] AVX2 version of gcm_enc/dec engaged.
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.287607] AES CTR mode by8 optimization enabled
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.324050] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.324089] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.326348] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    3.32rnel: [    8.074269] EXT4-fs (sda1): write access will be enabled during recovery
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    8.143850] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    8.150710] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    8.151610] EXT4-fs (sda1): recovery complete
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    8.156553] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    8.362504] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    8.477364] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    8.532114] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    8.718948] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    9.288667] random: cloud-init: uninitialized urandom read (32 bytes read, 44 bits of entropy available)
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [    9.418498] systemd-udevd[702]: starting version 204
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-aueues/tx-3/xps_cpus
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 google-accounts: INFO Starting Google Accounts daemon.
Aug 16 15:10:25 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 google-accounts: INFO Creating a new user account for me.
Aug 16 15:10:26 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 google-accounts: INFO Created user account me.
Aug 16 15:10:26 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 google-accounts: INFO Creating a new user account for bogdana.
Aug 16 15:10:26 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 google-accounts: INFO Created user account bogdana.
Aug 16 15:10:26 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 google-accounts: INFO Creating a new user account for aj.
Aug 16 15:10:26 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [   11.779418] random: nonblocking pool is initialized
Aug 16 15:10:26 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 google-accounts: INFO Created user account aj.
Aug 16 15:10:26 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 google-accounts: INFO Creating a new user account for asari.
Aug 16 15:10:26 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 google-accounts: INFO Created user account asari.
Aug 16 15:10:26 travis-job-9765da95-bd86-45a6-be3e-IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 16 15:14:30 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [  255.603268] device vethbd23e13 entered promiscuous mode
Aug 16 15:14:30 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [  255.603337] docker0: port 1(vethbd23e13) entered forwarding state
Aug 16 15:14:30 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [  255.603345] docker0: port 1(vethbd23e13) entered forwarding state
Aug 16 15:14:30 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [  255.603783] docker0: port 1(vethbd23e13) entered disabled state
Aug 16 15:14:30 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [  255.698393] cgroup: docker-runc (4904) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 16 15:14:30 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [  255.698398] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 16 15:14:30 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [  255.766024] eth0: renamed from veth69da196
Aug 16 15:14:30 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [  255.801734] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 16 15:14:30 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [  255.802740] docker0: port 1(vethbd23e13) entered forwarding state
Aug 16 15:14:30 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [  255.802755] docker0: port 1(vethbd23e13) entered forwarding state
Aug 16 15:14:30 travis-job-9765da95-bd86-45a6-be3e-ab0db0d1a8c3 kernel: [  255.802773] IPv6: ADD4553012 .
2319136 ./obj/build
1713368 ./obj/build/x86_64-unknown-linux-gnu
1175068 ./.git
1058260 ./src
---
151200 ./src/tools/clang
149124 ./src/llvm-emscripten/test
148684 ./obj/build/bootstrap/debug/incremental
134252 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z
134248 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z/s-f3wsksgf3j-1uwr8wa-19p1no4jx31q5
128756 ./obj/build/x86_64-unknown-linux-gnvers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0dc2f9a6
$ dmesg | grep -i kill
