plain
[00:47:25] ....................................................................................................
[00:47:28] ....................................................................................................
[00:47:30] ....................................................................................................
[00:47:34] ....................................................................................................
[00:47:37] ................iiiiiiiii...........................................................................
[00:47:43] ....................................................................................................
[00:47:46] ......................i.............................................................................
[00:47:49] .................................i..................................................................
[00:47:52] ....................................................................................................
---
[00:49:13] ....................................................................................................
[00:49:22] ....................................................................................................
[00:49:35] ....................................................................................................
[00:49:44] ....................................................................................................
[00:49:53] ........F...........................................................................................
[00:50:08] ....................................................................................................
[00:50:16] ....................................................................................................
[00:50:28] ..........................................................................i.........................
[00:50:38] ....................................................................................................
---
[00:53:45] ---- [run-pass] run-pass/issue-13494.rs stdout ----
[00:53:45] 
[00:53:45] error: compilation failed!
[00:53:45] status: exit code: 1
[00:53:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issue-13494.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-13494/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-13494/auxiliary"
[00:53:45] ------------------------------------------
[00:53:45] 
[00:53:45] ------------------------------------------
[00:53:45] stderr:
[00:53:45] stderr:
[00:53:45] ------------------------------------------
[00:53:45] error[E0433]: failed to resolve. Maybe a missing `extern crate core;`?
[00:53:45]    |
[00:53:45] 35 | /         select! {
[00:53:45] 35 | /         select! {
[00:53:45] 36 | |             _ = rx2.recv() => (),
[00:53:45] 37 | |             _ = rcv.recv() => ()
[00:53:45] 38 | |         }
[00:53:45]    | |_________^ Maybe a missing `extern crate core;`?
[00:53:45]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:53:45] 
[00:53:45] error: aborting due to previous error
[00:53:45] 
---
[00:53:45] 
[00:53:45] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:53:45] 
[00:53:45] 
[00:53:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:53:45] 
[00:53:45] 
[00:53:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:53:45] Build completed unsuccessfully in 0:09:03
[00:53:45] Build completed unsuccessfully in 0:09:03
[00:53:45] Makefile:58: recipe for target 'check' failed
[00:53:45] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1bfb3ff4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:20e0a310
$ sudo tail -n 500 /var/log/syslog
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Using GB pages for direct mapping
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] kvm-clock: using sched offset of 1754848404 cycles
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Zone ranges:
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000]   Device   empty
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Movable zone start for each node
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Early memory node ranges
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Policy zone: Normal
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] console [ttyS0] enabled
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.557123] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.561651] pid_max: default: 32768 minimum: 301
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.564634] ACPI: Core revision 20150930
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.572340] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.574428] Security Framework initialized
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.576141] Yama: becoming mindful.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.577159] AppArmor: AppArmor disabled by boot time parameter
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.581066] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.593386] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.600295] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.603270] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.606011] Initializing cgroup subsys io
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.608060] Initializing cgroup subsys memory
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.609758] Initializing cgroup subsys devices
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.611748] Initializing cgroup subsys freezer
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.613225] Initializing cgroup subsys net_cls
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.614568] Initializing cgroup subsys perf_event
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.616281] Initializing cgroup subsys net_prio
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.617925] Initializing cgroup subsys hugetlb
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.619888] Initializing cgroup subsys pids
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.621326] CPU: Physical Processor ID: 0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.622874] CPU: Processor Core ID: 0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.624505] mce: CPU supports 32 MCE banks
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.626155] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.628424] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.632986] Freeing SMP alternatives memory: 32K
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.645143] ftrace: allocating 32185 entries in 126 pages
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.706119] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.709254] smpboot: Max logical packages: 2
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.711682] x2apic enabled
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.714191] Switched APIC routing to physical x2apic.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.719332] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.826842] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.831779] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.840042] x86: Booting SMP configuration:
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.842934] .... node  #0, CPUs:      #1
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.845060] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.850213]  #2
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.851323] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.856501]  #3
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.857499] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.862813] x86: Booted up 1 node, 4 CPUs
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.865664] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.870070] devtmpfs: initialized
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.876182] evm: security.selinux
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.877553] evm: security.SMACK64
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.879478] evm: security.SMACK64EXEC
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.881508] evm: security.SMACK64TRANSMUTE
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.883768] evm: security.SMACK64MMAP
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.885306] evm: security.ima
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.886899] evm: security.capability
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.888868] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.893652] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.896368] pinctrl core: initialized pinctrl subsystem
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.898319] RTC time: 13:14:16, date: 08/13/18
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.901457] NET: Registered protocol family 16
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.914960] cpuidle: using governor ladder
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.927066] cpuidle: using governor menu
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.929659] PCCT header not found.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.931214] ACPI: bus type PCI registered
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.932645] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.936080] PCI: Using configuration type 1 for base access
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.952775] ACPI: Added _OSI(Module Device)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.954993] ACPI: Added _OSI(Processor Device)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.956586] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.959126] ACPI: Added _OSI(Processor Aggregator Device)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.965427] ACPI: Executed 2 blocks of module-level executable AML code
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.994402] ACPI: Interpreter enabled
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.996889] ACPI: (supports S0 S3 S4 S5)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    0.999265] ACPI: Using IOAPIC for interrupt routing
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.002325] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.035849] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.039141] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.042388] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.044875] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.052255] PCI host bridge to bus 0000:00
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.054571] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.058081] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.062672] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.065994] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.070079] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.072405] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.072930] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.102022] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.129591] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.133089] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.145111] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.153285] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.177976] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.189798] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.200704] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.226131] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.231696] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.238835] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.243946] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.248343] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.271744] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.274756] vgaarb: loaded
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.276025] SCSI subsystem initialized
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.277600] libata version 3.00 loaded.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.277630] ACPI: bus type USB registered
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.279120] usbcore: registered new interface driver usbfs
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.282295] usbcore: registered new interface driver hub
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.284798] usbcore: registered new device driver usb
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.287260] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.291095] dmi: Firmware registration failed.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.292955] PCI: Using ACPI for IRQ routing
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.294291] PCI: pci_cache_line_size set to 64 bytes
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.294426] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.294430] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.294606] NetLabel: Initializing
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.295966] NetLabel:  domain hash size = 128
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.297624] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.299753] NetLabel:  unlabeled traffic allowed by default
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.303590] amd_nb: Cannot enumerate AMD northbridges
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.306060] clocksource: Switched to clocksource kvm-clock
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.317789] pnp: PnP ACPI init
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.319418] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.319535] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.319582] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.319634] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.319677] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.319718] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.319760] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.319943] pnp: PnP ACPI: found 7 devices
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.331100] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.336038] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.336042] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.336044] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.336045] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.336096] NET: Registered protocol family 2
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.337966] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.341732] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.345493] TCP: Hash tables configured (established 131072 bind 65536)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.348951] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.352061] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.355565] NET: Registered protocol family 1
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.356886] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.359211] PCI: CLS 0 bytes, default 64
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    1.359281] Unpacking initramfs...
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.605359] Freeing initrd memory: 21432K
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.607938] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.613367] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.621626] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.625973] hw unit of domain pp0-core 2^-0 Joules
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.627634] hw unit of domain package 2^-0 Joules
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.629445] hw unit of domain dram 2^-0 Joules
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.631400] Scanning for low memory corruption every 60 seconds
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.634404] audit: initializing netlink subsys (disabled)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.636292] audit: type=2000 audit(1534166059.126:1): initialized
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.640236] Initialise system trusted keyring
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.642512] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.644342] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.647933] zbud: loaded
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.649940] VFS: Disk quotas dquot_6.6.0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.651787] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.654377] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.658048] fuse init (API version 7.23)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.660054] Key type big_key registered
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.661986] Allocating IMA MOK and blacklist keyrings.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.667185] Key type asymmetric registered
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.668921] Asymmetric key parser 'x509' registered
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.671594] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.675985] io scheduler noop registered
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.677595] io scheduler deadline registered (default)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.679442] io scheduler cfq registered
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.680882] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.682983] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.685207] intel_idle: does not run on family 6 model 45
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.685363] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.687837] ACPI: Power Button [PWRF]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.689708] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.692538] ACPI: Sleep Button [SLPF]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.695064] GHES: HEST is not enabled!
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.700187] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.702379] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.711881] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.714974] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.728544] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.753613] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.779101] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.805794] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.831236] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.837228] Linux agpgart interface v0.103
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.844965] loop: module loaded
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.846640] libphy: Fixed MDIO Bus: probed
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.847894] tun: Universal TUN/TAP device driver, 1.6
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.850593] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.905232] PPP generic driver version 2.4.2
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.908277] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.910600] ehci-pci: EHCI PCI platform driver
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.912561] ehci-platform: EHCI generic platform driver
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.914223] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.916272] ohci-pci: OHCI PCI platform driver
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.917769] ohci-platform: OHCI generic platform driver
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.920420] uhci_hcd: USB Universal Host Controller Interface driver
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.922643] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.925921] i8042: Warning: Keylock active
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.928630] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.930717] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.933362] mousedev: PS/2 mouse device common for all mice
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.936298] rtc_cmos 00:00: RTC can wake from S4
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.938597] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.942452] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.944984] i2c /dev entries driver
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.948144] device-mapper: uevent: version 1.0.3
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.950425] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.953154] ledtrig-cpu: registered to indicate activity on CPUs
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.956548] NET: Registered protocol family 10
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.959080] NET: Registered protocol family 17
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.960972] Key type dns_resolver registered
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.963485] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.966574] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.968925] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.971171] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.973477] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.978573] registered taskstats version 1
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.980437] Loading compiled-in X.509 certificates
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.983693] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.987540] zswap: loaded using pool lzo/zbud
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.992391] Key type trusted registered
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    3.999413] Key type encrypted registered
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.001326] ima: No TPM chip found, activating TPM-bypass!
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.003434] evm: HMAC attrs: 0x1
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.005615]   Magic number: 14:534:230
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.007466] rtc_cmos 00:00: setting system clock to 2018-08-13 13:14:19 UTC (1534166059)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.010995] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.013685] EDD information not available.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.015507] PM: Hibernation image not present or could not be loaded.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.017460] Freeing unused kernel memory: 1496K
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.019052] Write protecting the kernel read-only data: 14336k
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.022601] Freeing unused kernel memory: 1956K
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.025098] Freeing unused kernel memory: 92K
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.045644] systemd-udevd[119]: starting version 204
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.116420] scsi host0: Virtio SCSI HBA
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.130169] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.133344] AVX version of gcm_enc/dec engaged.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.134889] AES CTR mode by8 optimization enabled
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.138550] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.186881] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.186997] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.186999] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.187376] sd 0:0:1:0: [sda] Write Protect is off
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.187377] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.187517] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.190557]  sda: sda1
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.192049] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.630303] tsc: Refined TSC clocksource calibration: 2599.927 MHz
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.632527] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2579f6ee1bc, max_idle_ns: 440795239464 ns
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    4.976536] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    7.110423] floppy0: no floppy controllers found
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.274102] raid6: sse2x1   gen()  8666 MB/s
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.342186] raid6: sse2x1   xor()  6690 MB/s
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.410117] raid6: sse2x2   gen() 11041 MB/s
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.478155] raid6: sse2x2   xor()  7481 MB/s
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.546180] raid6: sse2x4   gen() 12209 MB/s
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.614269] raid6: sse2x4   xor()  8144 MB/s
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.619649] raid6: using algorithm sse2x4 gen() 12209 MB/s
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.622619] raid6: .... xor() 8144 MB/s, rmw enabled
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.624539] raid6: using ssse3x2 recovery algorithm
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.630173] xor: automatically using best checksumming function:
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.670216]    avx       : 21688.000 MB/sec
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.690483] Btrfs loaded
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.755419] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.758821] EXT4-fs (sda1): write access will be enabled during recovery
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.848028] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.856975] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.859349] EXT4-fs (sda1): recovery complete
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    8.867272] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    9.136624] random: init: uninitialized urandom read (12 bytes read, 23 bits of entropy available)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    9.279013] random: mountall: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    9.343248] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [    9.599278] random: cloud-init: uninitialized urandom read (32 bytes read, 32 bits of entropy available)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   10.265880] random: cloud-init: uninitialized urandom read (32 bytes read, 39 bits of entropy available)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   10.422008] systemd-udevd[702]: starting version 204
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   10.575383] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   10.616708] intel_rapl: no valid rapl domains found in package 0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   10.674252] ppdev: user-space parallel port driver
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   10.829874] random: mktemp: uninitialized urandom read (6 bytes read, 50 bits of entropy available)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   10.893381] random: mktemp: uninitialized urandom read (6 bytes read, 50 bits of entropy available)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   10.969726] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   11.155258] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   11.443022] random: mktemp: uninitialized urandom read (12 bytes read, 54 bits of entropy available)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   11.528186] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   11.619317] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   11.670836] EXT4-fs (sda1): resized filesystem to 7864064
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   12.095276] init: failsafe main process (1094) killed by TERM signal
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO Running set_multiqueue.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO Set channels for eth0 to 4.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 13 13:14:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 13 13:14:28 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 13:14:28 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 13:14:28 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 google-clock-skew: INFO Synced system time with hardware clock.
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   13.052915] random: nonblocking pool is initialized
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 google-accounts: INFO Starting Google Accounts daemon.
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 google-accounts: INFO Creating a new user account for me.
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 google-accounts: INFO Created user account me.
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 google-accounts: INFO Creating a new user account for bogdana.
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 google-accounts: INFO Created user account bogdana.
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 google-accounts: INFO Creating a new user account for aj.
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 google-accounts: INFO Created user account aj.
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 google-accounts: INFO Creating a new user account for asari.
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 google-accounts: INFO Created user account asari.
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 cron[1427]: (CRON) INFO (pidfile fd = 3)
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 cron[1476]: (CRON) STARTUP (fork ok)
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 cron[1476]: (CRON) INFO (Running @reboot jobs)
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 google-accounts: INFO Removing user packer.
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 acpid: starting up with netlink and the input layer
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 acpid: 1 rule loaded
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 acpid: waiting for events: event logging is off
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 haveged: haveged starting up
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   13.568831] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   13.571838] Bridge firewalling registered
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   13.587469] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   13.591547] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   13.607565] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 13:14:29 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   13.646345] floppy0: no floppy controllers found
Aug 13 13:14:30 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   14.660539] Initializing XFRM netlink socket
Aug 13 13:14:30 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   14.666510] Netfilter messages via NETLINK v0.30.
Aug 13 13:14:30 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   14.669605] ctnetlink v0.93: registering with nfnetlink.
Aug 13 13:14:52 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpdate[1771]: adjust time server 169.254.169.254 offset 0.002905 sec
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpd[1804]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpd[1805]: proto: precision = 0.107 usec
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpd[1805]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpd[1805]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpd[1805]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpd[1805]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpd[1805]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpd[1805]: Listen normally on 3 eth0 10.20.255.31 UDP 123
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpd[1805]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpd[1805]: peers refreshed
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpd[1805]: Listening on routing socket on fd #21 for interface updates
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   43.820293] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 startup-script: INFO Found startup-script in metadata.
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 startup-script: INFO startup-script: job 1 at Mon Aug 13 16:24:00 2018
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 startup-script: INFO startup-script: Return code 0.
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 startup-script: INFO startup-script: Return code 0.
Aug 13 13:14:59 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 startup-script: INFO Finished running startup scripts.
Aug 13 13:15:00 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ec2: 
Aug 13 13:15:00 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ec2: #############################################################
Aug 13 13:15:00 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 13 13:15:00 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ec2: 1024 bc:9a:cd:7d:5d:ce:b8:0f:7d:51:d5:7b:15:b5:59:e9  root@travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 (DSA)
Aug 13 13:15:00 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ec2: 256 89:64:46:a2:73:ca:66:1b:73:1f:52:fa:7e:41:8c:92  root@travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 (ECDSA)
Aug 13 13:15:00 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ec2: 256 b0:f5:12:32:14:bc:c6:75:21:85:dc:70:c1:a9:80:64  root@travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 (ED25519)
Aug 13 13:15:00 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ec2: 2048 6b:4f:ac:84:47:d1:18:bb:1f:fc:eb:18:9a:63:7b:0e  root@travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 (RSA)
Aug 13 13:15:00 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 13 13:15:00 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ec2: #############################################################
Aug 13 13:15:47 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [   91.161920] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 13 13:16:55 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [  159.133622] device veth5480c94 entered promiscuous mode
Aug 13 13:16:55 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [  159.133699] docker0: port 1(veth5480c94) entered forwarding state
Aug 13 13:16:55 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [  159.133707] docker0: port 1(veth5480c94) entered forwarding state
Aug 13 13:16:55 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [  159.134195] docker0: port 1(veth5480c94) entered disabled state
Aug 13 13:16:55 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [  159.236456] cgroup: docker-runc (4795) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 13 13:16:55 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [  159.236458] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 13 13:16:55 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [  159.303650] eth0: renamed from veth3df4916
Aug 13 13:16:55 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [  159.337000] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 13 13:16:55 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [  159.338265] docker0: port 1(veth5480c94) entered forwarding state
Aug 13 13:16:55 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [  159.338289] docker0: port 1(veth5480c94) entered forwarding state
Aug 13 13:16:55 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [  159.338315] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 13 13:16:58 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpd[1805]: Listen normally on 5 docker0 fe80::42:efff:fe4f:440b UDP 123
Aug 13 13:16:58 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpd[1805]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 13 13:16:58 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpd[1805]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 13 13:16:58 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpd[1805]: peers refreshed
Aug 13 13:16:58 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 ntpd[1805]: new interface(s) found: waking up resolver
Aug 13 13:17:01 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 CRON[4913]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 13 13:17:10 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [  174.362069] docker0: port 1(veth5480c94) entered forwarding state
Aug 13 14:03:50 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [ 2974.682838] traps: a[5297] trap invalid opcode ip:55d7c9c6ea9b sp:7ffd4fafb7e0 error:0 in a[55d7c9c6b000+6000]
Aug 13 14:04:05 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [ 2989.278871] traps: a[8114] trap invalid opcode ip:7fa84d236491 sp:7ffd0e4b80d0 error:0 in libstd-2339b911e3c09de8.so[7fa84d1d6000+16f000]
Aug 13 14:04:05 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [ 2989.311683] traps: a[8118] trap invalid opcode ip:7fe59a27b491 sp:7ffc699a1f60 error:0 in libstd-2339b911e3c09de8.so[7fe59a21b000+16f000]
Aug 13 14:05:27 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [ 3070.975381] traps: a[22954] trap invalid opcode ip:5652d6444d98 sp:7ffeabab5830 error:0 in a[5652d6442000+4000]
Aug 13 14:08:09 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [ 3233.176222] a[18951]: segfault at 0 ip 000055e411de4463 sp 00007ffdc203e540 error 6 in a[55e411de1000+5000]
Aug 13 14:08:18 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [ 3242.197109] a[19711]: segfault at 1 ip 0000562c59cbeb8c sp 00007ffe0c60f9c0 error 6 in a[562c59cbc000+4000]
Aug 13 14:08:21 travis-job-aa85de26-7a33-4ff2-8759-871b6a0d7742 kernel: [ 3245.802728] traps: a[20054] trap invalid opcode ip:56375397b42c sp:7fffc3752a90 error:0 in a[563753978000+7000]
---
travis_time:end:051713d3:start=1534169375706949024,finish=1534169375716392381,duration=9443357
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:026644fd
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05211840
travis_time:start:05211840
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:1fc428c9
$ dmesg | grep -i kill
