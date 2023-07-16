plain
######################################################################    97.5%
######################################################################## 100.0%
[00:01:44] extracting /checkout/obj/build/cache/2018-08-01/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:14] downloading https://static.rust-lang.org/dist/2018-08-01/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:43] Warning: Transient problem: timeout Will retry in 1 seconds. 3 retries left.
####################################################                      73.3%
######################################################################## 100.0%
[00:02:50] extracting /checkout/obj/build/cache/2018-08-01/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:50]     Updating registry `https://github.com/rust-lang/crates.io-index`
---

[00:08:28] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:08:28] tidy error: /checkout/src/librustc_resolve/lib.rs:4273: line longer than 100 chars
[00:08:29] some tidy checks failed
[00:08:29] 
[00:08:29] 
[00:08:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:08:29] 
[00:08:29] 
[00:08:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:08:29] Build completed unsuccessfully in 0:00:53
[00:08:29] Build completed unsuccessfully in 0:00:53
[00:08:29] Makefile:79: recipe for target 'tidy' failed
[00:08:29] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:19e40764
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:3028d71b
$ sudo tail -n 500 /var/log/syslog
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   2 disabled
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   3 disabled
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   4 disabled
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   5 disabled
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   6 disabled
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   7 disabled
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Using GB pages for direct mapping
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] kvm-clock: using sched offset of 1369600207 cycles
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Zone ranges:
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   Device   empty
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Movable zone start for each node
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Early memory node ranges
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Policy zone: Normal
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] console [ttyS0] enabled
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.325739] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.327016] pid_max: default: 32768 minimum: 301
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.327772] ACPI: Core revision 20150930
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.334175] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.335327] Security Framework initialized
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.336055] Yama: becoming mindful.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.336580] AppArmor: AppArmor disabled by boot time parameter
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.339448] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.348560] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.353701] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.354694] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.355968] Initializing cgroup subsys io
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.356557] Initializing cgroup subsys memory
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.357738] Initializing cgroup subsys devices
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.358971] Initializing cgroup subsys freezer
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.359595] Initializing cgroup subsys net_cls
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.360291] Initializing cgroup subsys perf_event
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.361133] Initializing cgroup subsys net_prio
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.362102] Initializing cgroup subsys hugetlb
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.363026] Initializing cgroup subsys pids
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.363775] CPU: Physical Processor ID: 0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.364380] CPU: Processor Core ID: 0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.365251] mce: CPU supports 32 MCE banks
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.366159] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.367061] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.369933] Freeing SMP alternatives memory: 32K
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.378610] ftrace: allocating 32185 entries in 126 pages
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.425137] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.426066] smpboot: Max logical packages: 2
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.427343] x2apic enabled
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.429085] Switched APIC routing to physical x2apic.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.432889] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.541003] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.542711] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.545132] x86: Booting SMP configuration:
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.545820] .... node  #0, CPUs:      #1
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.546635] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.549935]  #2
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.550458] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.553697]  #3
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.554106] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.557278] x86: Booted up 1 node, 4 CPUs
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.557993] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.560386] devtmpfs: initialized
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.564861] evm: security.selinux
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.565384] evm: security.SMACK64
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.566028] evm: security.SMACK64EXEC
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.566606] evm: security.SMACK64TRANSMUTE
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.567221] evm: security.SMACK64MMAP
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.567868] evm: security.ima
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.568302] evm: security.capability
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.569136] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.570633] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.571703] pinctrl core: initialized pinctrl subsystem
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.572636] RTC time: 13:57:21, date: 08/10/18
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.574073] NET: Registered protocol family 16
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.585056] cpuidle: using governor ladder
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.597033] cpuidle: using governor menu
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.597986] PCCT header not found.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.598597] ACPI: bus type PCI registered
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.599236] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.600647] PCI: Using configuration type 1 for base access
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.614055] ACPI: Added _OSI(Module Device)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.614834] ACPI: Added _OSI(Processor Device)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.615479] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.616139] ACPI: Added _OSI(Processor Aggregator Device)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.619403] ACPI: Executed 2 blocks of module-level executable AML code
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.642636] ACPI: Interpreter enabled
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.643381] ACPI: (supports S0 S3 S4 S5)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.644011] ACPI: Using IOAPIC for interrupt routing
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.644764] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.673383] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.674342] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.675369] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.676303] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.678528] PCI host bridge to bus 0000:00
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.679185] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.680253] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.681257] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.682520] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.683708] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.684528] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.684926] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.699210] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.712699] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.714102] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.718628] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.722417] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.732971] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.737671] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.741276] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.755071] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.757480] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.759640] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.761899] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.764029] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.783899] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.785120] vgaarb: loaded
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.785744] SCSI subsystem initialized
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.786414] libata version 3.00 loaded.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.786433] ACPI: bus type USB registered
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.787042] usbcore: registered new interface driver usbfs
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.787877] usbcore: registered new interface driver hub
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.788641] usbcore: registered new device driver usb
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.789529] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.790518] dmi: Firmware registration failed.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.791393] PCI: Using ACPI for IRQ routing
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.792071] PCI: pci_cache_line_size set to 64 bytes
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.792162] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.792163] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.792292] NetLabel: Initializing
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.792771] NetLabel:  domain hash size = 128
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.793364] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.794056] NetLabel:  unlabeled traffic allowed by default
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.795057] amd_nb: Cannot enumerate AMD northbridges
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.795816] clocksource: Switched to clocksource kvm-clock
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.803284] pnp: PnP ACPI init
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.803860] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.803930] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.803976] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.804026] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.804069] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.804113] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.804154] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.804323] pnp: PnP ACPI: found 7 devices
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.811627] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.812936] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.812938] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.812940] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.812941] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.812971] NET: Registered protocol family 2
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.813810] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.815196] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.816358] TCP: Hash tables configured (established 131072 bind 65536)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.817343] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.818241] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.819870] NET: Registered protocol family 1
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.820662] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.821604] PCI: CLS 0 bytes, default 64
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    0.821654] Unpacking initramfs...
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.826053] Freeing initrd memory: 21432K
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.826772] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.827780] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.829369] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.830771] hw unit of domain pp0-core 2^-0 Joules
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.831569] hw unit of domain package 2^-0 Joules
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.832293] hw unit of domain dram 2^-0 Joules
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.833128] Scanning for low memory corruption every 60 seconds
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.834618] audit: initializing netlink subsys (disabled)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.835553] audit: type=2000 audit(1533909443.405:1): initialized
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.836764] Initialise system trusted keyring
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.837622] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.838668] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.840897] zbud: loaded
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.841619] VFS: Disk quotas dquot_6.6.0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.842281] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.843485] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.844933] fuse init (API version 7.23)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.845812] Key type big_key registered
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.846423] Allocating IMA MOK and blacklist keyrings.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.848050] Key type asymmetric registered
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.848847] Asymmetric key parser 'x509' registered
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.849721] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.850875] io scheduler noop registered
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.851709] io scheduler deadline registered (default)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.852567] io scheduler cfq registered
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.853234] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.854060] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.855092] intel_idle: does not run on family 6 model 45
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.855191] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.856312] ACPI: Power Button [PWRF]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.856952] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.858086] ACPI: Sleep Button [SLPF]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.859039] GHES: HEST is not enabled!
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.861111] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.862089] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.866070] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.867083] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.871916] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.894648] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.917800] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.940554] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.963731] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.967110] Linux agpgart interface v0.103
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.970717] loop: module loaded
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.971560] libphy: Fixed MDIO Bus: probed
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.972438] tun: Universal TUN/TAP device driver, 1.6
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    2.973349] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.002471] PPP generic driver version 2.4.2
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.003706] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.005262] ehci-pci: EHCI PCI platform driver
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.006153] ehci-platform: EHCI generic platform driver
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.007564] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.008924] ohci-pci: OHCI PCI platform driver
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.010068] ohci-platform: OHCI generic platform driver
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.011114] uhci_hcd: USB Universal Host Controller Interface driver
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.012621] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.014935] i8042: Warning: Keylock active
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.016792] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.017689] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.018735] mousedev: PS/2 mouse device common for all mice
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.020254] rtc_cmos 00:00: RTC can wake from S4
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.021603] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.023041] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.024613] i2c /dev entries driver
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.025258] device-mapper: uevent: version 1.0.3
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.026357] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.028306] ledtrig-cpu: registered to indicate activity on CPUs
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.029983] NET: Registered protocol family 10
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.031107] NET: Registered protocol family 17
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.032057] Key type dns_resolver registered
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.033306] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.034497] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.036092] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.037419] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.038931] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.040978] registered taskstats version 1
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.041661] Loading compiled-in X.509 certificates
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.043171] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.044774] zswap: loaded using pool lzo/zbud
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.047567] Key type trusted registered
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.051697] Key type encrypted registered
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.052587] ima: No TPM chip found, activating TPM-bypass!
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.053845] evm: HMAC attrs: 0x1
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.054857]   Magic number: 14:583:988
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.055651] machinecheck machinecheck1: hash matches
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.056999] rtc_cmos 00:00: setting system clock to 2018-08-10 13:57:23 UTC (1533909443)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.059079] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.060363] EDD information not available.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.061343] PM: Hibernation image not present or could not be loaded.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.062668] Freeing unused kernel memory: 1496K
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.063369] Write protecting the kernel read-only data: 14336k
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.065053] Freeing unused kernel memory: 1956K
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.065962] Freeing unused kernel memory: 92K
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.078706] systemd-udevd[120]: starting version 204
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.133821] scsi host0: Virtio SCSI HBA
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.140222] AVX version of gcm_enc/dec engaged.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.141203] AES CTR mode by8 optimization enabled
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.141272] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.177343] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.177357] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.177359] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.177527] sd 0:0:1:0: [sda] Write Protect is off
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.177530] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.177568] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.178939]  sda: sda1
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.179619] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.220316] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.831970] tsc: Refined TSC clocksource calibration: 2600.001 MHz
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    3.833212] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3ce1c4c, max_idle_ns: 440795206275 ns
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    4.057359] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    6.127979] floppy0: no floppy controllers found
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.283847] raid6: sse2x1   gen()  9033 MB/s
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.351929] raid6: sse2x1   xor()  6670 MB/s
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.419867] raid6: sse2x2   gen() 11241 MB/s
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.487855] raid6: sse2x2   xor()  7322 MB/s
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.555898] raid6: sse2x4   gen() 13023 MB/s
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.623844] raid6: sse2x4   xor()  9035 MB/s
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.624804] raid6: using algorithm sse2x4 gen() 13023 MB/s
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.625547] raid6: .... xor() 9035 MB/s, rmw enabled
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.626600] raid6: using ssse3x2 recovery algorithm
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.628946] xor: automatically using best checksumming function:
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.667850]    avx       : 27856.000 MB/sec
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.681837] Btrfs loaded
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.721428] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.722648] EXT4-fs (sda1): write access will be enabled during recovery
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.801766] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.807573] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.808428] EXT4-fs (sda1): recovery complete
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.812710] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    7.999430] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    8.103157] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    8.150041] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    8.321239] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    8.808298] random: cloud-init: uninitialized urandom read (32 bytes read, 45 bits of entropy available)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    8.920916] systemd-udevd[703]: starting version 204
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    9.015339] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    9.056028] intel_rapl: no valid rapl domains found in package 0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    9.096598] intel_rapl: no valid rapl domains found in package 0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    9.112525] ppdev: user-space parallel port driver
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    9.190990] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    9.232049] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    9.290549] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    9.449485] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    9.680427] random: mktemp: uninitialized urandom read (12 bytes read, 60 bits of entropy available)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    9.747893] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    9.811518] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [    9.839750] EXT4-fs (sda1): resized filesystem to 7864064
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [   10.280452] init: failsafe main process (1095) killed by TERM signal
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO Running set_multiqueue.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO Set channels for eth0 to 4.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 10 13:57:30 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 10 13:57:31 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 10 13:57:31 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 10 13:57:31 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 10 13:57:31 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 10 13:57:31 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 13:57:31 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 13:57:31 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [   10.982572] random: nonblocking pool is initialized
Aug 10 13:57:31 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 google-accounts: INFO Starting Google Accounts daemon.
Aug 10 13:57:31 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 google-accounts: INFO Creating a new user account for me.
Aug 10 13:57:31 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 google-accounts: INFO Created user account me.
Aug 10 13:57:31 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 google-accounts: INFO Creating a new user account for bogdana.
Aug 10 13:57:31 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 10 13:57:31 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 google-accounts: INFO Created user account bogdana.
Aug 10 13:57:31 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 google-accounts: INFO Creating a new user account for aj.
Aug 10 13:57:31 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 google-accounts: INFO Created user account aj.
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 google-clock-skew: INFO Synced system time with hardware clock.
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 google-accounts: INFO Creating a new user account for asari.
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 google-accounts: INFO Created user account asari.
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 google-accounts: INFO Removing user packer.
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 cron[1435]: (CRON) INFO (pidfile fd = 3)
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 cron[1475]: (CRON) STARTUP (fork ok)
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 cron[1475]: (CRON) INFO (Running @reboot jobs)
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 acpid: starting up with netlink and the input layer
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 acpid: 1 rule loaded
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 acpid: waiting for events: event logging is off
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 haveged: haveged starting up
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [   11.482902] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [   11.496684] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [   11.621440] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [   11.624449] Bridge firewalling registered
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [   11.636980] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [   11.690741] Initializing XFRM netlink socket
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [   11.696639] Netfilter messages via NETLINK v0.30.
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [   11.698965] ctnetlink v0.93: registering with nfnetlink.
Aug 10 13:57:32 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [   12.071941] floppy0: no floppy controllers found
Aug 10 13:57:55 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpdate[1794]: adjust time server 169.254.169.254 offset 0.004776 sec
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpd[1829]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpd[1830]: proto: precision = 0.137 usec
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpd[1830]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpd[1830]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpd[1830]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpd[1830]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpd[1830]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpd[1830]: Listen normally on 3 eth0 10.20.255.33 UDP 123
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpd[1830]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpd[1830]: peers refreshed
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpd[1830]: Listening on routing socket on fd #21 for interface updates
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [   41.657610] init: plymouth-upstart-bridge main process ended, respawning
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 startup-script: INFO Found startup-script in metadata.
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 startup-script: INFO startup-script: job 1 at Fri Aug 10 17:08:00 2018
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 startup-script: INFO startup-script: Return code 0.
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 startup-script: INFO startup-script: Return code 0.
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 startup-script: INFO Finished running startup scripts.
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ec2: 
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ec2: #############################################################
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ec2: 1024 00:ce:de:58:91:74:49:7f:85:4e:9a:90:b5:e1:c9:d2  root@travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 (DSA)
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ec2: 256 e9:ba:a6:52:27:36:d9:26:38:44:fa:c2:35:c0:58:a0  root@travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 (ECDSA)
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ec2: 256 ae:a1:2a:ac:2b:23:3a:95:89:13:e0:60:60:de:20:02  root@travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 (ED25519)
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ec2: 2048 6e:1e:db:0e:d4:36:68:71:f5:1a:40:f7:67:d9:84:84  root@travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 (RSA)
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 10 13:58:02 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ec2: #############################################################
Aug 10 14:02:07 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [  287.014883] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 10 14:03:43 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [  382.834320] device vethb7e7758 entered promiscuous mode
Aug 10 14:03:43 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [  382.921514] cgroup: docker-runc (4654) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 10 14:03:43 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [  382.921516] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 10 14:03:43 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [  382.985479] eth0: renamed from veth8b79ef7
Aug 10 14:03:43 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [  383.025124] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 10 14:03:43 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [  383.026254] docker0: port 1(vethb7e7758) entered forwarding state
Aug 10 14:03:43 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [  383.026270] docker0: port 1(vethb7e7758) entered forwarding state
Aug 10 14:03:43 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [  383.026306] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 10 14:03:46 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpd[1830]: Listen normally on 5 docker0 fe80::42:a3ff:fe98:1903 UDP 123
Aug 10 14:03:46 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpd[1830]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 10 14:03:46 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpd[1830]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 10 14:03:46 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpd[1830]: peers refreshed
Aug 10 14:03:46 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 ntpd[1830]: new interface(s) found: waking up resolver
Aug 10 14:03:58 travis-job-4e27fbdd-4d9c-4786-b44f-b0af0a7a5078 kernel: [  398.060830] docker0: port 1(vethb7e7758) entered forwarding state
---
travis_time:end:12f6ce83:start=1533910270018010485,finish=1533910270023700874,duration=5690389
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3206958c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:024c1980
travis_time:start:024c1980
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:1056a658
$ dmesg | grep -i kill
