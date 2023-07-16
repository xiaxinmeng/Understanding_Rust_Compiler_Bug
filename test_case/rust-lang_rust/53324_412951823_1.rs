\n\n[PhantomData] can also be used to express information about unused type\nparameters.\n\n[PhantomData]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-36638.rs","byte_start":527,"byte_end":531,"line_start":13,"line_end":13,"column_start":12,"column_end":16,"is_primary":true,"text":[{"text":"struct Foo<Self>(Self);","highlight_start":12,"highlight_end":16}],"label":"unused type parameter","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing `Self` or using a marker such as `std::marker::PhantomData`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0392]: parameter `Self` is never used\n  --> /checkout/src/test/ui/issues/issue-36638.rs:13:12\n   |\nLL | struct Foo<Self>(Self);\n   |            ^^^^ unused type parameter\n   |\n   = help: consider removing `Self` or using a marker such as `std::marker::PhantomData`\n\n"}
[00:49:12] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:49:12] {"message":"For more information about this error, try `rustc --explain E0392`.","code":null,"levncachable
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000]   1 disabled
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000]   2 disabled
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000]   3 disabled
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000]   4 disabled
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000]   5 disabled
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000]   6 disabled
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000]   7 disabled
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] Using GB pages for direct mapping
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 14 16:36:12 travis000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] kvm-clock: using sched offset of 1750611121 cycles
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] Zone ranges:
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000]   Device   empty
Aug 14 16:36:12 travis-job-0407eor memmap
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 14 16:36:12 travis-job-040Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] Policy zone: Normal
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-1546bc8-faea-4278-b461-15462963c78e kernel: [    0.564063] Initializing cgroup subsys devices
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.566530] Initializing cgroup subsys freezer
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.568990] Initializing cgroup subsys net_cls
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.570545] Initializing cgroup subsys perf_event
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.572551] Initializing cgroup subsys net_prio
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.574055] Initializing cgroup subsys hugetlb
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.575587] Initializing cgroup subsys pids
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.576994] CPU: Physical Processor ID: 0
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.578402] CPU: Processor Core ID: 0
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.580919] mce: CPU supports 32 MCE banks
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.582569] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.584447] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.589313] Freeing SMP alternatives memory: 32K
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel963c78e kernel: [    0.844183] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.847067] pinctrl core: initialized pinctrl subsystem
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.849552] RTC time: 16:36:01, date: 08/14/18
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.852299] NET: Registered protocol family 16
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.865551] cpuidle: using governor ladder
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.877560] cpuidle: using governor menu
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.879449] PCCT header not found.
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.881003] ACPI: bus type PCI registered
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.882188] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.885153] PCI: Using configuration type 1 for base access
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.899395] ACPI: Added _OSI(Module Device)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.901938] ACPI: Added _OSI(Processor Device)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    0.903712] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: 8-b461-15462963c78e kernel: [    1.107271] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.116382] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.123417] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.144876] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.148956] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.152960] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.156712] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.161452] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.183914] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.186775] vgaarb: loaded
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.187838] SCSI subsystem initialized
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.189628] libata version 3.00 loaded.
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.189655] ACPI: bus type USB registered
Aug 14 16:36:12 travism: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.231884] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.231887] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.231888] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.231890] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.231930] NET: Registered protocol family 2
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.232915] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.235431] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.236594] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.238106] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.239191] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    1.240502] NET: Registered protocol family 1
Aug 14 16:is-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.410788] audit: type=2000 audit(1534264563.763:1): initialized
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.412148] Initialise system trusted keyring
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.413488] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.415402] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.417491] zbud: loaded
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.418148] VFS: Disk quotas dquot_6.6.0
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.419363] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.421505] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.423447] fuse init (API version 7.23)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.424551] Key type big_key registered
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.425435] Allocating IMA MOK and blacklist keyrings.
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.428116] Key type asymmetric registered
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.429179] Asymmetric key parser 'x509' registered
63c78e kernel: [    3.444891] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.446003] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.451003] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.452059] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.458618] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.482388] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.506081] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.530022] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.553989] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.558205] Linux agpgart interface v0.103
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.562193] loop: module loaded
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.563262] libphy: Fixed MDIO Bus: probed
Aug 14 16:36:12 t7] AES CTR mode by8 optimization enabled
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.771021] scsi host0: Virtio SCSI HBA
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.776699] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.809365] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.809387] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.809389] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.813114] sd 0:0:1:0: [sda] Write Protect is off
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.813884] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.814141] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.817119]  sda: sda1
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.818678] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    3.826833] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    4very algorithm
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    8.323357] xor: automatically using best checksumming function:
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    8.362494]    avx       : 21332.000 MB/sec
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    8.378908] Btrfs loaded
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    8.435324] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    8.437598] EXT4-fs (sda1): write access will be enabled during recovery
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    8.530752] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    8.546300] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    8.548758] EXT4-fs (sda1): recovery complete
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    8.556734] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    8.803482] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    8.940879] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    8.999297] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    9.222277] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [    9.858922] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [   10.018872] systemd-udevd[702]: starting version 204
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [   10.152719] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [   10.269139] ppdev: user-space parallel port driver
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [   10.390464] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [   10.457907] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [   10.536769] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [   10.717842] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [   11.019finity_list: real affinity 0
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15462963c78e instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 14 16:36:12 travis-job-0407ebc8-faea-4278-b461-15-b461-15462963c78e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 16:36:14 travis-job-0407ebc8-faea-4278-b461-15462963c78e pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 16:36:14 travis-job-0407ebc8-faea-4278-b461-15462963c78e cron[1594]: (CRON) INFO (pidfile fd = 3)
Aug 14 16:36:14 travis-job-0407ebc8-faea-4278-b461-15462963c78e cron[1635]: (CRON) STARTUP (fork ok)
Aug 14 16:36:14 travis-job-0407ebc8-faea-4278-b461-15462963c78e cron[1635]: (CRON) INFO (Running @reboot jobs)
Aug 14 16:36:14 travis-job-0407ebc8-faea-4278-b461-15462963c78e acpid: starting up with netlink and the input layer
Aug 14 16:36:14 travis-job-0407ebc8-faea-4278-b461-15462963c78e acpid: 1 rule loaded
Aug 14 16:36:14 travis-job-0407ebc8-faea-4278-b461-15462963c78e acpid: waiting for events: event logging is off
Aug 14 16:36:14 travis-job-0407ebc8-faea-4278-b461-15462963c78e haveged: haveged starting up
Aug 14 16:36:14 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [   13.783172] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 16:36:19 travis-job-0407ebc8-faea-4278-b461-15462963c78e ntpd[1732]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 14 16:36:19 travis-job-0407ebc8-faea-4278-b461-15462963c78e ntpd[1733]: proto: precision = 0.101 usec
Aug 14 16:36:19 travis-job-0407ebc8-faea-4278-b461-15462963c78e ntpd[1733]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 16:36:19 travis-job-0407ebc8-faea-4278-b461-15462963c78e ntpd[1733]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 16:36:19 travis-job-0407ebc8-b461-15462963c78e kernel: [  206.444769] device vethc360137 entered promiscuous mode
Aug 14 16:39:27 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [  206.444858] docker0: port 1(vethc360137) entered forwarding state
Aug 14 16:39:27 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [  206.444869] docker0: port 1(vethc360137) entered forwarding state
Aug 14 16:39:27 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [  206.445843] docker0: port 1(vethc360137) entered disabled state
Aug 14 16:39:27 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [  206.553556] cgroup: docker-runc (4739) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 16:39:27 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [  206.553559] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 16:39:27 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [  206.636056] eth0: renamed from veth8aaf1b2
Aug 14 16:39:27 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [  206.683174] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 16:39:27 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [  206.684691] docker0: port 1(vethc360137) entered forwarding state
Aug 14 16:39:27 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [  206.684708] docker0: port 1(vethc360137) entered forwarding state
Aug 14 16:39:27 travis-job-0407ebc8-faea-4278-b461-15462963c78e kernel: [  206.684729] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 14 16:39:30 travis-job-0407ebc8-faea-4278-b461fold:start:after_failure.2
