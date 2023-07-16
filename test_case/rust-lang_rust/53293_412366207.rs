plain
[00:00:00] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins) registered for path 'src/libcompiler_builtins'
[00:00:00] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
[00:00:00] Submodule 'src/stdsimd' (https://github.com/rust-lang-nursery/stdsimd) registered for path 'src/stdsimd'
[00:00:00] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
[00:00:00] tar: This does not look like a tar archive
[00:00:00] 
[00:00:00] gzip: stdin: not in gzip format
[00:00:00] tar: Child returned status 1
[00:00:00] tar: Error is not recoverable: exiting now
[00:00:00] tar: Error is not recoverable: exiting now
[00:00:00] Submodule 'src/tools/lld' (https://github.com/rust-lang/lld.git) registered for path 'src/tools/lld'
[00:00:00] Submodule 'src/tools/miri' (https://github.com/solson/miri.git) registered for path 'src/tools/miri'
[00:00:00] Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
[00:00:00] Submodule 'src/rust-installer' (https://github.com/rust-lang/rust-installer.git) registered for path 'src/tools/rust-installer'
[00:00:00] Submodule 'src/tools/rustfmt' (https://github.com/rust-lang-nursery/rustfmt.git) registered for path 'src/tools/rustfmt'
[00:00:00] Cloning into '/home/travis/build/rust-lang/rust/src/dlmalloc'...
[00:00:00] tar: This does not look like a tar archive
[00:00:00] gzip: stdin: not in gzip format
[00:00:00] tar: Child returned status 1
[00:00:00] tar: Error is not recoverable: exiting now
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/doc/nomicon'...
---
[00:36:35]    Compiling minifier v0.0.14
[00:36:37]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:37:58]    Compiling rustdoc-tool v0.0.0 (file:///checkout/src/tools/rustdoc)
s: 7645519600211568 ns
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.000000] Policy zone: Normal
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [0.542925] Initializing cgroup subsys freezer
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.544811] Initializing cgroup subsys net_cls
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.546553] Initializing cgroup subsys perf_event
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.548411] Initializing cgroup subsys net_prio
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.550580] Initializing cgroup subsys hugetlb
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.552391] Initializing cgroup subsys pids
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.554149] CPU: Physical Processor ID: 0
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.555815] CPU: Processor Core ID: 0
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.557229] mce: CPU supports 32 MCE banks
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.558854] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.560978] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.566032] Freeing SMP alternatives memory: 32K
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.579279] ftrace: allocating 32185 entries in 126 pages
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    0.640751] smpboot: APIC(0133cbc170c kernel: [    1.188804] usbcore: registered new interface driver hub
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    1.193123] usbcore: registered new device driver usb
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    1.196132] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    1.199502] dmi: Firmware registration failed.
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    1.201725] PCI: Using ACPI for IRQ routing
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    1.203306] PCI: pci_cache_line_size set to 64 bytes
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    1.203417] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    1.203420] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    1.203570] NetLabel: Initializing
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    1.205075] NetLabel:  domain hash size = 128
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    1.206780] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    1.209445] NetLabel:  unlabeled traffic allowed by default
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    1.212623] amd_nb: Cannot enumerate AMD northbridges
Aug 12 18decf-4618-9639-28133cbc170c kernel: [    1.270076] PCI: CLS 0 bytes, default 64
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    1.270987] Unpacking initramfs...
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.487869] Freeing initrd memory: 21432K
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.488655] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.489582] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.491521] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.493407] hw unit of domain pp0-core 2^-0 Joules
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.494276] hw unit of domain package 2^-0 Joules
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.495096] hw unit of domain dram 2^-0 Joules
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.496091] Scanning for low memory corruption every 60 seconds
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.497773] audit: initializing netlink subsys (disabled)
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.498857] audit: type=2000 audit(1534099922.508:1): initialized
Aug 12 18:52:11 travis-job-fae4ec2e-decf-461:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.517449] io scheduler noop registered
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.518088] io scheduler deadline registered (default)
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.519049] io scheduler cfq registered
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.519904] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.520745] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.521786] intel_idle: does not run on family 6 model 45
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.521902] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.523308] ACPI: Power Button [PWRF]
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.523921] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.525225] ACPI: Sleep Button [SLPF]
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.526310] GHES: HEST is not enabled!
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.529040] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.5299452:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.737424] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.738913] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.740218] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.742364] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.745535] registered taskstats version 1
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.747237] Loading compiled-in X.509 certificates
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.749305] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.751819] zswap: loaded using pool lzo/zbud
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.755166] Key type trusted registered
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.760056] Key type encrypted registered
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.761024] ima: No TPM chip found, activating TPM-bypass!
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.762316] evm: HMAC attrs: 0x1
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618rsion of gcm_enc/dec engaged.
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.861749] AES CTR mode by8 optimization enabled
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.901663] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.901692] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.901693] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.901864] sd 0:0:1:0: [sda] Write Protect is off
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.901865] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.901912] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.903283]  sda: sda1
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.904195] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    3.919108] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    4.494844] tsc: Refined TSC clocksource calibration: 2599.768 MHz
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    4.496389] clocksource: t:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    8.410743]    avx       : 21532.000 MB/sec
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    8.429140] Btrfs loaded
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    8.496093] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    8.499670] EXT4-fs (sda1): write access will be enabled during recovery
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    8.602135] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    8.612663] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    8.615212] EXT4-fs (sda1): recovery complete
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    8.622957] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    8.887149] random: init: uninitialized urandom read (12 bytes read, 23 bits of entropy available)
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    9.035798] random: mountall: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    9.104674] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 12 18:52:11 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [    9.355419] random: cloud-init: uniniti18:52:13 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c google-accounts: INFO Creating a new user account for carmen.
Aug 12 18:52:13 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c google-clock-skew: INFO Synced system time with hardware clock.
Aug 12 18:52:13 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 12 18:52:13 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c pollinate: To re-seed this system again, use the -r|--reseed option
Aug 12 18:52:13 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c google-accounts: INFO Created user account carmen.
Aug 12 18:52:13 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c google-accounts: INFO Creating a new user account for maria.
Aug 12 18:52:13 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c google-accounts: INFO Created user account maria.
Aug 12 18:52:13 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c google-accounts: INFO Removing user packer.
Aug 12 18:52:14 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c cron[1708]: (CRON) INFO (pidfile fd = 3)
Aug 12 18:52:14 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 12 18:52:14 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c cron[1743]: (CRON) STARTUP (fork ok)
Aug 12 18:52:14 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c pollinate: To re-seed this system again, use the -r|--reseed option
Aug 12 18:52:14 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c cron[1743]: (CRON) INFO (Running @reboot jobs)
Aug 12 18:52:14 travis-job-fae4ec2e-decf-4618-9639-28133cbc1cbc170c ec2: 256 87:47:10:1f:88:17:45:0b:86:e1:29:43:96:21:f8:ee  root@travis-job-fae4ec2e-decf-4618-9639-28133cbc170c (ECDSA)
Aug 12 18:52:20 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c ec2: 256 09:da:c2:b8:67:e4:38:f2:37:c9:ac:b5:cc:e3:66:6d  root@travis-job-fae4ec2e-decf-4618-9639-28133cbc170c (ED25519)
Aug 12 18:52:20 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c ec2: 2048 99:36:a8:ab:12:f0:55:ee:6b:31:fb:5e:ac:ed:70:64  root@travis-job-fae4ec2e-decf-4618-9639-28133cbc170c (RSA)
Aug 12 18:52:20 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 12 18:52:20 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c ec2: #############################################################
Aug 12 18:52:27 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c ntpdate[2241]: the NTP socket is in use, exiting
Aug 12 18:53:03 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [   64.491089] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 12 18:53:52 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [  112.685051] device vethb2bae78 entered promiscuous mode
Aug 12 18:53:52 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [  112.685157] docker0: port 1(vethb2bae78) entered forwarding state
Aug 12 18:53:52 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [  112.685167] docker0: port 1(vethb2bae78) entered forwarding state
Aug 12 18:53:52 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [  112.685573] docker0: port 1(vethb2bae78) entered disabled state
Aug 12 18:53:52 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [  112.780770] cgroup: docker-133cbc170c kernel: [  127.958688] docker0: port 1(vethb2bae78) entered forwarding state
Aug 12 19:17:01 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c CRON[16314]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 12 19:31:06 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [ 2347.115166] docker0: port 1(vethb2bae78) entered disabled state
Aug 12 19:31:06 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [ 2347.115227] veth223409b: renamed from eth0
Aug 12 19:31:06 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [ 2347.185961] docker0: port 1(vethb2bae78) entered disabled state
Aug 12 19:31:06 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [ 2347.187880] device vethb2bae78 left promiscuous mode
Aug 12 19:31:06 travis-job-fae4ec2e-decf-4618-9639-28133cbc170c kernel: [ 2347.187883] docker0: port 1(vethb2bae78) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:086b2c10
