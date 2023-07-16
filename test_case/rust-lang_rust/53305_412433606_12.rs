compile_fail,E0502\nfn bar(x: &mut i32) {}\nfn foo(a: &mut i32) {\n    let ref y = a; // a is borrowed as immutable.\n    bar(a); // error: cannot borrow `*a` as mutable because `a` is also borrowed\n            //        as immutable\"--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:42] 
[00:58:42] 
[00:58:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:42] Build completed unsuccessfully in 0:02:15
[00:58:42] Build completed unsuccessfully in 0:02:15
[00:58:42] make: *** [check] Error 1
[00:58:42] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:19e3cc54
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:001c7867
$ sudo tail -n 500 /var/log/syslog
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000]   3 disabled
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000]   4 disabled
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000]   5 disabled
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000]   6 disabled
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000]   7 disabled
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff88 0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000] Policy zone: Normal
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.000000] Kernel command line: BOOT_IMAnt-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.588710] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.592255] Initializing cgroup subsys io
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.594577] Initializing cgroup subsys memory
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.596674] Initializing cgroup subsys devices
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.599006] Initializing cgroup subsys freezer
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.600781] Initializing cgroup subsys net_cls
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.603183] Initializing cgroup subsys perf_event
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.605195] Initializing cgroup subsys net_prio
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.607584] Initializing cgroup subsys hugetlb
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.609870] Initializing cgroup subsys pids
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.612067] CPU: Physical Processor ID: 0
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.614191] CPU: Processor Core ID: 0
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.617940] mce: CPU supports 32 MCE banks
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.620922] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.623467] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.629087] Freeing SMP alternatives memory: 32K
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.641045] ftrace: allocating 32185 entries in 126 pages
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.697305] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.701071] smpboot: Max logical packages: 2
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.703641] x2apic enabled
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.705814] Switched APIC routing to physical x2apic.
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.711695] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.822294] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.826552] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.831224] x86: Booting SMP configuration:
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.833020] .... node  #0, CPUs:      #1
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.834962] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.841311]  #2
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.842206] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.848336]  #3
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.849534] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.855120] x86: Booted up 1 node, 4 CPUs
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.856807] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.861667] devtmpfs: initialized
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.866642] evm: security.selinux
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.867966] evm: security.SMACK64
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.869267] evm: security.SMACK64EXEC
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.870591] evm: security.SMACK64TRANSMUTE
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.872439] evm: security.SMACK64MMAP
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.874038] evm: security.ima
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.875134] evm: security.capability
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.877604] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.882459] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.886019] pinctrl core: initialized pinctrl subsystem
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.888938] RTC time:  6:35:18, date: 08/13/18
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.892445] NET: Registered protocol family 16
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.906374] cpuidle: using governor ladder
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.918377] cpuidle: using governor menu
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.920493] PCCT header not found.
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.922550] ACPI: bus type PCI registered
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    0.924915] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [ -427a-8b43-f6618b190a70 kernel: [    3.372242] hw unit of domain package 2^-0 Joules
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.373035] hw unit of domain dram 2^-16 Joules
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.374107] Scanning for low memory corruption every 60 seconds
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.375893] audit: initializing netlink subsys (disabled)
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.377009] audit: type=2000 audit(1534142120.724:1): initialized
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.378568] Initialise system trusted keyring
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.379966] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.381389] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.383801] zbud: loaded
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.384720] VFS: Disk quotas dquot_6.6.0
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.385672] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.387810] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.389162] fuse init (API versi618b190a70 kernel: [    3.403605] ACPI: Power Button [PWRF]
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.404683] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.406105] ACPI: Sleep Button [SLPF]
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.407326] GHES: HEST is not enabled!
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.410019] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.411262] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.416752] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.417806] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.424198] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.446554] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.470126] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.492869] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 13 06:35:29CI generic platform driver
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.587894] uhci_hcd: USB Universal Host Controller Interface driver
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.589906] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.592929] i8042: Warning: Keylock active
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.595570] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.597545] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.599728] mousedev: PS/2 mouse device common for all mice
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.602551] rtc_cmos 00:00: RTC can wake from S4
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.604739] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.606908] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.608576] i2c /dev entries driver
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.609746] device-mapper: uevent: version 1.0.3
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.611968] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel713867] Freeing unused kernel memory: 1956K
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.717017] Freeing unused kernel memory: 92K
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.737405] systemd-udevd[118]: starting version 204
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.800837] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.807011] scsi host0: Virtio SCSI HBA
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.811677] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.821163] AVX2 version of gcm_enc/dec engaged.
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.823363] AES CTR mode by8 optimization enabled
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.874172] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.874315] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.874318] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.874961] sd 0:0:1:0: [sda] Write Protect is off
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.874963] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.875043] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.878563]  sda: sda1
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    3.881083] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    4.372707] tsc: Refined TSC clocksource calibration: 2300.000 MHz
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    4.375720] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735223b2, max_idle_ns: 440795277976 ns
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    4.658708] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    6.804960] floppy0: no floppy controllers found
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    7.968541] raid6: sse2x1   gen()  8791 MB/s
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.036605] raid6: sse2x1   xor()  6547 MB/s
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.104536] raid6: sse2x2   gen() 10433 MB/s
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.172557] raid6: sse2x2   xor()  7144 MB/s
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.240606] raid6: sse2x4   gen() 12278 MB/s
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.308559] raid6: sse2x4   xor()  8729 MB/s
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.376603] raid6: avx2x1   gen() 17020 MB/s
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.444585] raid6: avx2x2   gen() 19486 MB/s
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.512590] raid6: avx2x4   gen() 22352 MB/s
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.517502] raid6: using algorithm avx2x4 gen() 22352 MB/s
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.520208] raid6: using avx2x2 recovery algorithm
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.525883] xor: automatically using best checksumming function:
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.568546]    avx       : 26827.000 MB/sec
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.585182] Btrfs loaded
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.650739] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.654065] EXT4-fs (sda1): write access will be enabled during recovery
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.738161] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 13 06:35:29 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [    8.748316] EXT4-fs (sda1): inished running startup scripts.
Aug 13 06:36:00 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 ec2: 
Aug 13 06:36:00 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 ec2: #############################################################
Aug 13 06:36:00 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 13 06:36:00 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 ec2: 1024 25:05:0c:88:be:d8:21:85:01:84:39:dc:d6:90:14:72  root@travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 (DSA)
Aug 13 06:36:00 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 ec2: 256 06:ab:b5:f1:55:08:1e:2b:b9:a8:09:d3:a8:b0:11:e6  root@travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 (ECDSA)
Aug 13 06:36:00 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 ec2: 256 fb:28:44:b4:a6:bd:b4:3b:91:9b:9d:5a:c0:42:06:3e  root@travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 (ED25519)
Aug 13 06:36:00 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 ec2: 2048 93:06:e6:74:ec:6c:a6:b6:d3:a2:7a:7d:c9:62:61:32  root@travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 (RSA)
Aug 13 06:36:00 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 13 06:36:00 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 ec2: #############################################################
Aug 13 06:40:14 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [  297.038794] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 13 06:46:52 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [  694.696308] device veth30c7548 entered promiscuous mode
Aug 13 06:46:52 travis-job-0b53bb77-99da-427a-8b43-fg 13 06:47:07 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [  709.954533] docker0: port 1(veth30c7548) entered forwarding state
Aug 13 07:17:01 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 CRON[16315]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 13 07:38:57 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [ 3819.824098] veth2283d9b: renamed from eth0
Aug 13 07:38:57 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [ 3819.865847] docker0: port 1(veth30c7548) entered disabled state
Aug 13 07:38:57 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [ 3819.898817] docker0: port 1(veth30c7548) entered disabled state
Aug 13 07:38:57 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [ 3819.900447] device veth30c7548 left promiscuous mode
Aug 13 07:38:57 travis-job-0b53bb77-99da-427a-8b43-f6618b190a70 kernel: [ 3819.900450] docker0: port 1(veth30c7548) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:08e1f5b8
