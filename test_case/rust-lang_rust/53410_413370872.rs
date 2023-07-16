plain
#########                                                                 13.5%
######################################################################## 100.0%
[00:02:00] extracting /checkout/obj/build/cache/2018-08-01/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:00]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:33] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:33] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:33] make: *** [prepare] Error 1
[00:02:33] Makefile:81: recipe for target 'prepare' failed
[00:02:34] Command failed. Attempt 2/5:
[00:02:34]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:34]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:35] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:35] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:35] Makefile:81: recipe for target 'prepare' failed
[00:02:35] make: *** [prepare] Error 1
[00:02:37] Command failed. Attempt 3/5:
[00:02:37]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:37]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:37] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:37] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:37] make: *** [prepare] Error 1
[00:02:37] Makefile:81: recipe for target 'prepare' failed
[00:02:40] Command failed. Attempt 4/5:
[00:02:40]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:40]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:41] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:41] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:41] Makefile:81: recipe for target 'prepare' failed
[00:02:41] make: *** [prepare] Error 1
[00:02:45] Command failed. Attempt 5/5:
[00:02:45]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:45]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:45] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:45] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:45] make: *** [prepare] Error 1
[00:02:45] Makefile:81: recipe for target 'prepare' failed
[00:02:45] The command has failed after 5 attempts.
travis_time:end:00011616:start=1534375281499107522,finish=1534375449380722571,duration=167881615049
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:026a24a2
$ sudo tail -n 500 /var/log/syslog
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] kvm-clock: using sched offset of 1890542065 cycles
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] Zone ranges:
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000]   Device   empty
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] Movable zone start for each node
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] Early memory node ranges
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] Policy zone: Normal
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] console [ttyS0] enabled
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.457141] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.459406] pid_max: default: 32768 minimum: 301
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.460762] ACPI: Core revision 20150930
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.467881] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.469662] Security Framework initialized
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.470572] Yama: becoming mindful.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.471893] AppArmor: AppArmor disabled by boot time parameter
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.475478] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.486662] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.492222] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.494395] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.496548] Initializing cgroup subsys io
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.497395] Initializing cgroup subsys memory
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.498236] Initializing cgroup subsys devices
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.499958] Initializing cgroup subsys freezer
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.501048] Initializing cgroup subsys net_cls
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.502275] Initializing cgroup subsys perf_event
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.503469] Initializing cgroup subsys net_prio
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.504971] Initializing cgroup subsys hugetlb
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.505811] Initializing cgroup subsys pids
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.507108] CPU: Physical Processor ID: 0
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.507814] CPU: Processor Core ID: 0
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.510090] mce: CPU supports 32 MCE banks
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.511363] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.512493] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.517553] Freeing SMP alternatives memory: 32K
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.528913] ftrace: allocating 32185 entries in 126 pages
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.588119] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.591328] smpboot: Max logical packages: 2
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.593605] x2apic enabled
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.596226] Switched APIC routing to physical x2apic.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.601218] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.708052] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.711363] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.716592] x86: Booting SMP configuration:
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.717780] .... node  #0, CPUs:      #1
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.720049] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.726162]  #2
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.727261] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.733088]  #3
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.733792] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.739564] x86: Booted up 1 node, 4 CPUs
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.740907] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.745631] devtmpfs: initialized
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.751322] evm: security.selinux
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.753113] evm: security.SMACK64
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.755054] evm: security.SMACK64EXEC
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.757147] evm: security.SMACK64TRANSMUTE
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.758568] evm: security.SMACK64MMAP
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.760143] evm: security.ima
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.761662] evm: security.capability
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.763785] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.769242] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.772190] pinctrl core: initialized pinctrl subsystem
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.774917] RTC time: 23:20:11, date: 08/15/18
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.777546] NET: Registered protocol family 16
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.788141] cpuidle: using governor ladder
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.800141] cpuidle: using governor menu
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.801871] PCCT header not found.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.803547] ACPI: bus type PCI registered
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.805413] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.808400] PCI: Using configuration type 1 for base access
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.826258] ACPI: Added _OSI(Module Device)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.828406] ACPI: Added _OSI(Processor Device)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.831241] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.834228] ACPI: Added _OSI(Processor Aggregator Device)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.839451] ACPI: Executed 2 blocks of module-level executable AML code
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.866166] ACPI: Interpreter enabled
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.868305] ACPI: (supports S0 S3 S4 S5)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.869712] ACPI: Using IOAPIC for interrupt routing
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.871957] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.907822] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.911459] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.915244] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.919281] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.924897] PCI host bridge to bus 0000:00
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.927114] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.930344] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.933323] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.937703] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.941711] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.944158] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.944652] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.970909] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    0.999383] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.003436] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.014822] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.023092] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.046222] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.057171] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.065799] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.090197] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.095606] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.101722] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.106719] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.112280] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.136226] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.139705] vgaarb: loaded
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.141468] SCSI subsystem initialized
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.143785] libata version 3.00 loaded.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.143816] ACPI: bus type USB registered
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.145875] usbcore: registered new interface driver usbfs
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.147787] usbcore: registered new interface driver hub
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.150250] usbcore: registered new device driver usb
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.152775] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.156453] dmi: Firmware registration failed.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.158445] PCI: Using ACPI for IRQ routing
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.159851] PCI: pci_cache_line_size set to 64 bytes
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.159962] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.159964] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.160123] NetLabel: Initializing
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.161476] NetLabel:  domain hash size = 128
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.162946] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.165184] NetLabel:  unlabeled traffic allowed by default
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.167607] amd_nb: Cannot enumerate AMD northbridges
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.169735] clocksource: Switched to clocksource kvm-clock
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.180551] pnp: PnP ACPI init
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.182009] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.182083] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.182126] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.182222] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.182264] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.182322] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.182363] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.182538] pnp: PnP ACPI: found 7 devices
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.191637] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.195435] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.195438] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.195440] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.195441] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.195491] NET: Registered protocol family 2
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.197939] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.201462] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.204329] TCP: Hash tables configured (established 131072 bind 65536)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.206760] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.208790] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.212976] NET: Registered protocol family 1
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.215234] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.217635] PCI: CLS 0 bytes, default 64
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    1.217721] Unpacking initramfs...
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.467244] Freeing initrd memory: 21432K
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.468927] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.471367] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.475393] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.479668] hw unit of domain pp0-core 2^-0 Joules
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.482879] hw unit of domain package 2^-0 Joules
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.485273] hw unit of domain dram 2^-0 Joules
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.487079] Scanning for low memory corruption every 60 seconds
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.490430] audit: initializing netlink subsys (disabled)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.492416] audit: type=2000 audit(1534375213.694:1): initialized
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.495240] Initialise system trusted keyring
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.497963] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.500569] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.504235] zbud: loaded
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.506456] VFS: Disk quotas dquot_6.6.0
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.508438] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.513665] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.516984] fuse init (API version 7.23)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.518730] Key type big_key registered
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.519990] Allocating IMA MOK and blacklist keyrings.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.526559] Key type asymmetric registered
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.527976] Asymmetric key parser 'x509' registered
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.530155] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.534953] io scheduler noop registered
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.537013] io scheduler deadline registered (default)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.539471] io scheduler cfq registered
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.542431] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.545086] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.548686] intel_idle: does not run on family 6 model 62
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.548802] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.551563] ACPI: Power Button [PWRF]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.553282] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.556535] ACPI: Sleep Button [SLPF]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.559066] GHES: HEST is not enabled!
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.564507] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.567243] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.578142] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.580829] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.593297] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.617840] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.661473] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.688258] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.716009] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.723228] Linux agpgart interface v0.103
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.729234] loop: module loaded
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.730655] libphy: Fixed MDIO Bus: probed
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.732423] tun: Universal TUN/TAP device driver, 1.6
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.734559] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.800897] PPP generic driver version 2.4.2
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.803075] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.806015] ehci-pci: EHCI PCI platform driver
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.807646] ehci-platform: EHCI generic platform driver
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.810409] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.812694] ohci-pci: OHCI PCI platform driver
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.814433] ohci-platform: OHCI generic platform driver
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.816154] uhci_hcd: USB Universal Host Controller Interface driver
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.818287] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.821069] i8042: Warning: Keylock active
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.823871] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.825821] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.827736] mousedev: PS/2 mouse device common for all mice
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.830356] rtc_cmos 00:00: RTC can wake from S4
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.832961] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.835199] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.837495] i2c /dev entries driver
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.838773] device-mapper: uevent: version 1.0.3
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.840736] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.843858] ledtrig-cpu: registered to indicate activity on CPUs
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.847455] NET: Registered protocol family 10
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.850086] NET: Registered protocol family 17
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.851789] Key type dns_resolver registered
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.853950] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.856552] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.860088] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.862442] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.864551] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.868472] registered taskstats version 1
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.870185] Loading compiled-in X.509 certificates
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.873145] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.877308] zswap: loaded using pool lzo/zbud
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.881718] Key type trusted registered
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.888886] Key type encrypted registered
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.890330] ima: No TPM chip found, activating TPM-bypass!
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.892429] evm: HMAC attrs: 0x1
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.894361]   Magic number: 14:850:352
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.895499] acpi LNXCPU:c0: hash matches
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.897366] rtc_cmos 00:00: setting system clock to 2018-08-15 23:20:14 UTC (1534375214)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.900539] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.902702] EDD information not available.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.904233] PM: Hibernation image not present or could not be loaded.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.906164] Freeing unused kernel memory: 1496K
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.907928] Write protecting the kernel read-only data: 14336k
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.911962] Freeing unused kernel memory: 1956K
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.914314] Freeing unused kernel memory: 92K
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    3.935352] systemd-udevd[120]: starting version 204
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    4.027184] scsi host0: Virtio SCSI HBA
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    4.034987] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    4.042213] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    4.045344] AVX version of gcm_enc/dec engaged.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    4.047119] AES CTR mode by8 optimization enabled
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    4.140526] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    4.143687] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    4.147338] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    4.150618] sd 0:0:1:0: [sda] Write Protect is off
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    4.152336] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    4.152556] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    4.158750]  sda: sda1
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    4.162032] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    4.485914] tsc: Refined TSC clocksource calibration: 2499.770 MHz
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    4.488563] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x240860798ba, max_idle_ns: 440795260630 ns
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    4.888642] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    7.005964] floppy0: no floppy controllers found
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.185768] raid6: sse2x1   gen()  9003 MB/s
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.253769] raid6: sse2x1   xor()  6972 MB/s
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.321769] raid6: sse2x2   gen() 11216 MB/s
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.389793] raid6: sse2x2   xor()  7873 MB/s
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.457770] raid6: sse2x4   gen() 12535 MB/s
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.525759] raid6: sse2x4   xor()  8325 MB/s
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.526484] raid6: using algorithm sse2x4 gen() 12535 MB/s
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.527337] raid6: .... xor() 8325 MB/s, rmw enabled
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.528100] raid6: using ssse3x2 recovery algorithm
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.530450] xor: automatically using best checksumming function:
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.569770]    avx       : 22159.000 MB/sec
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.584808] Btrfs loaded
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.630416] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.632010] EXT4-fs (sda1): write access will be enabled during recovery
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.697112] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.709802] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.710900] EXT4-fs (sda1): recovery complete
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.717708] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    8.915330] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    9.027513] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    9.078595] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    9.283407] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [    9.899080] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   10.064321] systemd-udevd[703]: starting version 204
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   10.160209] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   10.278918] ppdev: user-space parallel port driver
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   10.377458] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   10.434411] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   10.505527] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   10.672601] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   11.004013] random: mktemp: uninitialized urandom read (12 bytes read, 58 bits of entropy available)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   11.091543] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   11.186832] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   11.234612] EXT4-fs (sda1): resized filesystem to 7864064
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   11.669055] init: failsafe main process (1095) killed by TERM signal
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO Running set_multiqueue.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO Set channels for eth0 to 4.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 15 23:20:22 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Starting Google Accounts daemon.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Creating a new user account for me.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Created user account me.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Creating a new user account for henrik.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Created user account henrik.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Creating a new user account for emma.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Created user account emma.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Creating a new user account for igor.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Created user account igor.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Created user account konstantinhaase.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Creating a new user account for aj.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   12.963543] random: nonblocking pool is initialized
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Created user account aj.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Creating a new user account for solarce.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Created user account solarce.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Creating a new user account for asari.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Created user account asari.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   13.153026] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   13.157992] Bridge firewalling registered
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Creating a new user account for bogdana.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   13.172455] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   13.226454] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   13.229927] floppy0: no floppy controllers found
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   13.230297] work still pending
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Created user account bogdana.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Creating a new user account for konstantin.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Created user account konstantin.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   13.358414] Initializing XFRM netlink socket
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Creating a new user account for carmen.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   13.375890] Netfilter messages via NETLINK v0.30.
Aug 15 23:20:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   13.383563] ctnetlink v0.93: registering with nfnetlink.
Aug 15 23:20:24 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 23:20:24 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 23:20:24 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Created user account carmen.
Aug 15 23:20:24 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-clock-skew: INFO Synced system time with hardware clock.
Aug 15 23:20:24 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Creating a new user account for maria.
Aug 15 23:20:24 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Created user account maria.
Aug 15 23:20:24 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 google-accounts: INFO Removing user packer.
Aug 15 23:20:24 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 cron[1709]: (CRON) INFO (pidfile fd = 3)
Aug 15 23:20:24 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 23:20:24 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 23:20:24 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 cron[1739]: (CRON) STARTUP (fork ok)
Aug 15 23:20:24 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 cron[1739]: (CRON) INFO (Running @reboot jobs)
Aug 15 23:20:24 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 acpid: starting up with netlink and the input layer
Aug 15 23:20:24 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 acpid: 1 rule loaded
Aug 15 23:20:24 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 acpid: waiting for events: event logging is off
Aug 15 23:20:25 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 haveged: haveged starting up
Aug 15 23:20:25 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   14.735371] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpd[1849]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpd[1850]: proto: precision = 0.103 usec
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpd[1850]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpd[1850]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpd[1850]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpd[1850]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpd[1850]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpd[1850]: Listen normally on 3 eth0 10.20.2.84 UDP 123
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpd[1850]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpd[1850]: peers refreshed
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpd[1850]: Listening on routing socket on fd #21 for interface updates
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   19.953699] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 startup-script: INFO Found startup-script in metadata.
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 startup-script: INFO startup-script: job 1 at Thu Aug 16 02:30:00 2018
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 startup-script: INFO startup-script: Return code 0.
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 startup-script: INFO startup-script: Return code 0.
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 startup-script: INFO Finished running startup scripts.
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ec2: 
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ec2: #############################################################
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ec2: 1024 84:91:45:35:50:af:85:97:fa:21:b0:9e:76:34:0c:ee  root@travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 (DSA)
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ec2: 256 31:12:f6:e8:8f:06:6f:c6:eb:d1:0c:70:ce:7f:5d:3d  root@travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 (ECDSA)
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ec2: 256 7c:d8:cc:80:2e:26:bf:0b:83:3c:c4:d8:8d:63:20:f6  root@travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 (ED25519)
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ec2: 2048 00:6e:dd:d0:46:79:1c:3a:da:78:a5:50:52:8d:8f:60  root@travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 (RSA)
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 15 23:20:30 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ec2: #############################################################
Aug 15 23:20:38 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpdate[2248]: the NTP socket is in use, exiting
Aug 15 23:21:20 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [   70.042489] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 15 23:23:08 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [  177.893220] device vetha12c4cb entered promiscuous mode
Aug 15 23:23:08 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [  177.994752] cgroup: docker-runc (4942) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 15 23:23:08 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [  177.994755] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 15 23:23:08 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [  178.066433] eth0: renamed from vethf56c4d4
Aug 15 23:23:08 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [  178.106334] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 15 23:23:08 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [  178.107807] docker0: port 1(vetha12c4cb) entered forwarding state
Aug 15 23:23:08 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [  178.107833] docker0: port 1(vetha12c4cb) entered forwarding state
Aug 15 23:23:08 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [  178.107855] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 15 23:23:11 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpd[1850]: Listen normally on 5 docker0 fe80::42:22ff:fe35:6fd3 UDP 123
Aug 15 23:23:11 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpd[1850]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 15 23:23:11 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpd[1850]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 15 23:23:11 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpd[1850]: peers refreshed
Aug 15 23:23:11 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 ntpd[1850]: new interface(s) found: waking up resolver
Aug 15 23:23:23 travis-job-bb3f76d7-4c66-403c-8933-cd41bf6a66d4 kernel: [  193.129487] docker0: port 1(vetha12c4cb) entered forwarding state
---
travis_time:end:1a4ebfa2:start=1534375449844912956,finish=1534375449852140592,duration=7227636
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:220f35fd
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1f219f1c
travis_time:start:1f219f1c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:05359c8e
$ dmesg | grep -i kill
