\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lifetime-errors/exunknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:49:18] 
[00:49:18] 
[00:49:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:18] Build completed unsuccessfully in 0:03:17
[00:49:18] Build completed unsuccessfully in 0:03:17
[00:49:18] make: *** [check] Error 1
[00:49:18] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:092fe671
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0007447c
$ sudo tail -n 500 /var/log/syslog
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.0d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.354735] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.358868] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.360112] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.361580] Initializing cgroup subsys io
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.362176] Initializing cgroup subsys memory
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.362988] Initializing cgroup subsys devices
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.363691] Initializing cgroup subsys freezer
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.364431] Initializing cgroup subsys net_cls
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.365100] Initializing cgroup subsys perf_event
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.365957] Initializing cgroup subsys net_prio
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.366628] Initializing cgroup subsys hugetlb
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.367343] Initializing cgroup subsys pids
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.368191] CPU: Physical Processor ID: 0
Augvis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.601967] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.603271] PCI: Using configuration type 1 for base access
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.616924] ACPI: Added _OSI(Module Device)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.617618] ACPI: Added _OSI(Processor Device)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.618386] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.619221] ACPI: Added _OSI(Processor Aggregator Device)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.622537] ACPI: Executed 2 blocks of module-level executable AML code
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.645742] ACPI: Interpreter enabled
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.646655] ACPI: (supports S0 S3 S4 S5)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.647605] ACPI: Using IOAPIC for interrupt routing
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.648479] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.678065] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.679153] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.680242] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.681548] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.684147] PCI host bridge to bus 0000:00
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.684765] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.685681] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.686711] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.687860] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.688892] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.689904] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.690317] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931980f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.775820] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.795793] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.796931] vgaarb: loaded
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.797652] SCSI subsystem initialized
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.798334] libata version 3.00 loaded.
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.798376] ACPI: bus type USB registered
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.799005] usbcore: registered new interface driver usbfs
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.800025] usbcore: registered new interface driver hub
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.800888] usbcore: registered new device driver usb
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.802055] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.803076] dmi: Firmware registration failed.
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.804014] PCI: Using ACPI for IRQ routing
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.804758] PCI: pci_cache_line_size set to 64 bytes
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.804862] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.804864] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.804998] NetLabel: Initializing
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.805535] NetLabel:  domain hash size = 128
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.806186] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.806907] NetLabel:  unlabeled traffic allowed by default
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.808217] amd_nb: Cannot enumerate AMD northbridges
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.809117] clocksource: Switched to clocksource kvm-clock
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.816522] pnp: PnP ACPI init
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.817181] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.817261] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.817306] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    0.817360] pnp 00:03: Plug and Plunters 10737418240 ms ovfl timer
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.815843] hw unit of domain pp0-core 2^-0 Joules
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.816635] hw unit of domain package 2^-0 Joules
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.817364] hw unit of domain dram 2^-0 Joules
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.818135] Scanning for low memory corruption every 60 seconds
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.819491] audit: initializing netlink subsys (disabled)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.820422] audit: type=2000 audit(1533805148.834:1): initialized
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.821595] Initialise system trusted keyring
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.822487] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.823561] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.825642] zbud: loaded
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.826303] VFS: Disk quotas dquot_6.6.0
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.827060] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.840842] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.841920] ACPI: Power Button [PWRF]
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.842488] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.843979] ACPI: Sleep Button [SLPF]
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.845081] GHES: HEST is not enabled!
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.847475] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.848550] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.853166] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.854107] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.859048] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.881555] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    2.904487] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, baseoading compiled-in X.509 certificates
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    3.029806] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    3.031598] zswap: loaded using pool lzo/zbud
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    3.034569] Key type trusted registered
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    3.038765] Key type encrypted registered
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    3.039614] ima: No TPM chip found, activating TPM-bypass!
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    3.040812] evm: HMAC attrs: 0x1
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    3.042134]   Magic number: 14:975:977
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    3.042926] rtc_cmos 00:00: setting system clock to 2018-08-09 08:59:09 UTC (1533805149)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    3.044843] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    3.046055] EDD information not available.
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    3.046942] PM: Hibernation image not present or could not be loaded.
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    3.048335] Freeing unused kernel memory: 1496K
Aug  9 08:59:16 tr-2ef4-40d9-aff4-10931935b9d9 kernel: [    7.485158] raid6: sse2x2   xor()  7328 MB/s
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    7.553138] raid6: sse2x4   gen() 12861 MB/s
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    7.621140] raid6: sse2x4   xor()  9002 MB/s
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    7.622240] raid6: using algorithm sse2x4 gen() 12861 MB/s
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    7.623196] raid6: .... xor() 9002 MB/s, rmw enabled
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    7.624084] raid6: using ssse3x2 recovery algorithm
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    7.626256] xor: automatically using best checksumming function:
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    7.665130]    avx       : 27703.000 MB/sec
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    7.678275] Btrfs loaded
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    7.718438] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    7.719627] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    7.802075] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    7.808889] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    7.810010] EXT4-fs (sda1): recovery complete
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    7.815075] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    8.009975] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    8.118934] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    8.164717] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    8.356381] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    8.889788] random: cloud-init: uninitialized urandom read (32 bytes read, 46 bits of entropy available)
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    9.023496] systemd-udevd[700]: starting version 204
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    9.130591] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    9.172787] intel_rapl: no valid rapl domains found in package 0
Aug  9 08:59:16 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 kernel: [    9artup-script: INFO startup-script: job 1 at Thu Aug  9 12:09:00 2018
Aug  9 08:59:48 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 startup-script: INFO Finished running startup scripts.
Aug  9 08:59:48 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 ec2: 
Aug  9 08:59:48 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 ec2: 
Aug  9 08:59:48 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 ec2: #############################################################
Aug  9 08:59:48 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 08:59:48 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 ec2: 1024 8b:39:76:78:a6:59:5d:9b:a9:a0:7e:eb:c1:f1:5e:14  root@travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 (DSA)
Aug  9 08:59:48 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 ec2: 256 86:cb:62:70:bc:3f:5f:c4:a5:d8:fc:36:17:0c:e8:59  root@travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 (ECDSA)
Aug  9 08:59:48 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 ec2: 256 7f:10:e0:1d:74:23:2d:92:7f:c2:85:e6:49:93:a3:df  root@travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 (ED25519)
Aug  9 08:59:48 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 ec2: 2048 5f:66:71:89:a0:90:5d:1b:12:5a:42:4a:4c:c3:99:b2  root@travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 (RSA)
Aug  9 08:59:48 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 08:59:48 travis-job-880f2d49-2ef4-40d9-aff4-10931935b9d9 ec2: #############################################################
Aug  9 09:01:27 travis-job-880f2d49-2ef4-40d9-aff4-10nown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
112056 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
108984 ./src/llvm/test/CodeGen
98952 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
97908 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release
