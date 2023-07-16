plain
[00:53:39] ....................................................................................................
[00:53:43] ................................................................................................i...
[00:53:46] ....................................................................................................
[00:53:49] ....................................................................................................
[00:53:52] .............................................iiiiiiiii..............................................
[00:53:57] ....................................................................................................
[00:54:01] ....................................................................................................
[00:54:04] ........................i...........................................................................
[00:54:07] ...........................i...............................................i.i..ii..................
---
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:05] 
[01:02:05] running 29 tests
[01:02:30] .......F.....................
[01:02:30] 
[01:02:30] ---- [ui] ui-fulldeps/invalid-punct-ident-4.rs stdout ----
[01:02:30] diff of stderr:
[01:02:30] 
[01:02:30] 
[01:02:30] 2   --> $DIR/invalid-punct-ident-4.rs:16:1
[01:02:30] 3    |
[01:02:30] 4 LL | lexer_failure!(); //~ ERROR proc macro panicked
[01:02:30] +    | ^^^^^^^^^^^^^^^^^ unexpected close delimiter
[01:02:30] 6 
[01:02:30] 7 error: proc macro panicked
[01:02:30] 8   --> $DIR/invalid-punct-ident-4.rs:16:1
[01:02:30] 8   --> $DIR/invalid-punct-ident-4.rs:16:1
[01:02:30] 
[01:02:30] 
[01:02:30] The actual stderr differed from the expected stderr.
[01:02:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/invalid-punct-ident-4/invalid-punct-ident-4.stderr
[01:02:30] To update references, rerun the tests and pass the `--bless` flag
[01:02:30] To only update this specific test, also pass `--test-args invalid-punct-ident-4.rs`
[01:02:30] error: 1 errors occurred comparing output.
[01:02:30] status: exit code: 1
[01:02:30] status: exit code: 1
[01:02:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/invalid-punct-ident-4.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknow15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOO0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] setup_percpu: NR_CPUS:512 ed, 0K cma-reserved)
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] console [ttyS0] enabled
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.976131] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.983717] pid_max: default: 32768 minimum: 301
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.987190] ACPI: Core revision 20150930
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    0.996550] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.002878] Security Framework initialized
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.007638] Yama: becoming mindful.
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.010588] AppArmor: AppArmor disabled by boot time parameter
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.016819] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.030727] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.042516] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.047086] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.053843] Initializing cgroup subsys io
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.056972] Initializing cgroup subsys memory
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.062598] Initializing cgroup subsys devices
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.067039] Initializing cgroup subsys freezer
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.073292] Initffff window]
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.636919] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.642710] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.647088] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.647532] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.685962] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.719181] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.724468] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.736861] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.746870] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.775517] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    1.790314] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel54e3 kernel: [    4.131456] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.135939] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.142272] zbud: loaded
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.144343] VFS: Disk quotas dquot_6.6.0
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.146825] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.151636] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.156361] fuse init (API version 7.23)
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.159527] Key type big_key registered
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.162408] Allocating IMA MOK and blacklist keyrings.
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.173401] Key type asymmetric registered
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.176432] Asymmetric key parser 'x509' registered
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.180679] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.185260] io scheduler noop registered
Aug 15 21:27:05 travis-job-eb27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.553714] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.558459] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.562027] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.569020] registered taskstats version 1
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.572666] Loading compiled-in X.509 certificates
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.577194] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.585210] zswap: loaded using pool lzo/zbud
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.591247] Key type trusted registered
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.600469] Key type encrypted registered
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.603445] ima: No TPM chip found, activating TPM-bypass!
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.607012] evm: HMAC attrs: 039-42b7-a9e2-a283438754e3 kernel: [    4.757628] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.769681] AVX version of gcm_enc/dec engaged.
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.773901] AES CTR mode by8 optimization enabled
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.917427] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.917457] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.917459] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.932815] sd 0:0:1:0: [sda] Write Protect is off
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.936296] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.937032] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.948690]  sda: sda1
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    4.953973] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    5.113300] tsc: Refined TSC clocksource calibration: 2600.000 MHz
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    5.114538] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3c3232d, max_idle_ns: 440795236700 ns
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    5.606640] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    7.717352] floppy0: no floppy controllers found
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    8.901208] raid6: sse2x1   gen()  8488 MB/s
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    8.969195] raid6: sse2x1   xor()  6413 MB/s
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    9.037194] raid6: sse2x2   gen() 10381 MB/s
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    9.105192] raid6: sse2x2   xor()  7086 MB/s
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    9.173197] raid6: sse2x4   gen() 12247 MB/s
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    9.241197] raid6: sse2x4   xor()  8601 MB/s
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    9.244756] raid6: using algorithm sse2x4 gen() 12247 MB/s
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    9.247974] raid6: .... xor() 8601 MB/s, rmw enabled
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    9.250836] raid6: using ssse3x2 recovery algorithm
Aug 15 21:27:05 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [    9.254888] xor: automatic2-a283438754e3 google-accounts: INFO Creating a new user account for aj.
Aug 15 21:27:06 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 google-accounts: INFO Created user account aj.
Aug 15 21:27:06 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 google-accounts: INFO Creating a new user account for asari.
Aug 15 21:27:06 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 google-accounts: INFO Created user account asari.
Aug 15 21:27:06 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 google-accounts: INFO Removing user packer.
Aug 15 21:27:06 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [   13.878146] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 15 21:27:06 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [   13.881560] Bridge firewalling registered
Aug 15 21:27:06 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [   13.891393] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 15 21:27:06 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [   13.919740] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 21:27:06 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [   13.995655] Initializing XFRM netlink socket
Aug 15 21:27:06 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 kernel: [   14.002705] Netfilter messages via NETLINK v0.30.
Aug 15 21:27:06 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 21:27:06 travis-job-eb768934-2d39-42b7-a9e2-a283438754e3 pollinate: To re-seed this system again, use the 4954000 .
2614408 ./obj/build
2008648 ./obj/build/x86_64-unknown-linux-gnu
1171976 ./.git
1058088 ./src
---
149128 ./src/llvm-emscripten/test
148684 ./obj/build/bootstrap/debug/incremental
140192 ./obj/build/x86_64-unknown-linux-gnu/test/ui
134252 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z
134248 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z/s-f3vz9ks8hg-1h09qy4-19p1no4jx31q5
128748 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
125268 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
125264 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
122500 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
