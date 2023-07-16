plain
[00:49:48] ....................................................................................................
[00:49:51] ....................................................................................................
[00:49:54] ....................................................................................................
[00:49:57] ....................................................................................................
[00:49:59] ..............iiiiiiiii.............................................................................
[00:50:05] ....................................................................................................
[00:50:09] ....................i...............................................................................
[00:50:12] ...............................i....................................................................
[00:50:15] ....................................................................................................
---
[01:12:07] .....i......i...i......i............................................................................
[01:12:13] ....................................................................................................
[01:12:25] ..................iiii........ii....................................................................
[01:12:30] ....................................................................................................
[01:12:42] ..................................................................iiii.............................F
[01:12:58] ..F.................................................................................................

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04f7a414
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:070dba21
$ sudo tail -n 500 /var/log/syslog
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 11 11:05:47 travis-job-50)
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 11 11:05:47 travis-job-50321723 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] console [ttyS0] enabled
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.350962] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.352548] pid_max: default: 32768 minimum: 301
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.353538] ACPI: Core revision 20150930
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.360138] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.361168] Security Framework initialized
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.361737] Yama: becoming mindful.
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.362354] AppArmor: AppArmor disabled by boot time parameter
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.365154] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.374441] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.378937] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 11 11:05:47 travis-job-50321723-f046-43. node  #0, CPUs:      #1
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.584445] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.587823]  #2
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.588395] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.591679]  #3
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.592134] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.595471] x86: Booted up 1 node, 4 CPUs
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.596210] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.598796] devtmpfs: initialized
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.603222] evm: security.selinux
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.603753] evm: security.SMACK64
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.604291] evm: security.SMACK64EXEC
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.604880] evm: security.SMACK64TRANSMUTE
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    0.605588] evm: security.SMACK64MMAP
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: c4c3e78 kernel: [    2.970855] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    2.973201] ACPI: Sleep Button [SLPF]
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    2.975644] GHES: HEST is not enabled!
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    2.980219] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    2.982665] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    2.991714] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    2.994945] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.005640] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.030239] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.056310] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.083184] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.109566] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baudstered to indicate activity on CPUs
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.262100] NET: Registered protocol family 10
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.265746] NET: Registered protocol family 17
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.268702] Key type dns_resolver registered
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.272174] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.275421] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.279252] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.282842] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.287030] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.293845] registered taskstats version 1
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.297083] Loading compiled-in X.509 certificates
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.301439] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.309796] zswap0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.615801]  sda: sda1
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.618683] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.913866] tsc: Refined TSC clocksource calibration: 2600.006 MHz
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    3.918592] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a42068c7, max_idle_ns: 440795285103 ns
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    4.300159] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    6.457921] floppy0: no floppy controllers found
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    7.625808] raid6: sse2x1   gen()  8608 MB/s
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    7.693754] raid6: sse2x1   xor()  6413 MB/s
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    7.761809] raid6: sse2x2   gen() 10677 MB/s
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    7.829762] raid6: sse2x2   xor()  7084 MB/s
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    7.897788] raid6: sse2x4   gen() 12434 MB/s
Aug 11 11:05:47 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [    7.965856] raid6: sse2x4   xor()  8arting Google Accounts daemon.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Creating a new user account for me.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Created user account me.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Creating a new user account for henrik.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Created user account henrik.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Creating a new user account for emma.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Created user account emma.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Creating a new user account for igor.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Created user account igor.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Created user account konstantinhaase.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Creating a new user account for aj.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Created user account aj.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Creating a new user account for solarce.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Created user account solarce.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Creating a new user account for asari.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [   12.813648] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [   12.817913] Bridge firewalling registered
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [   12.830459] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 11 11:05:48 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Created user account asari.
Aug 11 11:05:49 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Creating a new user account for bogdana.
Aug 11 11:05:49 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [   12.870794] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 11 11:05:49 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Created user account bogdana.
Aug 11 11:05:49 travis-job-50321723-f046-4358-a17d-1debec4c3e78 google-accounts: INFO Creating a new user account for konstantin.
Aug 11 11:05:49 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [   12.942455] Initializing XFRM netlink socket
Aug 11 11:05:49 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [   12.952141] Netfilter messages via NETLINK v0.30.
Aug 11 11:05:49 travis-job-50321723-f046-4358-a17d-1debec4c3e78 kernel: [   12.955685] ctnetlink v0.93
none            4.0K     0  4.0K   0% /sys/fs/cgroup
none            5.0M     0  5.0M   0% /run/lock
none            7.4G     0  7.4G   0% /run/shm
none            100M     0  100M   0% /run/user
