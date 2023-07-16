plain
[00:04:41]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:04:42] error: expected item after doc comment
[00:04:42]   --> libstd/keyword_docs.rs:55:1
[00:04:42]    |
[00:04:42] 55 | /// [book]: https://doc.rust-lang.org/book/second-edition/ch03-01-variables-and-mutability.html
[00:04:42] 
[00:04:42] 
[00:04:42] error: expected item, found `<eof>`
[00:04:42]   --> libstd/keyword_docs.rs:55:1
[00:04:42]    |
[00:04:42] 55 | /// [book]: https://doc.rust-lang.org/book/second-edition/ch03-01-variables-and-mutability.html
[00:04:42] 
[00:04:42] error: aborting due to 2 previous errors
[00:04:42] 
[00:04:43] error: Could not compile `std`.
[00:04:43] error: Could not compile `std`.
[00:04:43] 
[00:04:43] Caused by:
[00:04:43]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg 'feature="alloc_jemalloc"' --cfg 'feature="backtrace"' --cfg 'feature="jemalloc"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' -C metadata=0556150f5905f74e -C extra-filename=-0556150f5905f74e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-269c5b575a825095.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-5d65c96a58e63a55.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-1e00a6aabc9eec07.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-c1b9aa9d5447cc92.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-bd44783aadae9ca1.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-26675515f179c1b9.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-2c3eace469f066b1.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-d613d4c27f372055.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-48c6d4917235fa2b.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-59bc1b2707f17ab5.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-35e21386b0dc3419.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-2702d1393bd48ed5.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-b43d6c2dac6b10e3.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/ -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace -l static=backtrace -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-ed6515dfcadbf7bb/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 1)
[00:04:43] expected success, got: exit code: 101
[00:04:43] expected success, got: exit code: 101
[00:04:43] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:04:43] travis_fold:end:stage0-std

[00:04:43] travis_time:end:stage0-std:start=1533843771480345737,finish=1533843806827006289,duration=35346660552


[00:04:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:43] Build completed unsuccessfully in 0:00:36
[00:04:43] Makefile:28: recipe for target 'all' failed
[00:04:43] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:226ae970
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:077bfd46
$ sudo tail -n 500 /var/log/syslog
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] kvm-clock: using sched offset of 1425577853 cycles
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] Zone ranges:
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000]   Device   empty
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] Movable zone start for each node
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] Early memory node ranges
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] Policy zone: Normal
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] Hierarchical RCU implementation.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] console [ttyS0] enabled
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.302577] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.303717] pid_max: default: 32768 minimum: 301
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.304441] ACPI: Core revision 20150930
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.310717] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.311728] Security Framework initialized
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.312292] Yama: becoming mindful.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.312793] AppArmor: AppArmor disabled by boot time parameter
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.315189] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.324094] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.328213] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.329272] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.330621] Initializing cgroup subsys io
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.331228] Initializing cgroup subsys memory
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.331855] Initializing cgroup subsys devices
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.332467] Initializing cgroup subsys freezer
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.333160] Initializing cgroup subsys net_cls
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.333852] Initializing cgroup subsys perf_event
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.334568] Initializing cgroup subsys net_prio
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.335235] Initializing cgroup subsys hugetlb
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.335857] Initializing cgroup subsys pids
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.336538] CPU: Physical Processor ID: 0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.337148] CPU: Processor Core ID: 0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.338472] mce: CPU supports 32 MCE banks
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.339249] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.340169] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.343428] Freeing SMP alternatives memory: 32K
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.353098] ftrace: allocating 32185 entries in 126 pages
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.406203] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.407227] smpboot: Max logical packages: 2
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.408352] x2apic enabled
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.410040] Switched APIC routing to physical x2apic.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.413449] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.521882] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.523656] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.525993] x86: Booting SMP configuration:
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.526699] .... node  #0, CPUs:      #1
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.527469] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.531583]  #2
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.532010] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.536266]  #3
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.536797] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.540844] x86: Booted up 1 node, 4 CPUs
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.541524] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.544168] devtmpfs: initialized
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.548524] evm: security.selinux
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.549143] evm: security.SMACK64
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.549666] evm: security.SMACK64EXEC
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.550211] evm: security.SMACK64TRANSMUTE
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.550956] evm: security.SMACK64MMAP
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.551487] evm: security.ima
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.551924] evm: security.capability
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.552791] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.554316] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.555474] pinctrl core: initialized pinctrl subsystem
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.556619] RTC time: 19:36:41, date: 08/09/18
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.558183] NET: Registered protocol family 16
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.569932] cpuidle: using governor ladder
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.581925] cpuidle: using governor menu
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.582811] PCCT header not found.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.583801] ACPI: bus type PCI registered
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.584536] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.585618] PCI: Using configuration type 1 for base access
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.598847] ACPI: Added _OSI(Module Device)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.599534] ACPI: Added _OSI(Processor Device)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.600310] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.601256] ACPI: Added _OSI(Processor Aggregator Device)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.604695] ACPI: Executed 2 blocks of module-level executable AML code
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.627509] ACPI: Interpreter enabled
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.628236] ACPI: (supports S0 S3 S4 S5)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.628985] ACPI: Using IOAPIC for interrupt routing
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.629750] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.659103] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.660360] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.661419] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.662434] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.664868] PCI host bridge to bus 0000:00
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.665528] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.666671] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.667807] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.668882] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.669991] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.670907] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.671339] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.686691] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.701877] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.704325] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.712929] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.716963] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.729619] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.734989] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.739030] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.752238] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.754515] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.756983] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.759488] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.761707] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.782023] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.783348] vgaarb: loaded
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.784080] SCSI subsystem initialized
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.785026] libata version 3.00 loaded.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.785052] ACPI: bus type USB registered
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.785923] usbcore: registered new interface driver usbfs
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.786842] usbcore: registered new interface driver hub
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.788011] usbcore: registered new device driver usb
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.789204] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.790419] dmi: Firmware registration failed.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.791236] PCI: Using ACPI for IRQ routing
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.792427] PCI: pci_cache_line_size set to 64 bytes
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.792524] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.792526] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.792643] NetLabel: Initializing
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.793418] NetLabel:  domain hash size = 128
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.794346] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.796130] NetLabel:  unlabeled traffic allowed by default
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.797236] amd_nb: Cannot enumerate AMD northbridges
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.798414] clocksource: Switched to clocksource kvm-clock
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.805819] pnp: PnP ACPI init
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.806482] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.806552] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.806598] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.806648] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.806690] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.806732] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.806773] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.806928] pnp: PnP ACPI: found 7 devices
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.814288] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.815627] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.815629] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.815631] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.815632] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.815666] NET: Registered protocol family 2
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.816526] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.818606] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.820085] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.821123] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.822470] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.823659] NET: Registered protocol family 1
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.824376] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.825255] PCI: CLS 0 bytes, default 64
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    0.825305] Unpacking initramfs...
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.829310] Freeing initrd memory: 21432K
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.830040] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.831364] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.832808] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.834708] hw unit of domain pp0-core 2^-0 Joules
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.835459] hw unit of domain package 2^-0 Joules
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.836198] hw unit of domain dram 2^-16 Joules
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.836909] Scanning for low memory corruption every 60 seconds
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.838325] audit: initializing netlink subsys (disabled)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.839207] audit: type=2000 audit(1533843403.356:1): initialized
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.840383] Initialise system trusted keyring
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.841296] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.842364] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.844359] zbud: loaded
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.845011] VFS: Disk quotas dquot_6.6.0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.845684] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.847028] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.848386] fuse init (API version 7.23)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.849265] Key type big_key registered
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.849901] Allocating IMA MOK and blacklist keyrings.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.852047] Key type asymmetric registered
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.852793] Asymmetric key parser 'x509' registered
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.853612] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.855021] io scheduler noop registered
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.855743] io scheduler deadline registered (default)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.856734] io scheduler cfq registered
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.857857] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.859290] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.861354] intel_idle: does not run on family 6 model 63
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.861453] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.862724] ACPI: Power Button [PWRF]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.863577] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.864824] ACPI: Sleep Button [SLPF]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.866044] GHES: HEST is not enabled!
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.868423] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.869531] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.875240] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.876990] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.882765] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.905540] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.929285] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.953335] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.977148] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.981282] Linux agpgart interface v0.103
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.984903] loop: module loaded
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.986134] libphy: Fixed MDIO Bus: probed
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.987540] tun: Universal TUN/TAP device driver, 1.6
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    2.989212] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.031659] PPP generic driver version 2.4.2
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.033186] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.035256] ehci-pci: EHCI PCI platform driver
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.036658] ehci-platform: EHCI generic platform driver
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.038503] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.040683] ohci-pci: OHCI PCI platform driver
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.042391] ohci-platform: OHCI generic platform driver
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.044039] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.046193] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.049677] i8042: Warning: Keylock active
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.051950] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.053748] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.055896] mousedev: PS/2 mouse device common for all mice
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.058040] rtc_cmos 00:00: RTC can wake from S4
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.059858] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.061983] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.063664] i2c /dev entries driver
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.064925] device-mapper: uevent: version 1.0.3
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.066576] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.069338] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.072358] NET: Registered protocol family 10
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.073916] NET: Registered protocol family 17
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.075340] Key type dns_resolver registered
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.076916] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.078591] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.080121] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.081738] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.083217] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.085823] registered taskstats version 1
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.086905] Loading compiled-in X.509 certificates
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.089967] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.092945] zswap: loaded using pool lzo/zbud
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.096174] Key type trusted registered
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.100310] Key type encrypted registered
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.101457] ima: No TPM chip found, activating TPM-bypass!
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.102999] evm: HMAC attrs: 0x1
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.104187]   Magic number: 14:285:647
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.105560] acpi LNXCPU:00: hash matches
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.107069] rtc_cmos 00:00: setting system clock to 2018-08-09 19:36:43 UTC (1533843403)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.109924] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.111483] EDD information not available.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.112707] PM: Hibernation image not present or could not be loaded.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.114072] Freeing unused kernel memory: 1496K
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.115136] Write protecting the kernel read-only data: 14336k
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.116847] Freeing unused kernel memory: 1956K
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.117881] Freeing unused kernel memory: 92K
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.130907] systemd-udevd[119]: starting version 204
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.196675] scsi host0: Virtio SCSI HBA
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.204664] AVX2 version of gcm_enc/dec engaged.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.205749] AES CTR mode by8 optimization enabled
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.206663] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.238850] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.238854] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.240945] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.242021] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.242872] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.243016] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.245564]  sda: sda1
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.246819] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.254715] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.834569] tsc: Refined TSC clocksource calibration: 2300.000 MHz
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    3.835622] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735223b2, max_idle_ns: 440795277976 ns
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    4.087624] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    6.178602] floppy0: no floppy controllers found
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    7.338453] raid6: sse2x1   gen()  8826 MB/s
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    7.406448] raid6: sse2x1   xor()  6894 MB/s
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    7.474446] raid6: sse2x2   gen() 11250 MB/s
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    7.542440] raid6: sse2x2   xor()  7494 MB/s
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    7.610446] raid6: sse2x4   gen() 12953 MB/s
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    7.678441] raid6: sse2x4   xor()  9071 MB/s
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    7.746451] raid6: avx2x1   gen() 17110 MB/s
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    7.814446] raid6: avx2x2   gen() 20170 MB/s
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    7.882444] raid6: avx2x4   gen() 22913 MB/s
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    7.883668] raid6: using algorithm avx2x4 gen() 22913 MB/s
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    7.884713] raid6: using avx2x2 recovery algorithm
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    7.886895] xor: automatically using best checksumming function:
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    7.926439]    avx       : 27401.000 MB/sec
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    7.940358] Btrfs loaded
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    7.981635] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    7.983082] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    8.057841] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    8.065408] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    8.066208] EXT4-fs (sda1): recovery complete
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    8.070993] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    8.267697] random: init: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    8.393911] random: mountall: uninitialized urandom read (12 bytes read, 32 bits of entropy available)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    8.441170] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    8.639403] random: cloud-init: uninitialized urandom read (32 bytes read, 40 bits of entropy available)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    9.173082] random: cloud-init: uninitialized urandom read (32 bytes read, 49 bits of entropy available)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    9.313677] systemd-udevd[701]: starting version 204
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    9.446263] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    9.512684] intel_rapl: no valid rapl domains found in package 0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    9.555148] intel_rapl: no valid rapl domains found in package 0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    9.574903] ppdev: user-space parallel port driver
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    9.657894] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    9.708417] random: mktemp: uninitialized urandom read (6 bytes read, 62 bits of entropy available)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    9.769495] random: cloud-init: uninitialized urandom read (32 bytes read, 62 bits of entropy available)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [    9.934995] random: cloud-init: uninitialized urandom read (32 bytes read, 62 bits of entropy available)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [   10.226005] random: mktemp: uninitialized urandom read (12 bytes read, 65 bits of entropy available)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [   10.291622] random: mktemp: uninitialized urandom read (6 bytes read, 66 bits of entropy available)
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [   10.360068] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [   10.402718] EXT4-fs (sda1): resized filesystem to 7864064
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [   10.735277] init: failsafe main process (1093) killed by TERM signal
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO Running set_multiqueue.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO Set channels for eth0 to 4.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [   11.507616] random: nonblocking pool is initialized
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Starting Google Accounts daemon.
Aug  9 19:36:51 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Creating a new user account for me.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Created user account me.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Creating a new user account for henrik.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Created user account henrik.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Creating a new user account for emma.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Created user account emma.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Creating a new user account for igor.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d cron[1392]: (CRON) INFO (pidfile fd = 3)
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Created user account igor.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d cron[1471]: (CRON) STARTUP (fork ok)
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d cron[1471]: (CRON) INFO (Running @reboot jobs)
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d acpid: starting up with netlink and the input layer
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d acpid: 1 rule loaded
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d acpid: waiting for events: event logging is off
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Created user account konstantinhaase.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Creating a new user account for aj.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Created user account aj.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d haveged: haveged starting up
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Creating a new user account for solarce.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [   12.058459] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Created user account solarce.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [   12.070753] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Creating a new user account for asari.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Created user account asari.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Creating a new user account for bogdana.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Created user account bogdana.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Creating a new user account for konstantin.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Created user account konstantin.
Aug  9 19:36:52 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Creating a new user account for carmen.
Aug  9 19:36:53 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 19:36:53 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Created user account carmen.
Aug  9 19:36:53 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Creating a new user account for maria.
Aug  9 19:36:53 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [   12.290768] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  9 19:36:53 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [   12.293806] Bridge firewalling registered
Aug  9 19:36:53 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [   12.303922] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 19:36:53 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Created user account maria.
Aug  9 19:36:53 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d google-accounts: INFO Removing user packer.
Aug  9 19:36:53 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [   12.372170] Initializing XFRM netlink socket
Aug  9 19:36:53 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [   12.379113] Netfilter messages via NETLINK v0.30.
Aug  9 19:36:53 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [   12.381641] ctnetlink v0.93: registering with nfnetlink.
Aug  9 19:36:53 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [   12.506484] floppy0: no floppy controllers found
Aug  9 19:37:16 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpdate[1862]: adjust time server 169.254.169.254 offset 0.002123 sec
Aug  9 19:37:22 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpd[1897]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 19:37:22 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpd[1898]: proto: precision = 0.135 usec
Aug  9 19:37:22 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpd[1898]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 19:37:22 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpd[1898]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 19:37:22 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpd[1898]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 19:37:22 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpd[1898]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 19:37:22 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpd[1898]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 19:37:22 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpd[1898]: Listen normally on 3 eth0 10.20.1.250 UDP 123
Aug  9 19:37:22 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpd[1898]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 19:37:22 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpd[1898]: peers refreshed
Aug  9 19:37:22 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpd[1898]: Listening on routing socket on fd #21 for interface updates
Aug  9 19:37:23 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [   42.255488] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 19:37:23 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d startup-script: INFO Found startup-script in metadata.
Aug  9 19:37:23 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  9 19:37:23 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d startup-script: INFO startup-script: job 1 at Thu Aug  9 22:47:00 2018
Aug  9 19:37:23 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d startup-script: INFO startup-script: Return code 0.
Aug  9 19:37:23 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d startup-script: INFO startup-script: Return code 0.
Aug  9 19:37:23 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d startup-script: INFO Finished running startup scripts.
Aug  9 19:37:23 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ec2: 
Aug  9 19:37:23 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ec2: #############################################################
Aug  9 19:37:23 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 19:37:23 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ec2: 1024 cd:50:b2:d5:6e:16:d9:13:a2:88:05:4b:0e:da:98:f4  root@travis-job-354b207f-9eb9-484b-9776-f4cd1644055d (DSA)
Aug  9 19:37:23 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ec2: 256 11:46:1b:43:aa:00:6d:0d:a4:be:c6:cf:ca:67:fc:db  root@travis-job-354b207f-9eb9-484b-9776-f4cd1644055d (ECDSA)
Aug  9 19:37:23 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ec2: 256 71:36:18:cb:65:36:5b:86:04:60:59:53:1f:70:6d:35  root@travis-job-354b207f-9eb9-484b-9776-f4cd1644055d (ED25519)
Aug  9 19:37:23 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ec2: 2048 bc:1c:e2:07:41:8c:26:8a:03:f8:1b:b5:74:60:ce:1e  root@travis-job-354b207f-9eb9-484b-9776-f4cd1644055d (RSA)
Aug  9 19:37:23 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 19:37:23 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ec2: #############################################################
Aug  9 19:38:42 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [  122.214573] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 19:39:39 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [  178.853733] device veth8098b2e entered promiscuous mode
Aug  9 19:39:39 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [  178.853800] docker0: port 1(veth8098b2e) entered forwarding state
Aug  9 19:39:39 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [  178.853808] docker0: port 1(veth8098b2e) entered forwarding state
Aug  9 19:39:39 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [  178.854140] docker0: port 1(veth8098b2e) entered disabled state
Aug  9 19:39:39 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [  178.944858] cgroup: docker-runc (4889) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 19:39:39 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [  178.944861] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  9 19:39:39 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [  179.012501] eth0: renamed from vethc8a43b6
Aug  9 19:39:39 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [  179.044621] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  9 19:39:39 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [  179.045925] docker0: port 1(veth8098b2e) entered forwarding state
Aug  9 19:39:39 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [  179.045939] docker0: port 1(veth8098b2e) entered forwarding state
Aug  9 19:39:39 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [  179.045963] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  9 19:39:42 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpd[1898]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  9 19:39:42 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpd[1898]: Listen normally on 6 docker0 fe80::42:53ff:fe3a:6492 UDP 123
Aug  9 19:39:42 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpd[1898]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  9 19:39:42 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpd[1898]: peers refreshed
Aug  9 19:39:42 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d ntpd[1898]: new interface(s) found: waking up resolver
Aug  9 19:39:54 travis-job-354b207f-9eb9-484b-9776-f4cd1644055d kernel: [  194.072316] docker0: port 1(veth8098b2e) entered forwarding state
---
travis_time:end:3610119e:start=1533843807379547978,finish=1533843807387281761,duration=7733783
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1563beac
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:032d02f7
travis_time:start:032d02f7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:011aeab4
$ dmesg | grep -i kill
