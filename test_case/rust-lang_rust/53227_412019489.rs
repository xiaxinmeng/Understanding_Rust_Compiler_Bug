plain
[01:03:47] ....................................................................................................
[01:03:50] ....................................................................................................
[01:03:52] ....................................................................................................
[01:03:55] ....................................................................................................
[01:03:58] ................iiiiiiiii...........................................................................
[01:04:04] ....................................................................................................
[01:04:07] .....................i..............................................................................
[01:04:10] ................................i...................................................................
[01:04:13] ....................................................................................................
---
[01:32:24] travis_fold:end:stage0-linkchecker

[01:32:24] travis_time:end:stage0-linkchecker:start=1533890888415555506,finish=1533890890948524543,duration=2532969037

[01:32:24] std/marker/trait.Unpin.html:2: broken link - std/mem/struct.PinMut.html
[01:32:33] core/marker/trait.Unpin.html:2: broken link - core/mem/struct.PinMut.html

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09bc8194
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:395c4d41
$ sudo tail -n 500 /var/log/syslog
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] Using GB pages for direct mapping
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 10 07:13:20 travis-job-d1055a97-8110--clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] kvm-clock: using sched offset of 1453495486 cycles
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] Zone ranges:
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000]   Device   empty
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] Movable zone start for each node
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] Early memory node ranges
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000]   node   0: [mem 0x0000000100000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] PM: Registered:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] console [ttyS0] enabled
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.323080] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.324305] pid_max: default: 32768 minimum: 301
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.325042] ACPI: Core revision 20150930
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.331356] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.332669] Security Framework initialized
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.333521] Yama: a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.638474] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.667598] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.668843] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.669973] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.670926] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.673462] PCI host bridge to bus 0000:00
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.674251] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.675265] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.676411] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.677619] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.679234] pci_bus 0000:00: root bus resource [bus    0.800642] dmi: Firmware registration failed.
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.801485] PCI: Using ACPI for IRQ routing
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.802160] PCI: pci_cache_line_size set to 64 bytes
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.802254] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.802256] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.802374] NetLabel: Initializing
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.802887] NetLabel:  domain hash size = 128
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.803598] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.804512] NetLabel:  unlabeled traffic allowed by default
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.805449] amd_nb: Cannot enumerate AMD northbridges
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.806413] clocksource: Switched to clocksource kvm-clock
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.813556] pnp: PnP ACPI init
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.814146] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.814217] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.814262] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.814312] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.814353] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.814393] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.814447] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.814609] pnp: PnP ACPI: found 7 devices
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.822152] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.823695] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.823697] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.823699] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.823701] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.823733] NET: Registered protocol family 2
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.824835] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.826196] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.827794] TCP: Hash tables configured (established 131072 bind 65536)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.828951] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.830128] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.832073] NET: Registered protocol family 1
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.832871] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.833799] PCI: CLS 0 bytes, default 64
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    0.833849] Unpacking initramfs...
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.813199] Freeing initrd memory: 21432K
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.813879] PCI-DMA: Using software bounce bufferinel: [    2.828576] zbud: loaded
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.829402] VFS: Disk quotas dquot_6.6.0
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.830093] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.831396] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.832930] fuse init (API version 7.23)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.833849] Key type big_key registered
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.834534] Allocating IMA MOK and blacklist keyrings.
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.836693] Key type asymmetric registered
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.837345] Asymmetric key parser 'x509' registered
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.838189] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.839512] io scheduler noop registered
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.840188] io scheduler deadline registered (default)
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.841039] io scheduler cfq registered
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.841733] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.842948] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.844009] intel_idle: does not run on family 6 model 45
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.844099] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.845126] ACPI: Power Button [PWRF]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.846004] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.847248] ACPI: Sleep Button [SLPF]
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.848296] GHES: HEST is not enabled!
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.850452] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.851446] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.856539] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.857725] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.863231] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.885579] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.908307] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.931302] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.954236] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.958109] Linux agpgart interface v0.103
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.961093] loop: module loaded
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.961909] libphy: Fixed MDIO Bus: probed
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.963081] tun: Universal TUN/TAP device driver, 1.6
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.964092] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.999179] PPP generic driver version 2.4.2
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    2.999947] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3os as rtc0
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.016262] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.017344] i2c /dev entries driver
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.017885] device-mapper: uevent: version 1.0.3
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.018845] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.020512] ledtrig-cpu: registered to indicate activity on CPUs
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.022388] NET: Registered protocol family 10
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.023428] NET: Registered protocol family 17
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.024103] Key type dns_resolver registered
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.025327] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.026574] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.027361] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.028883] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 07:13:20 travis-job-d1055ad97 kernel: [    3.054246] EDD information not available.
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.055527] PM: Hibernation image not present or could not be loaded.
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.057026] Freeing unused kernel memory: 1496K
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.057816] Write protecting the kernel read-only data: 14336k
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.059925] Freeing unused kernel memory: 1956K
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.061253] Freeing unused kernel memory: 92K
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.075763] systemd-udevd[118]: starting version 204
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.131356] scsi host0: Virtio SCSI HBA
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.138955] AVX version of gcm_enc/dec engaged.
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.139694] AES CTR mode by8 optimization enabled
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.140534] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.176862] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 10 07:13:20 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [    3.176902] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 G 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 10 07:13:21 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 10 07:13:21 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [   11.135619] random: nonblocking pool is initialized
Aug 10 07:13:21 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug 10 07:13:21 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 10 07:13:21 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 07:13:21 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 google-accounts: INFO Starting Google Accounts daemon.
Aug 10 07:13:21 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 google-accounts: INFO Creating a new user account for me.
Aug 10 07:13:21 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 google-accounts: INFO Created user account me.
Aug 10 07:13:21 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 google-accounts: INFO Creating a new user account for bogdana.
Aug 10 07:13:21 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 07:13:21 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 cron[1402]: (CRON) INFO (pidfile fd = 3)
Aug 10 07:13:21 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 cron[1448]: (CRON) STARTUP (fork ok)
Aug 10 07:13:21 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 pollinate: To re-seed this system again, use thug 10 07:13:51 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 ntpd[1802]: proto: precision = 0.107 usec
Aug 10 07:13:51 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 ntpd[1802]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 10 07:13:51 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 ntpd[1802]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 10 07:13:51 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 ntpd[1802]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 10 07:13:51 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 ntpd[1802]: Listen normally on 3 eth0 10.20.0.68 UDP 123
Aug 10 07:13:51 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 ntpd[1802]: Listen normally on 3 eth0 10.20.0.68 UDP 123
Aug 10 07:13:51 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 ntpd[1802]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 10 07:13:51 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 ntpd[1802]: peers refreshed
Aug 10 07:13:51 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 ntpd[1802]: Listening on routing socket on fd #21 for interface updates
Aug 10 07:13:51 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [   41.876126] init: plymouth-upstart-bridge main process ended, respawning
Aug 10 07:13:52 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 startup-script: INFO Found startup-script in metadata.
Aug 10 07:13:52 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 10 07:13:52 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 10 07:13:52 travis-job-d travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [  155.973520] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 10 07:17:01 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 CRON[4126]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 10 07:31:33 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 1103.346066] device vethf8ef5fa entered promiscuous mode
Aug 10 07:31:33 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 1103.346133] docker0: port 1(vethf8ef5fa) entered forwarding state
Aug 10 07:31:33 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 1103.346143] docker0: port 1(vethf8ef5fa) entered forwarding state
Aug 10 07:31:33 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 1103.346690] docker0: port 1(vethf8ef5fa) entered disabled state
Aug 10 07:31:33 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 1103.438340] cgroup: docker-runc (4965) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 10 07:31:33 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 1103.438343] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 10 07:31:33 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 1103.509735] eth0: renamed from veth3a139d5
Aug 10 07:31:33 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 1103.554042] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 10 07:31:33 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 1103.555526] docker0: port 1(vethf8ef5fa) entered forwarding state
Aug 10 07:31:33 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 1103.555558] docker0: port 1(vethf8ef5fa) entered forwarding state
Aug 10 07:31:33 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 1103.555586] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 10 07:31:36 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 ntpd[1802]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 10 07:31:36 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 ntpd[1802]: Listen normally on 6 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 10 07:31:36 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 ntpd[1802]: Listen normally on 7 docker0 fe80::42:feff:fed4:ccc0 UDP 123
Aug 10 07:31:36 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 ntpd[1802]: peers refreshed
Aug 10 07:31:36 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 ntpd[1802]: new interface(s) found: waking up resolver
Aug 10 07:31:48 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 1118.561321] docker0: port 1(vethf8ef5fa) entered forwarding state
Aug 10 08:17:01 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 CRON[22012]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 10 08:20:10 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 4020.141317] traps: a[5343] trap invalid opcode ip:560059d47b4b sp:7ffd12bd0e50 error:0 in a[560059d44000+6000]
Aug 10 08:20:25 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 4034.927747] traps: a[8174] trap invalid opcode ip:7f360f7e30c1 sp:7ffd0a59c570 error:0 in libstd-2339b911e3c09de8.so[7f360f784000+16e000]
Aug 10 08:20:25 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 4034.982275] traps: a[8180] trap invalid opcode ip:7ff4d911d0c1 sp:7ffc28300640 error:0 in libstd-2339b911e3c09de8.so[7ff4d90be000+16e000]
Aug 10 08:21:47 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 4117.034345] traps: a[23010] trap invalid opcode ip:55c0fef4fd98 sp:7ffea4f1dce0 error:0 in a[55c0fef4d000+4000]
Aug 10 08:24:31 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 4280.941065] a[19040]: segfault at 0 ip 00005586523d7463 sp 00007fffa3db2bc0 error 6 in a[5586523d4000+5000]
Aug 10 08:24:40 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 4290.184047] a[19794]: segfault at 1 ip 000055bc16720b8c sp 00007ffdc6c0cc20 error 6 in a[55bc1671e000+4000]
Aug 10 08:24:44 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 4294.214242] traps: a[20164] trap invalid opcode ip:55e98a95c42c sp:7fffd4f51450 error:0 in a[55e98a959000+7000]
Aug 10 08:48:26 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 5715.905436] docker0: port 1(vethf8ef5fa) entered disabled state
Aug 10 08:48:26 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 5715.905507] veth3a139d5: renamed from eth0
Aug 10 08:48:26 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 5715.968331] docker0: port 1(vethf8ef5fa) entered disabled state
Aug 10 08:48:26 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 5715.969933] device vethf8ef5fa left promiscuous mode
Aug 10 08:48:26 travis-job-d1055a97-8110-4c26-83fc-e07cd1059d97 kernel: [ 5715.969941] docker0: port 1(vethf8ef5fa) entered disabled state
travis_fold:end:after_failu
