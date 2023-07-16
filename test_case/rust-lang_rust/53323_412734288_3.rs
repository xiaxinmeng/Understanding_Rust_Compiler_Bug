\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for evcgroup subsys net_cls
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.393864] Initializing cgroup subsys perf_event
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.394550] Initializing cgroup subsys net_prio
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.395179] Initializing cgroup subsys hugetlb
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.395789] Initializing cgroup subsys pids
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.396652] CPU: Physical Processor ID: 0
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.397208] CPU: Processor Core ID: 0
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.398918] mce: CPU supports 32 MCE banks
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.399656] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.400641] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.403951] Freeing SMP alternatives memory: 32K
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.414596] ftrace: allocating 32185 entries in 126 pages
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.472897] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.474039] smpboot: Max lo enabled
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.699908] ACPI: (supports S0 S3 S4 S5)
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.700514] ACPI: Using IOAPIC for interrupt routing
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.701218] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.730664] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.731901] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.732922] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.733913] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.736208] PCI host bridge to bus 0000:00
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.736781] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.737770] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    0.739187] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff windowb6646 kernel: [    3.286090] PPP generic driver version 2.4.2
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.288619] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.291537] ehci-pci: EHCI PCI platform driver
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.293512] ehci-platform: EHCI generic platform driver
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.295770] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.298223] ohci-pci: OHCI PCI platform driver
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.300036] ohci-platform: OHCI generic platform driver
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.302428] uhci_hcd: USB Universal Host Controller Interface driver
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.305012] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.308197] i8042: Warning: Keylock active
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.310831] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.312547] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.314614] mousedev: PS/2 mAug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.492165] AES CTR mode by8 optimization enabled
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.516753] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.541599] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.543447] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.545921] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.548094] sd 0:0:1:0: [sda] Write Protect is off
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.549572] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.549733] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.554970]  sda: sda1
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    3.557343] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    4.044287] tsc: Refined TSC clocksource calibration: 2499.782 MHz
Aug 14 01:48:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [    4.046480] clocksource: tsc: mask: 0xffffffffffffffff maFO Creating a new user account for aj.
Aug 14 01:48:47 travis-job-02d68d94-8105-4165-8348-96845b3b6646 google-accounts: INFO Created user account aj.
Aug 14 01:48:47 travis-job-02d68d94-8105-4165-8348-96845b3b6646 google-accounts: INFO Creating a new user account for asari.
Aug 14 01:48:47 travis-job-02d68d94-8105-4165-8348-96845b3b6646 google-accounts: INFO Created user account asari.
Aug 14 01:48:47 travis-job-02d68d94-8105-4165-8348-96845b3b6646 google-accounts: INFO Removing user packer.
Aug 14 01:48:47 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [   12.438049] random: nonblocking pool is initialized
Aug 14 01:48:47 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [   12.684400] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 01:48:47 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [   12.689109] Bridge firewalling registered
Aug 14 01:48:47 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [   12.701473] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 14 01:48:47 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [   12.739332] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 01:48:47 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [   12.816805] Initializing XFRM netlink socket
Aug 14 01:48:47 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [   12.825182] Netfilter messages via NETLINK v0.30.
Aug 14 01:48:47 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [   12.829610] ctnetlink v0.93: registering with nfnetlink.
Aug 14 01:48:47 travis-job-02d68(RSA)
Aug 14 01:49:18 travis-job-02d68d94-8105-4165-8348-96845b3b6646 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 14 01:49:18 travis-job-02d68d94-8105-4165-8348-96845b3b6646 ec2: #############################################################
Aug 14 01:49:46 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [   71.836974] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 14 01:50:59 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [  144.130368] device veth6543d8e entered promiscuous mode
Aug 14 01:50:59 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [  144.130420] docker0: port 1(veth6543d8e) entered forwarding state
Aug 14 01:50:59 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [  144.130429] docker0: port 1(veth6543d8e) entered forwarding state
Aug 14 01:50:59 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [  144.130886] docker0: port 1(veth6543d8e) entered disabled state
Aug 14 01:50:59 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [  144.236582] cgroup: docker-runc (4808) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 01:50:59 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [  144.236586] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 01:50:59 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [  144.308293] eth0: renamed from veth0b1ad33
Aug 14 01:50:59 travis-job-02d68d94-8105-4165-8348-96845b3b6646 kernel: [  144.347189] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 01:50:59 travis-job-02d68d94-8105-416piler_builtins/modules/compiler-rt/objects
34196 ./obj/build/x86_64-unknown-linux-gnu/doc/core/arch
33884 ./src/llvm-emscripten/lib/Target
32352 ./src/libcompiler_builtins/compiler-rt/test
31728 ./src/llvm/test/tools
