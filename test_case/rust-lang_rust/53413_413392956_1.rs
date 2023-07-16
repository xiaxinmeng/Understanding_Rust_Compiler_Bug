\n\n"},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/editions/edition-feature-ok.rs","byte_start":527,"byte_end":544,"line_start":14,"line_end":14,"column_start":12,"column_end":29,"is_primary":true,"text":[{"text":"#![feature(rust_2018_preview)]","highlight_start":12,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition\n  --> /checkout/src/test/ui/editions/edition-feature-ok.rs:14:12\n   |\nLL | #![feature(rust_2018_preview)]\n   |            ^^^^^^^^^^^^^^^^^\n\n"}
[00:49:11] 
[00:49:11] ------------------------t":"extern crate edition_lint_paths;","highlight_start":1,"highlight_end":33}],"label":null,"suggested_replacement":"","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused extern crate\n  --> /checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs:22:1\n   |\nLL | extern crate edition_lint_paths;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove it\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs:19:9\n   |\nLL | #![deny(rust_2018_idioms)]\n   |         ^^^^^^^^^^^^^^^^\n   = note: #[deny(unused_extern_crates)] implied by #[deny(rust_2018_idioms)]\n\n"}
[00:49:11] {"message":"`extern crate` is not idiomatic in the new edition","code":{"code":"unused_extern_crates","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs","byte_start":784,"byte_end":823,"line_start":25,"line_end":25,"column_start":1,"column_end":40,"is_primary":true,"text":[{"text":"extern crate edition_lint_paths as bar;","highlight_start":1,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"convert it to a `use`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs","byte_start":784,"byte_end":823,"line_start":25,"line_end":25,"column_start":1,"column_end":40,"is_primary":true,"text":[{"text":"extern crate edition_lint_paths as bar;","highlight_start":1,"highlight_end":40}],"label":null,"suggested_replacemeng 2018 01:28:45 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_fold:start:after_failure.1
travis_time:start:0f95e818
$ sudo tail -n 500 /var/log/syslog
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffAug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf2728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] Hierarchical RCU implementation.
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] console [ttyS0] enabled
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.609276] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.613143] pid_max: default: 32768 minimum: 301
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.615186] ACPI: Core revision 20150930
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.623269] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.627077] Security Framework initialized
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.628720] Yama: becoming mindful.
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.630249] AppArmor: AppArmor disabled by boot time parameter
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.634301] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.647669] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.655358] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.658232] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.661285] Initializing cgroup subsys io
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.663930] Initializing cgroup subsys memory
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.665816] Initializing cgroup subsys devices
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.668049] Initializing cgroup subsys freezer
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.670179] Initializing cgroup subsys net_cls
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.672663] Initializing cgroup subsys perf_event
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.674477] Initializing cgroup subsys net_prio
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.677377] Initializing cgroup subsys hugetlb
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.680077] Initializing cgroup subsys pids
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.681883] CPU: Physical Processor ID: 0
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.684199] CPU: Processor Core ID: 0
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.687360] mce: CPU supports 32 MCE banks
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.690074] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.693041] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.698592] Freeing SMP alternatives memory: 32K
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.711212] ftrace: allocating 32185 entries in 126 pages
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.775112] smpboot: APIC(0) Converting physical 0 to logical pck: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.938800] x86: Booted up 1 node, 4 CPUs
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.940433] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.945225] devtmpfs: initialized
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.951847] evm: security.selinux
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.953543] evm: security.SMACK64
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.955248] evm: security.SMACK64EXEC
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.957452] evm: security.SMACK64TRANSMUTE
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.959511] evm: security.SMACK64MMAP
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.961111] evm: security.ima
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.963338] evm: security.capability
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.966340] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.970382] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.973901] pinctrl core: initialized pinctrl subsystem
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.975866] RTC time:  0:38:32, date: 08/16/18
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.978472] NET: Registered protocol family 16
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    0.990394] cpuidle: using governor ladder
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.003212] cpuidle: using governor menu
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.005450] PCCT header not found.
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.007208] ACPI: bus type PCI registered
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.008673] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.013552] PCI: Using configuration type 1 for base access
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.029237] ACPI: Added _OSI(Module Device)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.032564] ACPI: Added _OSI(Processor Device)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.034883] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.036931] ACPI: Added _OSI(Processor Aggregator Device)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.041856] ACPI: Executed 2 blocks of module-level executable AML code
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.069175] ACPI: Interpreter enabled
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.071271] ACPI: (supports S0 S3 S4 S5)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.073030] ACPI: Using IOAPIC for interrupt routing
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.075657] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.110534] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.114356] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.117765] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.120640] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.128001] PCI host bridge to bus 0000:00
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.130650] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.134023] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.137442] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.143142] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.147412] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.150905] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.151391] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.179810] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.209206] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.212716] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.225066] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.235032] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.265784] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.278162] pci 0000:00:04.0: [    1.453757] PCI: CLS 0 bytes, default 64
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    1.454646] Unpacking initramfs...
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.715665] Freeing initrd memory: 21432K
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.716944] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.718330] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.720796] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.722997] hw unit of domain pp0-core 2^-0 Joules
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.724300] hw unit of domain package 2^-0 Joules
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.724996] hw unit of domain dram 2^-0 Joules
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.725873] Scanning for low memory corruption every 60 seconds
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.727330] audit: initializing netlink subsys (disabled)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.728130] audit: type=2000 audit(1534379914.675:1): initialized
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.7i: leaving for legacy driver
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.766287] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.767701] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.773332] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.796201] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.820033] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.843904] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.867542] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.871678] Linux agpgart interface v0.103
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.875808] loop: module loaded
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.877021] libphy: Fixed MDIO Bus: probed
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.878232] tun: Universal TUN/TAP device driver, 1.6
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.879884] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.944097] PPP generic driver version 2.4.2
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.945941] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.947836] ehci-pci: EHCI PCI platform driver
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.949350] ehci-platform: EHCI generic platform driver
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.951574] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.953656] ohci-pci: OHCI PCI platform driver
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.955481] ohci-platform: OHCI generic platform driver
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.957093] uhci_hcd: USB Universal Host Controller Interface driver
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.959470] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.963147] i8042: Warning: Keylock active
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.965258] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.966648] serio: i8042 AUX port at 0x60,0x64d6a-b56d8bf20f9b kernel: [    3.994565] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.996342] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.998079] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    3.999858] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    4.003137] registered taskstats version 1
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    4.004725] Loading compiled-in X.509 certificates
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    4.006889] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    4.010448] zswap: loaded using pool lzo/zbud
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    4.014248] Key type trusted registered
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    4.019635] Key type encrypted registered
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    4.020840] ima: No TPM chip found, activating TPM-bypass!
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    4.022481] evm: HMAC attrs: 0x1
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    4.02-ad6a-b56d8bf20f9b kernel: [    8.683425]    avx       : 21558.000 MB/sec
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    8.699482] Btrfs loaded
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    8.757609] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    8.761364] EXT4-fs (sda1): write access will be enabled during recovery
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    8.861650] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    8.880123] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    8.882885] EXT4-fs (sda1): recovery complete
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    8.892109] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    9.148901] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    9.308492] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    9.371557] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 16 00:38:43 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [    9.625624] random: cloud-init: uninitialized urandom read (32 bytes read, ravis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Starting Google Accounts daemon.
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Creating a new user account for me.
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Created user account me.
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Creating a new user account for henrik.
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Created user account henrik.
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Creating a new user account for emma.
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Created user account emma.
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Creating a new user account for igor.
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Created user account igor.
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [   13.392761] random: nonblocking pool is initialized
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Created user account konstantinhaase.
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Creating a new user account for aj.
Aug 16 00:38:44 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Created user account aj.
Aug 16 00:38:45 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Creating a new user account for solarce.
Aug 16 00:38:45 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-clock-skew: INFO Synced system time with hardware clock.
Aug 16 00:38:45 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Created user account solarce.
Aug 16 00:38:45 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Creating a new user account for asari.
Aug 16 00:38:45 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Created user account asari.
Aug 16 00:38:45 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Creating a new user account for bogdana.
Aug 16 00:38:45 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [   13.680896] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need thi83b-ede3-4c1f-ad6a-b56d8bf20f9b pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 00:38:45 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 00:38:45 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Creating a new user account for maria.
Aug 16 00:38:45 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Created user account maria.
Aug 16 00:38:45 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b google-accounts: INFO Removing user packer.
Aug 16 00:38:47 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b cron[1708]: (CRON) INFO (pidfile fd = 3)
Aug 16 00:38:47 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b cron[1740]: (CRON) STARTUP (fork ok)
Aug 16 00:38:47 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b cron[1740]: (CRON) INFO (Running @reboot jobs)
Aug 16 00:38:47 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 00:38:47 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 00:38:47 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b acpid: starting up with netlink and the input layer
Aug 16 00:38:47 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b acpid: 1 rule loaded
Aug 16 00:38:47 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b acpid: waiting for events: event logging is off
Aug 16 00:38:47 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b haveged: haveged starting up
Aug 16 00:38:47 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [   16.013568] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 16 00:38:52 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b ntpd[1844]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 16 00:38:52 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b ntpd[1845]: proto: precision = 0.111 usec
Aug 16 00:38:52 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b ntpd[1845]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 16 00:38:52 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b ntpd[1845]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 16 00:38:52 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b ntpd[1845]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 16 00:38:52 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b ntpd[1845]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 16 00:38:52 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b ntpd[1845]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 16 00:38:52 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b ntpd[1845]: Listen normally on 3 eth0 10.20.1.236 UDP 123
Aug 16 00:38:52 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b ntpd[1845]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 16 00:38:52 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b ntpd[1845]: peers refreshed
Aug 16 00:38:52 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b ntpd[1845]: Listening on routing socket on fd #21 for interface updates
Aug 16 00:38:52 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [   21.229072] init: plymouth-upstart-bridge main process ended, respawning
Aug 16 00:38:52 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8:f3:d8:9c:70:13:60:b4:33:e2:c3:8a:ed  root@travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b (RSA)
Aug 16 00:38:52 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 16 00:38:52 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b ec2: #############################################################
Aug 16 00:39:00 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b ntpdate[2709]: the NTP socket is in use, exiting
Aug 16 00:39:32 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [   60.933002] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 16 00:41:55 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [  203.942596] device veth7c7a3c7 entered promiscuous mode
Aug 16 00:41:55 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [  203.942664] docker0: port 1(veth7c7a3c7) entered forwarding state
Aug 16 00:41:55 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [  203.942674] docker0: port 1(veth7c7a3c7) entered forwarding state
Aug 16 00:41:55 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [  203.942881] docker0: port 1(veth7c7a3c7) entered disabled state
Aug 16 00:41:55 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [  204.044486] cgroup: docker-runc (4898) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 16 00:41:55 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [  204.044489] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 16 00:41:55 travis-job-d64af83b-ede3-4c1f-ad6a-b56d8bf20f9b kernel: [  204.128228] eth0: rena
