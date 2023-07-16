plain
[00:40:57] Documenting stage2 std (x86_64-unknown-linux-gnu)
[00:40:58]     Checking core v0.0.0 (file:///checkout/src/libcore)
[00:40:58]  Documenting core v0.0.0 (file:///checkout/src/libcore)
[00:40:58]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:41:10] error: internal compiler error: librustc/ty/query/mod.rs:102: tcx.rendered_const(DefId(0/0:389 ~ core[8787]::num[0]::dec2flt[0]::rawfp[0]::{{impl}}[1]::SIG_BITS[0])) unsupported by its crate
[00:41:10] thread '<unnamed>' panicked at 'Box<Any>', librustc_errors/lib.rs:578:9
[00:41:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:41:10] 
[00:41:10] 
[00:41:10] error: Unrecognized option: 'crate-version'
[00:41:10] error: Could not document `core`.
[00:41:10] 
[00:41:10] Caused by:
[00:41:10] Caused by:
[00:41:10]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name core libcore/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 1)
[00:41:10] warning: build failed, waiting for other jobs to finish...
c4-4a1f-939e-80fda52d12c2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.000000] Policy zone: Normal
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0 kernel: [    0.355293] mce: CPU supports 32 MCE banks
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.356048] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.356792] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.359541] Freeing SMP alternatives memory: 32K
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.368407] ftrace: allocating 32185 entries in 126 pages
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.416678] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.417675] smpboot: Max logical packages: 2
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.418814] x2apic enabled
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.420310] Switched APIC routing to physical x2apic.
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.423803] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.531721] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    0.533213] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kerne travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    2.963124] hw unit of domain package 2^-0 Joules
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    2.963800] hw unit of domain dram 2^-0 Joules
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    2.964564] Scanning for low memory corruption every 60 seconds
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    2.965992] audit: initializing netlink subsys (disabled)
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    2.966807] audit: type=2000 audit(1533655854.041:1): initialized
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    2.967937] Initialise system trusted keyring
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    2.968801] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    2.969666] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    2.971650] zbud: loaded
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    2.972306] VFS: Disk quotas dquot_6.6.0
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    2.972912] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    2.974082] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    2.975346165] ohci-platform: OHCI generic platform driver
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    3.147711] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    3.149332] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    3.152291] i8042: Warning: Keylock active
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    3.154306] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    3.155428] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    3.156852] mousedev: PS/2 mouse device common for all mice
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    3.158465] rtc_cmos 00:00: RTC can wake from S4
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    3.159812] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    3.161536] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    3.163098] i2c /dev entries driver
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    3.164005] device-mapper: uevent: version 1.0.3
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    3.165284] device-mapper: ioctl: 4.34.0-ioctl (2015-10-213-3bc4-4a1f-939e-80fda52d12c2 kernel: [    7.571010] raid6: sse2x2   gen() 11471 MB/s
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    7.639004] raid6: sse2x2   xor()  7817 MB/s
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    7.707003] raid6: sse2x4   gen() 12904 MB/s
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    7.775004] raid6: sse2x4   xor()  8839 MB/s
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    7.775804] raid6: using algorithm sse2x4 gen() 12904 MB/s
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    7.776600] raid6: .... xor() 8839 MB/s, rmw enabled
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    7.777278] raid6: using ssse3x2 recovery algorithm
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    7.779379] xor: automatically using best checksumming function:
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    7.819007]    avx       : 22180.000 MB/sec
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    7.832466] Btrfs loaded
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    7.869144] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    7.870157] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 15:31:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [    7.938691] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 15:31:00fda52d12c2 ntpd[1903]: new interface(s) found: waking up resolver
Aug  7 15:35:01 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [  249.897231] docker0: port 1(veth4d1a7cd) entered forwarding state
Aug  7 16:15:06 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [ 2655.043276] vethb3dca24: renamed from eth0
Aug  7 16:15:06 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [ 2655.069279] docker0: port 1(veth4d1a7cd) entered disabled state
Aug  7 16:15:06 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [ 2655.117002] docker0: port 1(veth4d1a7cd) entered disabled state
Aug  7 16:15:06 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [ 2655.118799] device veth4d1a7cd left promiscuous mode
Aug  7 16:15:06 travis-job-94523013-3bc4-4a1f-939e-80fda52d12c2 kernel: [ 2655.118802] docker0: port 1(veth4d1a7cd) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:1de09ff3
