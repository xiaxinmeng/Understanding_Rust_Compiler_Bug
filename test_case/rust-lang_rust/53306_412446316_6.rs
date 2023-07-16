\n\nFor more information on the rust ownership system, take a look at\nhttps://doc.rust-lang.org/stable/book/references-and-borrowing.html.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-slice-pattern-element-loan.rs",ui/borrowck/borrowck-slice-pattern-element-loan.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:44:56] 
[00:44:56] 
[00:44:56] failures:
[00:44:56]     [ui] ui/borrowck/borrowck-slice-pattern-element-loan.rs
[00:44:56]     [ui] ui/borrowck/borrowck-slice-pattern-element-loan.rs
[00:44:56] 
[00:44:56] test result: FAILED. 2244 passed; 1 failed; 7 ignored; 0 measured; 0 filtered out
[00:44:56] 
[00:44:56] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:44:56] 
[00:44:56] 
[00:44:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm--9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 13 07:46:54 travis-job-9f531855-e1352=-1
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.692624] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.695629] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.700440] x86: Booting SMP configuration:
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.702063] .... node  #0, CPUs:      #1
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.703666] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.709139]  #2
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.710160] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.715928]  #3
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.717006] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.722496] x86: Booted up 1 node, 4 CPUs
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.723982] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.727491] devtmpfs: initialized
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [ 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.895819] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.918596] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.940927] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.943901] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.952817] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.959924] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.978787] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.986863] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    0.994093] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    1.014078] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    1.018151] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c333498] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.336793] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.339269] hw unit of domain pp0-core 2^-0 Joules
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.340574] hw unit of domain package 2^-0 Joules
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.342152] hw unit of domain dram 2^-0 Joules
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.343952] Scanning for low memory corruption every 60 seconds
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.346734] audit: initializing netlink subsys (disabled)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.348389] audit: type=2000 audit(1534146406.147:1): initialized
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.350661] Initialise system trusted keyring
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.352910] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.354727] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.358094] zbud: loaded
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.360019] VFS: Disk quotas dquot_6.6.0
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.361215] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.363508] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.365696] fuse init (API version 7.23)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.367030] Key type big_key registered
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.368557] Allocating IMA MOK and blacklist keyrings.
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.372327] Key type asymmetric registered
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.373875] Asymmetric key parser 'x509' registered
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.375569] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.378127] io scheduler noop registered
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.379765] io scheduler deadline registered (default)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.381451] io scheduler cfq registered
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.382661] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 13 07:46:54 travis-job-9f531855-e135-40artc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.622929] i2c /dev entries driver
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.625170] device-mapper: uevent: version 1.0.3
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.627758] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.632703] ledtrig-cpu: registered to indicate activity on CPUs
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.636651] NET: Registered protocol family 10
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.638851] NET: Registered protocol family 17
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.640725] Key type dns_resolver registered
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.642840] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.645743] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.647860] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.649508] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.651457] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.654424] registered taskstats version 1
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.655806] Loading compiled-in X.509 certificates
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.657975] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.661525] zswap: loaded using pool lzo/zbud
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.664981] Key type trusted registered
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.670352] Key type encrypted registered
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.671628] ima: No TPM chip found, activating TPM-bypass!
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.673123] evm: HMAC attrs: 0x1
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.674967]   Magic number: 14:963:773
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.676215] acpi LNXCPU:90: hash matches
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.677813] rtc_cmos 00:00: setting system clock to 2018-08-13 07:46:46 UTC (1534146406)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    3.681096] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 13 07:46:54 travis-job-9f531855-e135-440a6-a20c-c8dea443eb80 kernel: [    9.848412] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [    9.949924] ppdev: user-space parallel port driver
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [   10.060449] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [   10.115278] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [   10.183230] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [   10.351970] random: cloud-init: uninitialized urandom read (32 bytes read, 60 bits of entropy available)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [   10.604468] random: mktemp: uninitialized urandom read (12 bytes read, 62 bits of entropy available)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [   10.679599] random: mktemp: uninitialized urandom read (6 bytes read, 63 bits of entropy available)
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [   10.752319] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [   10.793980] EXT4-fs (sda1): resized filesystem to 7864064
Aug 13 07:46:54 travis-jance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 google-accounts: INFO Starting Google Accounts daemon.
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 google-accounts: INFO Creating a new user account for me.
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [   11.906769] random: nonblocking pool is initialized
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 google-accounts: INFO Created user account me.
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 google-accounts: INFO Created user account me.
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 google-accounts: INFO Creating a new user account for henrik.
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 google-accounts: INFO Created user account henrik.
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 google-accounts: INFO Creating a new user account for emma.
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 google-accounts: INFO Created user account emma.
Aug 13 07:46:54 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 google-accounts: INFO Creating a new user account for igor.
Aug 13 07:46:55 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 google-accounts: INFO Created user account igis-job-9f531855-e135-40a6-a20c-c8dea443eb80 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 13 07:47:26 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 startup-script: INFO startup-script: Return code 0.
Aug 13 07:47:26 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 startup-script: INFO Finished running startup scripts.
Aug 13 07:47:26 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 ec2: 
Aug 13 07:47:26 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 ec2: 
Aug 13 07:47:26 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 ec2: #############################################################
Aug 13 07:47:26 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 13 07:47:26 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 ec2: 1024 26:21:3f:a3:0f:db:69:81:e8:84:fe:9e:4d:44:5f:23  root@travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 (DSA)
Aug 13 07:47:26 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 ec2: 256 cb:dd:f8:d7:12:17:20:50:ca:ea:57:a5:bd:a2:48:f9  root@travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 (ECDSA)
Aug 13 07:47:26 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 ec2: 256 ae:b8:96:74:29:8b:75:3c:7b:6e:eb:de:39:b1:47:40  root@travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 (ED25519)
Aug 13 07:47:26 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 ec2: 2048 c7:0d:c5:5b:b1:13:e9:53:9f:b2:09:9f:50:4b:db:6b  root@travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 (RSA)
Aug 13 07:47:26 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 13 07:47:26 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 ec2: #############################################################
Aug 13 07:48:50 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [  127.617358] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 13 07:49:55 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [  192.516395] device vethc7a2afa entered promiscuous mode
Aug 13 07:49:55 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [  192.516500] docker0: port 1(vethc7a2afa) entered forwarding state
Aug 13 07:49:55 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [  192.516507] docker0: port 1(vethc7a2afa) entered forwarding state
Aug 13 07:49:55 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [  192.516929] docker0: port 1(vethc7a2afa) entered disabled state
Aug 13 07:49:55 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [  192.619040] cgroup: docker-runc (4868) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 13 07:49:55 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [  192.619042] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 13 07:49:55 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [  192.690578] eth0: renamed from veth1000ec0
Aug 13 07:49:55 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [  192.726298] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 13 07:49:55 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [  192.727842] docker0: port 1(vethc7a2afa) entered forwarding state
Aug 13 186] device vethc7a2afa left promiscuous mode
Aug 13 08:33:48 travis-job-9f531855-e135-40a6-a20c-c8dea443eb80 kernel: [ 2825.524190] docker0: port 1(vethc7a2afa) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:1823ad2b
---
travis_time:end:0a443933:start=1534149230261817211,finish=1534149230269363085,duration=7545874
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a59ee70
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:002c5f22
travis_time:start:002c5f22
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:12046590

