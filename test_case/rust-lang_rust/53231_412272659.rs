plain
[00:53:59] ....................................................................................................
[00:54:02] ....................................................................................................
[00:54:04] ....................................................................................................
[00:54:08] ....................................................................................................
[00:54:11] ..............iiiiiiiii.............................................................................
[00:54:17] ....................................................................................................
[00:54:20] ....................i...............................................................................
[00:54:23] ...............................i....................................................................
[00:54:27] ....................................................................................................
---
[01:16:23] iii.................................................................................................
[01:16:37] ................................................................................................iii.
[01:16:45] .....i......i...i......i............................................................................
[01:16:50] ....................................................................................................
[01:17:03] ......F.............iiii........ii..................................................................
[01:17:19] ....................................................................iiii............................
[01:17:36] ....................................................................................................
[01:17:42] ..................................................................................iiii..............
[01:17:45] .................................
[01:17:45] .................................
[01:17:45] failures:
[01:17:45] 
[01:17:45] ---- keyword_docs.rs - let_keyword (line 48) stdout ----
[01:17:45] error[E0658]: attributes on expressions are experimental. (see issue #15701)
[01:17:45]  --> keyword_docs.rs:52:1
[01:17:45] 7 | #[allow(unused_assignments)]
[01:17:45]   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:17:45]   |
[01:17:45]   = help: add #![feature(stmt_expr_attributes)] to the crate attributes to enable
[01:17:45]   = help: add #![feature(stmt_expr_attributes)] to the crate attributes to enable
[01:17:45] 
[01:17:45] thread 'keyword_docs.rs - let_keyword (line 48)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:17:45] 
[01:17:45] 
[01:17:45] failures:
[01:17:45]     keyword_docs.rs - let_keyword (line 48)
[01:17:45]     keyword_docs.rs - let_keyword (line 48)
[01:17:45] 
[01:17:45] test result: FAILED. 908 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
[01:17:45] 
[01:17:45] error: test failed, to rerun pass '--doc'
[01:17:45] 
[01:17:45] 
[01:17:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:17:45] 
[01:17:45] 
[01:17:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:45] Build completed unsuccessfully in 0:26:40
[01:17:45] Build completed unsuccessfully in 0:26:40
[01:17:45] make: *** [check] Error 1
[01:17:45] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:090e8a78
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0938dedc
$ sudo tail -n 500 /var/log/syslog
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] Using GB pages for direct mapping
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] kvm-clock: using sched offset of 1427344589 cycles
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] Zone ranges:
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000]   Device   empty
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] Movable zone start for each node
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] Early memory node ranges
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000]   node   0: [mem 0x0000000100000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] PM: Registeredpcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] Policy zone: Normal
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 11 11ecoming mindful.
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.336595] AppArmor: AppArmor disabled by boot time parameter
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.339197] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.349635] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.354077] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.355666] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.357091] Initializing cgroup subsys io
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.357728] Initializing cgroup subsys memory
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.358342] Initializing cgroup subsys devices
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.359090] Initializing cgroup subsys freezer
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.359954] Initializing cgroup subsys net_cls
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.360590] Initializing cgroup subsys perf_event
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.361355] Initializing cgroup subsys net_prio
Aug 11 11:19:42 tr   0.573958] devtmpfs: initialized
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.578102] evm: security.selinux
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.578724] evm: security.SMACK64
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.579189] evm: security.SMACK64EXEC
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.579704] evm: security.SMACK64TRANSMUTE
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.580302] evm: security.SMACK64MMAP
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.580878] evm: security.ima
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.581343] evm: security.capability
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.582155] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.583659] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.585046] pinctrl core: initialized pinctrl subsystem
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.585980] RTC time: 11:19:31, date: 08/11/18
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.587421] NET: Registered protocol family 16
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.599370] cpuidle: using governor ladder
Aug 11 s-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.658370] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.686831] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.687726] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.688654] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.689640] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.691931] PCI host bridge to bus 0000:00
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.692539] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.693512] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.694609] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.695644] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.696732] pci_bus 0000:00: root bus res kernel: [    0.803306] dmi: Firmware registration failed.
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.804096] PCI: Using ACPI for IRQ routing
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.804731] PCI: pci_cache_line_size set to 64 bytes
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.804825] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.804827] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.804928] NetLabel: Initializing
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.805429] NetLabel:  domain hash size = 128
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.806064] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.806874] NetLabel:  unlabeled traffic allowed by default
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.807823] amd_nb: Cannot enumerate AMD northbridges
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.808682] clocksource: Switched to clocksource kvm-clock
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.816519] pnp: PnP ACPI init
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    0.817149] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb86122ae4 kernel: [    2.939868] zbud: loaded
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.942054] VFS: Disk quotas dquot_6.6.0
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.945181] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.949081] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.953319] fuse init (API version 7.23)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.956448] Key type big_key registered
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.958242] Allocating IMA MOK and blacklist keyrings.
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.965578] Key type asymmetric registered
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.968365] Asymmetric key parser 'x509' registered
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.970663] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.974443] io scheduler noop registered
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.976705] io scheduler deadline registered (default)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.979647] io scheduler cfq registered
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.981707] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.984580] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.989203] intel_idle: does not run on family 6 model 63
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.989297] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.993136] ACPI: Power Button [PWRF]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.995188] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    2.998908] ACPI: Sleep Button [SLPF]
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.001623] GHES: HEST is not enabled!
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.006830] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.009942] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.018603] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.021460] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.032022] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.057684] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.083376] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.109765] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.135076] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.142924] Linux agpgart interface v0.103
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.148977] loop: module loaded
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.151363] libphy: Fixed MDIO Bus: probed
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.153408] tun: Universal TUN/TAP device driver, 1.6
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.156033] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.202664] PPP generic driver version 2.4.2
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.205208] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kes-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.290224] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.296616] registered taskstats version 1
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.298654] Loading compiled-in X.509 certificates
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.301904] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.307632] zswap: loaded using pool lzo/zbud
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.313482] Key type trusted registered
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.321695] Key type encrypted registered
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.324629] ima: No TPM chip found, activating TPM-bypass!
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.327287] evm: HMAC attrs: 0x1
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.330318]   Magic number: 14:275:327
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.332226] acpi LNXCPU:a3: hash matches
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.334592] rtc_cmos 00:00: setting system clock to 2018-08-11 11:19:34 UTC (1533986374)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.339087] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.343169] EDD information not available.
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.345718] PM: Hibernation image not present or could not be loaded.
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.347421] Freeing unused kernel memory: 1496K
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.350587] Write protecting the kernel read-only data: 14336k
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.355637] Freeing unused kernel memory: 1956K
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.359456] Freeing unused kernel memory: 92K
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.379582] systemd-udevd[119]: starting version 204
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.441013] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.444600] scsi host0: Virtio SCSI HBA
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.462828] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.467961] AVX2 version of gcm_enc/dec engaged.
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    3.470493] AES CTR mode by8 85dd-ff0506122ae4 kernel: [    6.436942] floppy0: no floppy controllers found
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    7.600772] raid6: sse2x1   gen()  8727 MB/s
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    7.668774] raid6: sse2x1   xor()  6520 MB/s
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    7.736769] raid6: sse2x2   gen() 10677 MB/s
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    7.804767] raid6: sse2x2   xor()  7086 MB/s
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    7.872778] raid6: sse2x4   gen() 12295 MB/s
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    7.940770] raid6: sse2x4   xor()  8643 MB/s
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    8.008772] raid6: avx2x1   gen() 16631 MB/s
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    8.076783] raid6: avx2x2   gen() 19434 MB/s
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    8.144774] raid6: avx2x4   gen() 21522 MB/s
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    8.148790] raid6: using algorithm avx2x4 gen() 21522 MB/s
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    8.152206] raid6: using avx2x2 recovery algorithm
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    8.157786] xor: automatically using best checksumming function:
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    8.200 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    9.755111] random: cloud-init: uninitialized urandom read (32 bytes read, 41 bits of entropy available)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [    9.901403] systemd-udevd[701]: starting version 204
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [   10.023833] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [   10.079653] intel_rapl: no valid rapl domains found in package 0
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [   10.126473] intel_rapl: no valid rapl domains found in package 0
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [   10.132705] ppdev: user-space parallel port driver
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [   10.231011] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [   10.293621] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [   10.358288] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug 11 11:19:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [   10.533391] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug 11 11:19:42 travis-job-fbdd2b2f-debe4 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 11 11:19:43 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 11 11:19:43 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 11 11:19:43 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 11 11:19:43 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 11 11:19:43 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 11 11:19:43 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 11 11:19:43 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 11 11:19:43 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 11 11:19:43 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 11 11:19:43 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 11 11:19:43 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 11 11:ff0506122ae4 kernel: [   12.987819] Netfilter messages via NETLINK v0.30.
Aug 11 11:19:44 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [   12.991669] ctnetlink v0.93: registering with nfnetlink.
Aug 11 11:19:44 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [   13.092881] floppy0: no floppy controllers found
Aug 11 11:19:44 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 11 11:19:44 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 11 11:19:45 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 cron[1626]: (CRON) INFO (pidfile fd = 3)
Aug 11 11:19:45 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 11 11:19:45 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 11 11:19:45 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 cron[1665]: (CRON) STARTUP (fork ok)
Aug 11 11:19:45 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 cron[1665]: (CRON) INFO (Running @reboot jobs)
Aug 11 11:19:45 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 acpid: starting up with netlink and the input layer
Aug 11 11:19:45 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 acpid: 1 rule loaded
Aug 11 11:19:45 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 acpid: waiting for events: event logging is off
Aug 11 11:19:45 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 haveged: haveged starting up
Aug 11 11:19:50 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 startup-script: INFO Found startup-script in metadata.
Aug 11 11:19:50 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 11 11:19:50 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 startup-script: INFO startup-script: job 1 at Sat Aug 11 14:29:00 2018
Aug 11 11:19:50 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 startup-script: INFO startup-script: Return code 0.
Aug 11 11:19:50 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 startup-script: INFO startup-script: Return code 0.
Aug 11 11:19:50 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 startup-script: INFO Finished running startup scripts.
Aug 11 11:19:50 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 ec2: 
Aug 11 11:19:50 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 ec2: #############################################################
Aug 11 11:19:50 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 11 11:19:50 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 ec2: 1024 56:07:f5:9d:9d:f2:18:fd:58:01:b4:8d:1c:90:b8:fe  root@travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 (DSA)
Aug 11 11:19:50 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 ec2: 256 24:01:50:48:4d:e8:29:71:23:33:4a:7b:f0:20:fc:08  root@travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 (ECDSA)
Aug 11 11:19:50 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 ec2: 256 d0:1e:b2:e8:8c:36:9d:fa:94:d2:47:ca:c5:35:45:89  root@travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 (ED25519)
Aug 11 11:19:50 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 ec2: 20e3c09de8.so[7f40ae83e000+16e000]
Aug 11 12:19:36 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [ 3605.782677] traps: a[22814] trap invalid opcode ip:55b6fe520d98 sp:7ffeda69e440 error:0 in a[55b6fe51e000+4000]
Aug 11 12:22:31 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [ 3780.461465] a[18815]: segfault at 0 ip 0000563f444a8463 sp 00007fffa95db510 error 6 in a[563f444a5000+5000]
Aug 11 12:22:42 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [ 3791.071248] a[19584]: segfault at 1 ip 000055e385c32b8c sp 00007ffe9058dc70 error 6 in a[55e385c30000+4000]
Aug 11 12:22:46 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [ 3795.175380] traps: a[19956] trap invalid opcode ip:55a1cc4c642c sp:7fffcea9a7d0 error:0 in a[55a1cc4c3000+7000]
Aug 11 12:41:01 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [ 4890.241093] veth956aea3: renamed from eth0
Aug 11 12:41:01 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [ 4890.255134] docker0: port 1(veth2a90d44) entered disabled state
Aug 11 12:41:01 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [ 4890.301579] docker0: port 1(veth2a90d44) entered disabled state
Aug 11 12:41:01 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [ 4890.303224] device veth2a90d44 left promiscuous mode
Aug 11 12:41:01 travis-job-fbdd2b2f-debf-4fb8-85dd-ff0506122ae4 kernel: [ 4890.303227] docker0: port 1(veth2a90d44) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:33d2fd06
/src
