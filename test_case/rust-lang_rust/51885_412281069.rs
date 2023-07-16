plain
[00:45:49]     Checking rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:45:49]     Checking rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:45:49]     Checking rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:45:49]  Documenting std v0.0.0 (file:///checkout/src/libstd)
[00:45:54] warning: `[Weak]` cannot be resolved, ignoring it...
[00:45:54]    --> /checkout/src/liballoc/rc.rs:809:10
[00:45:54]     |
[00:45:54] 809 |     /// [`Weak`], so we `drop` the inner value.
[00:45:54]     |
[00:45:54]     = note: #[warn(intra_doc_link_resolution_failure)] on by default
[00:45:54]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:45:54] 
[00:45:54] 
[00:45:54] warning: `[Weak]` cannot be resolved, ignoring it...
[00:45:54]     |
[00:45:54]     |
[00:45:54] 918 |     /// [`Weak`], so we `drop` the inner value.
[00:45:54]     |
[00:45:54]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:45:54] 
[00:45:54] 
[00:45:54] warning: `[Weak::upgrade]` cannot be resolved, ignoring it...
[00:45:54]     --> /checkout/src/liballoc/rc.rs:1321:33
[00:45:54]      |
[00:45:54] 1321 |     /// it. Calling [`upgrade`][Weak::upgrade] on the return value always gives [`None`].
[00:45:54]      |
[00:45:54]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:45:54] 
[00:45:54] 
[00:45:54] warning: `[Weak::upgrade]` cannot be resolved, ignoring it...
[00:45:54]      |
[00:45:54]      |
[00:45:54] 1159 |     /// Calling [`upgrade`][Weak::upgrade] on the return value always
[00:45:54]      |
[00:45:54]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:45:54] 
[00:45:54] 
[00:45:54] warning: `[Weak::upgrade]` cannot be resolved, ignoring it...
[00:45:54]     --> /checkout/src/liballoc/rc.rs:1174:29
[00:45:54]      |
[00:45:54] 1174 |     /// Calling [`upgrade`][Weak::upgrade] on the return value always gives [`None`].
[00:45:54]      |
[00:45:54]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:45:54] 
[00:45:54] 
[00:45:54] warning: `[Seek::seek_relative]` cannot be resolved, ignoring it...
[00:45:54]     |
[00:45:54]     |
[00:45:54] 297 |     /// To seek without discarding the internal buffer, use [`Seek::seek_relative`].
[00:45:54]     |
[00:45:54]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:45:54] 
[00:45:54] 
[00:45:54] warning: `[std::io::Seek]` cannot be resolved, ignoring it...
[00:45:54]     |
[00:45:54]     |
[00:45:54] 299 |     /// See [`std::io::Seek`] for more details.
[00:45:54]     |
[00:45:54]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:45:54] 
[00:45:58]     Finished release [optimized] target(s) in 48.15s
---
[00:49:24] ....................................................................................................
[00:49:26] ....................................................................................................
[00:49:28] ....................................................................................................
[00:49:32] ....................................................................................................
[00:49:34] ..............iiiiiiiii.............................................................................
[00:49:40] ....................................................................................................
[00:49:44] ....................i...............................................................................
[00:49:47] ...............................i....................................................................
[00:49:49] ....................................................................................................
---
[01:18:16] travis_fold:end:stage0-linkchecker

[01:18:16] travis_time:end:stage0-linkchecker:start=1533999921388847401,finish=1533999923877029947,duration=2488182546

[01:20:05] std/convert/trait.From.html:206: broken link - std/convert/struct.OsString.html
[01:21:03] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:21:03] 
[01:21:03] 
[01:21:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:21:03] expected success, got: exit code: 101
[01:21:03] expected success, got: exit code: 101
[01:21:03] 
[01:21:03] 
[01:21:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:03] Build completed unsuccessfully in 0:34:25
[01:21:03] make: *** [check] Error 1
[01:21:03] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f63e6f8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:01a86088
$ sudo tail -n 500 /var/log/syslog
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] Scanning 1 areas for low memor000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71e1ed kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] Policy zone: Normal
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0-458b-a092-71ec5408a1ed kernel: [    0.000000] console [ttyS0] enabled
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.578705] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.582835] pid_max: default: 32768 minimum: 301
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.584789] ACPI: Core revision 20150930
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.592664] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.596447] Security Framework initialized
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.598263] Yama: becoming mindful.
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.599948] AppArmor: AppArmor disabled by boot time parameter
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.605585] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.617640] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.624333] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kern4KB 512, 2MB 8, 4MB 8
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.665430] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.670643] Freeing SMP alternatives memory: 32K
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.682031] ftrace: allocating 32185 entries in 126 pages
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.738218] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.741662] smpboot: Max logical packages: 2
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.744473] x2apic enabled
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.747145] Switched APIC routing to physical x2apic.
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.753406] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.864143] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.870380] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.877195] x86: Booting SMP configuration:
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.879753] .... node  #0, CPUs:      #1
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.881897] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.888339]  #2
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.889334] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.895296]  #3
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.896483] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.902067] x86: Booted up 1 node, 4 CPUs
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.904649] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.910340] devtmpfs: initialized
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.915938] evm: security.selinux
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.917827] evm: security.SMACK64
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.919337] evm: security.SMACK64EXEC
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.921202] evm: security.SMACK64TRANSMUTE
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.923876] evm: security.SMACK64MMAP
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    0.926022] evm: secur:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    1.352827] SCSI subsystem initialized
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    1.355683] libata version 3.00 loaded.
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    1.355710] ACPI: bus type USB registered
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    1.358238] usbcore: registered new interface driver usbfs
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    1.362925] usbcore: registered new interface driver hub
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    1.366536] usbcore: registered new device driver usb
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    1.370082] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    1.374224] dmi: Firmware registration failed.
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    1.377939] PCI: Using ACPI for IRQ routing
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    1.380700] PCI: pci_cache_line_size set to 64 bytes
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    1.380800] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    1.380803] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    1.381006] NetLabel: Initializingtravis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.570223] Allocating IMA MOK and blacklist keyrings.
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.578084] Key type asymmetric registered
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.580488] Asymmetric key parser 'x509' registered
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.583449] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.587806] io scheduler noop registered
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.590226] io scheduler deadline registered (default)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.593518] io scheduler cfq registered
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.596165] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.598623] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.602657] intel_idle: does not run on family 6 model 45
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.602754] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.606416] ACPI: Power Button [PWRF]
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.60-1506-458b-a092-71ec5408a1ed kernel: [    3.866127] Freeing unused kernel memory: 1956K
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.867174] Freeing unused kernel memory: 92K
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.881102] systemd-udevd[119]: starting version 204
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.937431] scsi host0: Virtio SCSI HBA
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.940649] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.956326] AVX version of gcm_enc/dec engaged.
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.957882] AES CTR mode by8 optimization enabled
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.987965] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.987977] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.987978] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.988140] sd 0:0:1:0: [sda] Write Protect is off
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.988141] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    3.988253] sd 0:0:1:0: [s: [    8.383372] raid6: sse2x4   gen() 12531 MB/s
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    8.451385] raid6: sse2x4   xor()  8581 MB/s
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    8.453352] raid6: using algorithm sse2x4 gen() 12531 MB/s
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    8.455891] raid6: .... xor() 8581 MB/s, rmw enabled
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    8.458116] raid6: using ssse3x2 recovery algorithm
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    8.461912] xor: automatically using best checksumming function:
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    8.503376]    avx       : 26660.000 MB/sec
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    8.520103] Btrfs loaded
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    8.583344] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    8.587664] EXT4-fs (sda1): write access will be enabled during recovery
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    8.671534] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    8.681663] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [    8.684499] EXT4-fs (sda1): recovery complete
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-45 user-space parallel port driver
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [   10.470750] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [   10.526417] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [   10.589763] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [   10.760226] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [   11.040127] random: mktemp: uninitialized urandom read (12 bytes read, 56 bits of entropy available)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [   11.119279] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [   11.201138] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [   11.261473] EXT4-fs (sda1): resized filesystem to 7864064
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [   11.587031] init: failsafe main process (1099) killed by TERM signal
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed instance-setup: INFO Running set_multiqueue.
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed instance-setup: INFO Set channels for eth0 to 4.
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 11 13:46:11 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to  13.084807] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 11 13:46:13 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [   13.153477] Initializing XFRM netlink socket
Aug 11 13:46:13 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [   13.160755] Netfilter messages via NETLINK v0.30.
Aug 11 13:46:13 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [   13.163651] ctnetlink v0.93: registering with nfnetlink.
Aug 11 13:46:13 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [   13.263261] random: nonblocking pool is initialized
Aug 11 13:46:13 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [   13.319613] floppy0: no floppy controllers found
Aug 11 13:46:13 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 11 13:46:13 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed pollinate: To re-seed this system again, use the -r|--reseed option
Aug 11 13:46:15 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed cron[1620]: (CRON) INFO (pidfile fd = 3)
Aug 11 13:46:15 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed cron[1652]: (CRON) STARTUP (fork ok)
Aug 11 13:46:15 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 11 13:46:15 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed pollinate: To re-seed this system again, use the -r|--reseed option
Aug 11 13:46:15 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed cron[1652]: (CRON) INFO (Running @reboot jobs)
Aug 11 13:46:15 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed acpid: starting up with netlink and the input layer
Aug 11 13:46:15 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed acpid: 1 rule loaded
Aug 11 13:46:15 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed acpid: waiting for events: event logging is off
Aug 11 13:46:15 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed haveged: haveged starting up
Aug 11 13:46:15 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed kernel: [   15.348066] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 11 13:46:20 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed ntpd[1754]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 11 13:46:20 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed ntpd[1755]: proto: precision = 0.106 usec
Aug 11 13:46:20 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed ntpd[1755]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 11 13:46:20 travis-job-d1d7e6a0-1506-458b-a092-71ec5408a1ed ntpd[1755]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
