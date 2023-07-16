plain
[00:00:00] Submodule 'src/tools/miri' (https://github.com/solson/miri.git) registered for path 'src/tools/miri'
[00:00:00] Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
[00:00:00] Submodule 'src/rust-installer' (https://github.com/rust-lang/rust-installer.git) registered for path 'src/tools/rust-installer'
[00:00:00] Submodule 'src/tools/rustfmt' (https://github.com/rust-lang-nursery/rustfmt.git) registered for path 'src/tools/rustfmt'
[00:00:00] tar: This does not look like a tar archive
[00:00:00] gzip: stdin: not in gzip format
[00:00:00] tar: Child returned status 1
[00:00:00] tar: Error is not recoverable: exiting now
[00:00:00] Cloning into '/home/travis/build/rust-lang/rust/src/dlmalloc'...
[00:00:00] Cloning into '/home/travis/build/rust-lang/rust/src/dlmalloc'...
[00:00:00] tar: This does not look like a tar archive
[00:00:00] gzip: stdin: not in gzip format
[00:00:00] tar: Child returned status 1
[00:00:00] tar: Error is not recoverable: exiting now
[00:00:00] tar: Error is not recoverable: exiting now
[00:00:00] tar: This does not look like a tar archive
[00:00:00] gzip: stdin: not in gzip format
[00:00:00] tar: Child returned status 1
[00:00:00] tar: Error is not recoverable: exiting now
[00:00:00] tar: Error is not recoverable: exiting now
[00:00:00] tar: This does not look like a tar archive
[00:00:00] gzip: stdin: not in gzip format
[00:00:00] tar: Child returned status 1
[00:00:00] tar: Error is not recoverable: exiting now
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rust-installer'...
---

[00:06:06] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:06:06] tidy error: /checkout/src/librustc_lint/builtin.rs:1966: line longer than 100 chars
[00:06:06] tidy error: /checkout/src/librustc_lint/builtin.rs:1984: line longer than 100 chars
[00:06:07] some tidy checks failed
[00:06:07] 
[00:06:07] 
[00:06:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:06:07] 
[00:06:07] 
[00:06:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:06:07] Build completed unsuccessfully in 0:00:55
[00:06:07] Build completed unsuccessfully in 0:00:55
[00:06:07] make: *** [tidy] Error 1
[00:06:07] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03258084
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:16ef8526
$ sudo tail -n 500 /var/log/syslog
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   5 disabled
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   6 disabled
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   7 disabled
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Using GB pages for direct mapping
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] kvm-clock: using sched offset of 2645845335 cycles
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Zone ranges:
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   Device   empty
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Movable zone start for each node
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Early memory node ranges
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Policy zone: Normal
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Hierarchical RCU implementation.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] console [ttyS0] enabled
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.697784] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.705248] pid_max: default: 32768 minimum: 301
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.707923] ACPI: Core revision 20150930
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.716662] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.721818] Security Framework initialized
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.724974] Yama: becoming mindful.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.728676] AppArmor: AppArmor disabled by boot time parameter
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.734779] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.748734] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.757684] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.762056] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.767606] Initializing cgroup subsys io
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.770100] Initializing cgroup subsys memory
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.773501] Initializing cgroup subsys devices
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.776766] Initializing cgroup subsys freezer
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.779407] Initializing cgroup subsys net_cls
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.781859] Initializing cgroup subsys perf_event
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.785502] Initializing cgroup subsys net_prio
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.789007] Initializing cgroup subsys hugetlb
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.792583] Initializing cgroup subsys pids
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.795575] CPU: Physical Processor ID: 0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.798008] CPU: Processor Core ID: 0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.800867] mce: CPU supports 32 MCE banks
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.803624] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.807282] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.813727] Freeing SMP alternatives memory: 32K
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.826412] ftrace: allocating 32185 entries in 126 pages
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.884093] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.888239] smpboot: Max logical packages: 2
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.891375] x2apic enabled
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.894929] Switched APIC routing to physical x2apic.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    0.900882] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.010347] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.016897] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.023530] x86: Booting SMP configuration:
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.025838] .... node  #0, CPUs:      #1
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.028080] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.034796]  #2
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.036089] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.045653]  #3
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.047099] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.053566] x86: Booted up 1 node, 4 CPUs
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.056196] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.061606] devtmpfs: initialized
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.067306] evm: security.selinux
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.069112] evm: security.SMACK64
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.071272] evm: security.SMACK64EXEC
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.073513] evm: security.SMACK64TRANSMUTE
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.075904] evm: security.SMACK64MMAP
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.078402] evm: security.ima
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.080324] evm: security.capability
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.083462] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.089046] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.092647] pinctrl core: initialized pinctrl subsystem
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.096441] RTC time:  4:43:26, date: 08/12/18
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.100131] NET: Registered protocol family 16
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.114424] cpuidle: using governor ladder
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.126440] cpuidle: using governor menu
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.129264] PCCT header not found.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.131097] ACPI: bus type PCI registered
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.133284] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.137854] PCI: Using configuration type 1 for base access
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.152243] ACPI: Added _OSI(Module Device)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.154565] ACPI: Added _OSI(Processor Device)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.157509] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.159901] ACPI: Added _OSI(Processor Aggregator Device)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.166508] ACPI: Executed 2 blocks of module-level executable AML code
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.192930] ACPI: Interpreter enabled
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.194825] ACPI: (supports S0 S3 S4 S5)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.197279] ACPI: Using IOAPIC for interrupt routing
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.201679] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.236962] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.240759] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.245363] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.249818] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.258510] PCI host bridge to bus 0000:00
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.260508] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.264255] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.267213] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.271498] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.275283] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.278301] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.278781] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.309029] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.338113] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.342731] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.353407] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.361984] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.388975] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.401153] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.410933] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.436957] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.444692] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.450916] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.457577] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.466627] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.491923] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.495392] vgaarb: loaded
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.497734] SCSI subsystem initialized
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.500128] libata version 3.00 loaded.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.500158] ACPI: bus type USB registered
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.503522] usbcore: registered new interface driver usbfs
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.506567] usbcore: registered new interface driver hub
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.510346] usbcore: registered new device driver usb
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.514314] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.519000] dmi: Firmware registration failed.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.522025] PCI: Using ACPI for IRQ routing
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.524699] PCI: pci_cache_line_size set to 64 bytes
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.524803] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.524806] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.524948] NetLabel: Initializing
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.526937] NetLabel:  domain hash size = 128
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.529187] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.532548] NetLabel:  unlabeled traffic allowed by default
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.536046] amd_nb: Cannot enumerate AMD northbridges
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.539466] clocksource: Switched to clocksource kvm-clock
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.550119] pnp: PnP ACPI init
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.552322] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.552398] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.552446] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.552534] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.552580] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.552629] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.552682] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.552861] pnp: PnP ACPI: found 7 devices
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.562704] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.567967] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.567970] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.567972] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.567973] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.568011] NET: Registered protocol family 2
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.571089] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.575405] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.579170] TCP: Hash tables configured (established 131072 bind 65536)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.583003] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.586134] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.591534] NET: Registered protocol family 1
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.594557] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.597458] PCI: CLS 0 bytes, default 64
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    1.597549] Unpacking initramfs...
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.655327] Freeing initrd memory: 21432K
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.656363] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.657533] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.659335] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.661421] hw unit of domain pp0-core 2^-0 Joules
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.662305] hw unit of domain package 2^-0 Joules
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.663582] hw unit of domain dram 2^-0 Joules
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.664510] Scanning for low memory corruption every 60 seconds
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.666313] audit: initializing netlink subsys (disabled)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.667209] audit: type=2000 audit(1534049008.060:1): initialized
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.668846] Initialise system trusted keyring
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.669850] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.670864] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.673387] zbud: loaded
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.674400] VFS: Disk quotas dquot_6.6.0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.675293] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.676897] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.678333] fuse init (API version 7.23)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.679255] Key type big_key registered
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.680031] Allocating IMA MOK and blacklist keyrings.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.682443] Key type asymmetric registered
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.683155] Asymmetric key parser 'x509' registered
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.683931] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.685228] io scheduler noop registered
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.686041] io scheduler deadline registered (default)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.687074] io scheduler cfq registered
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.687747] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.688804] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.689776] intel_idle: does not run on family 6 model 45
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.689868] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.690938] ACPI: Power Button [PWRF]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.691533] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.692561] ACPI: Sleep Button [SLPF]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.693543] GHES: HEST is not enabled!
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.695955] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.697995] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.702669] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.703553] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.708182] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.730687] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.754143] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.778717] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.802674] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.806873] Linux agpgart interface v0.103
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.810535] loop: module loaded
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.811691] libphy: Fixed MDIO Bus: probed
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.812761] tun: Universal TUN/TAP device driver, 1.6
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.813934] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.846505] PPP generic driver version 2.4.2
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.847933] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.849051] ehci-pci: EHCI PCI platform driver
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.849970] ehci-platform: EHCI generic platform driver
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.851317] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.852811] ohci-pci: OHCI PCI platform driver
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.853653] ohci-platform: OHCI generic platform driver
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.854578] uhci_hcd: USB Universal Host Controller Interface driver
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.855670] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.857866] i8042: Warning: Keylock active
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.859734] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.860991] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.862281] mousedev: PS/2 mouse device common for all mice
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.863886] rtc_cmos 00:00: RTC can wake from S4
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.865264] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.866874] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.868183] i2c /dev entries driver
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.869140] device-mapper: uevent: version 1.0.3
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.870123] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.872005] ledtrig-cpu: registered to indicate activity on CPUs
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.874169] NET: Registered protocol family 10
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.875354] NET: Registered protocol family 17
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.876418] Key type dns_resolver registered
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.877488] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.878680] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.880610] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.882436] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.883545] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.885595] registered taskstats version 1
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.886490] Loading compiled-in X.509 certificates
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.887878] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.889828] zswap: loaded using pool lzo/zbud
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.892885] Key type trusted registered
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.897427] Key type encrypted registered
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.898465] ima: No TPM chip found, activating TPM-bypass!
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.899339] evm: HMAC attrs: 0x1
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.900319]   Magic number: 14:20:717
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.901127] acpi LNXCPU:50: hash matches
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.902404] rtc_cmos 00:00: setting system clock to 2018-08-12 04:43:28 UTC (1534049008)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.904237] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.905461] EDD information not available.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.906412] PM: Hibernation image not present or could not be loaded.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.907834] Freeing unused kernel memory: 1496K
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.908531] Write protecting the kernel read-only data: 14336k
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.910251] Freeing unused kernel memory: 1956K
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.911508] Freeing unused kernel memory: 92K
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.925854] systemd-udevd[120]: starting version 204
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.977200] scsi host0: Virtio SCSI HBA
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.980884] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.988418] AVX version of gcm_enc/dec engaged.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    3.989130] AES CTR mode by8 optimization enabled
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    4.022752] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    4.022758] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    4.024816] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    4.025862] sd 0:0:1:0: [sda] Write Protect is off
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    4.026572] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    4.026685] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    4.029923]  sda: sda1
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    4.031300] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    4.067792] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    4.663653] tsc: Refined TSC clocksource calibration: 2600.001 MHz
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    4.664729] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3ce1c4c, max_idle_ns: 440795206275 ns
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    4.900624] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    6.971651] floppy0: no floppy controllers found
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.151586] raid6: sse2x1   gen()  8649 MB/s
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.219642] raid6: sse2x1   xor()  6435 MB/s
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.287554] raid6: sse2x2   gen() 10748 MB/s
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.355557] raid6: sse2x2   xor()  7176 MB/s
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.423544] raid6: sse2x4   gen() 12563 MB/s
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.491559] raid6: sse2x4   xor()  8860 MB/s
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.493800] raid6: using algorithm sse2x4 gen() 12563 MB/s
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.496945] raid6: .... xor() 8860 MB/s, rmw enabled
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.500203] raid6: using ssse3x2 recovery algorithm
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.505254] xor: automatically using best checksumming function:
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.547581]    avx       : 27389.000 MB/sec
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.564738] Btrfs loaded
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.625213] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.629437] EXT4-fs (sda1): write access will be enabled during recovery
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.745836] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.757306] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.760328] EXT4-fs (sda1): recovery complete
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    8.769222] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    9.034629] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    9.184900] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    9.244566] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [    9.491702] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   10.127720] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   10.269956] systemd-udevd[705]: starting version 204
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   10.413214] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   10.456943] intel_rapl: no valid rapl domains found in package 0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   10.504617] intel_rapl: no valid rapl domains found in package 0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   10.511072] ppdev: user-space parallel port driver
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   10.654658] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   10.713617] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   10.781649] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   10.955915] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   11.264655] random: mktemp: uninitialized urandom read (12 bytes read, 57 bits of entropy available)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   11.350233] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   11.442886] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   11.493113] EXT4-fs (sda1): resized filesystem to 7864064
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   11.782048] init: failsafe main process (1096) killed by TERM signal
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO Running set_multiqueue.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO Set channels for eth0 to 4.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 12 04:43:36 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 12 04:43:37 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e google-clock-skew: INFO Clock drift token has changed: 0.
Aug 12 04:43:37 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e google-clock-skew: INFO Clock drift token has changed: 0.
Aug 12 04:43:37 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e google-accounts: INFO Starting Google Accounts daemon.
Aug 12 04:43:37 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e google-accounts: INFO Creating a new user account for me.
Aug 12 04:43:37 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 12 04:43:37 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e google-accounts: INFO Created user account me.
Aug 12 04:43:37 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e google-accounts: INFO Creating a new user account for bogdana.
Aug 12 04:43:37 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e google-accounts: INFO Created user account bogdana.
Aug 12 04:43:37 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e google-accounts: INFO Creating a new user account for aj.
Aug 12 04:43:37 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e google-accounts: INFO Created user account aj.
Aug 12 04:43:37 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e google-accounts: INFO Creating a new user account for asari.
Aug 12 04:43:37 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e google-accounts: INFO Created user account asari.
Aug 12 04:43:37 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e google-accounts: INFO Removing user packer.
Aug 12 04:43:38 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e google-clock-skew: INFO Synced system time with hardware clock.
Aug 12 04:43:38 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   13.236096] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 12 04:43:38 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   13.239455] Bridge firewalling registered
Aug 12 04:43:38 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   13.251950] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 12 04:43:38 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   13.291232] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 12 04:43:38 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   13.362551] Initializing XFRM netlink socket
Aug 12 04:43:38 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   13.370901] Netfilter messages via NETLINK v0.30.
Aug 12 04:43:38 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   13.374291] ctnetlink v0.93: registering with nfnetlink.
Aug 12 04:43:38 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   13.491927] floppy0: no floppy controllers found
Aug 12 04:43:38 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 12 04:43:38 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e pollinate: To re-seed this system again, use the -r|--reseed option
Aug 12 04:43:38 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   13.800151] random: nonblocking pool is initialized
Aug 12 04:43:42 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 12 04:43:42 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e pollinate: To re-seed this system again, use the -r|--reseed option
Aug 12 04:43:42 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e cron[1633]: (CRON) INFO (pidfile fd = 3)
Aug 12 04:43:42 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e cron[1667]: (CRON) STARTUP (fork ok)
Aug 12 04:43:42 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e cron[1667]: (CRON) INFO (Running @reboot jobs)
Aug 12 04:43:42 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e acpid: starting up with netlink and the input layer
Aug 12 04:43:42 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e acpid: 1 rule loaded
Aug 12 04:43:42 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e acpid: waiting for events: event logging is off
Aug 12 04:43:42 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e haveged: haveged starting up
Aug 12 04:43:42 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   17.465400] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpd[1768]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpd[1769]: proto: precision = 0.100 usec
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpd[1769]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpd[1769]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpd[1769]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpd[1769]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpd[1769]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpd[1769]: Listen normally on 3 eth0 10.20.0.41 UDP 123
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpd[1769]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpd[1769]: peers refreshed
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpd[1769]: Listening on routing socket on fd #21 for interface updates
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   22.657123] init: plymouth-upstart-bridge main process ended, respawning
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e startup-script: INFO Found startup-script in metadata.
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e startup-script: INFO startup-script: job 1 at Sun Aug 12 07:53:00 2018
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e startup-script: INFO startup-script: Return code 0.
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e startup-script: INFO startup-script: Return code 0.
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e startup-script: INFO Finished running startup scripts.
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ec2: 
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ec2: #############################################################
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ec2: 1024 92:fa:39:f1:61:c7:04:f0:83:1d:57:99:d2:9d:e7:1c  root@travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e (DSA)
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ec2: 256 6f:a2:16:5f:3b:e3:17:79:53:8d:a4:4f:c2:5a:e5:ea  root@travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e (ECDSA)
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ec2: 256 d5:5e:2c:b2:c6:a5:04:b3:b0:14:a8:55:19:5e:0d:02  root@travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e (ED25519)
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ec2: 2048 65:e7:b8:69:3b:ca:20:30:c4:78:65:36:cd:7f:ed:17  root@travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e (RSA)
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 12 04:43:47 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ec2: #############################################################
Aug 12 04:43:53 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpdate[1850]: the NTP socket is in use, exiting
Aug 12 04:45:01 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [   96.125409] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 12 04:46:16 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [  171.105731] device veth7cbbbbd entered promiscuous mode
Aug 12 04:46:16 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [  171.105813] docker0: port 1(veth7cbbbbd) entered forwarding state
Aug 12 04:46:16 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [  171.105820] docker0: port 1(veth7cbbbbd) entered forwarding state
Aug 12 04:46:16 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [  171.106226] docker0: port 1(veth7cbbbbd) entered disabled state
Aug 12 04:46:16 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [  171.193520] cgroup: docker-runc (4711) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 12 04:46:16 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [  171.193522] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 12 04:46:16 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [  171.252881] eth0: renamed from veth56a9e7c
Aug 12 04:46:16 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [  171.288790] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 12 04:46:16 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [  171.289948] docker0: port 1(veth7cbbbbd) entered forwarding state
Aug 12 04:46:16 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [  171.289970] docker0: port 1(veth7cbbbbd) entered forwarding state
Aug 12 04:46:16 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [  171.289991] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 12 04:46:19 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpd[1769]: Listen normally on 5 docker0 fe80::42:e3ff:fe83:27ac UDP 123
Aug 12 04:46:19 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpd[1769]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 12 04:46:19 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpd[1769]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 12 04:46:19 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpd[1769]: peers refreshed
Aug 12 04:46:19 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e ntpd[1769]: new interface(s) found: waking up resolver
Aug 12 04:46:31 travis-job-8d70389f-8efb-4125-ad1c-d40406a3b92e kernel: [  186.284857] docker0: port 1(veth7cbbbbd) entered forwarding state
---
travis_time:end:01294ec3:start=1534049470243221768,finish=1534049470249867989,duration=6646221
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a27401b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10096278
travis_time:start:10096278
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:057f3fcf
$ dmesg | grep -i kill
