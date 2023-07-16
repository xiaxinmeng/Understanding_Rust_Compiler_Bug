plain
[00:48:57] ....................................................................................................
[00:49:00] ....................................................................................................
[00:49:02] ....................................................................................................
[00:49:05] ....................................................................................................
[00:49:08] .......iiiiiiiii....................................................................................
[00:49:14] ....................................................................................................
[00:49:18] ............i.......................................................................................
[00:49:20] .....................i..............................................................................
[00:49:24] ....................................................................................................
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:05] 
[00:57:05] running 104 tests
[00:57:08] i..ii..iii....i...i...........iii...........i....i....ii...i.i.ii..............i...ii..ii.i...F.iii.
[00:57:08] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:57:08] failures:
[00:57:08] 
[00:57:08] ---- [codegen] codegen/target-cpu-on-functions.rs stdout ----
[00:57:08] ---- [codegen] codegen/target-cpu-on-functions.rs stdout ----
[00:57:08] 
[00:57:08] error: compilation failed!
[00:57:08] status: exit code: 1
[00:57:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/target-cpu-on-functions.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/target-cpu-on-functions" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "no-prepopulate-passes" "-C" "panic=abort" "-Z" "cross-lang-lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/target-cpu-on-functions/auxiliary" "--emit=llvm-ir"
[00:57:08] ------------------------------------------
[00:57:08] 
[00:57:08] ------------------------------------------
[00:57:08] stderr:
[00:57:08] stderr:
[00:57:08] ------------------------------------------
[00:57:08] error: The current compilation is going to use thin LTO buffers without running LLVM's NameAnonGlobals pass. This will likely cause errors in LLVM. Consider adding -C passes=name-anon-globals to the compiler command line.
[00:57:08] error: aborting due to previous error
[00:57:08] 
[00:57:08] 
[00:57:08] ------------------------------------------
---
[00:57:08] test result: FAILED. 75 passed; 1 failed; 28 ignored; 0 measured; 0 filtered out
[00:57:08] 
[00:57:08] 
[00:57:08] 
[00:57:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:08] 
[00:57:08] 
[00:57:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:08] Build completed unsuccessfully in 0:10:58
[00:57:08] Build completed unsuccessfully in 0:10:58
[00:57:08] make: *** [check] Error 1
[00:57:08] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03ceba46
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:028fa4f6
$ sudo tail -n 500 /var/log/syslog
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]   6 disabled
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]   7 disabled
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2810-0x000f281f] mapped at [ffff8800000f2810]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Using GB pages for direct mapping
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27D0 000014 (v00 Google)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] kvm-clock: using sched offset of 1429563638 cycles
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Zone ranges:
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]   Device   empty
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Movable zone start for each node
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Early memory node ranges
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Policy zone: Normal
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] console [ttyS0] enabled
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.000000] tsc: Detected1] Initializing cgroup subsys io
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.397050] Initializing cgroup subsys memory
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.397910] Initializing cgroup subsys devices
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.398717] Initializing cgroup subsys freezer
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.399388] Initializing cgroup subsys net_cls
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.400268] Initializing cgroup subsys perf_event
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.401312] Initializing cgroup subsys net_prio
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.402182] Initializing cgroup subsys hugetlb
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.402913] Initializing cgroup subsys pids
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.403775] CPU: Physical Processor ID: 0
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.404620] CPU: Processor Core ID: 0
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.405191] mce: CPU supports 32 MCE banks
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.406259] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.407316] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.591293]  #2
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.591815] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.595220]  #3
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.595990] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.599176] x86: Booted up 1 node, 4 CPUs
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.600255] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.602693] devtmpfs: initialized
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.607113] evm: security.selinux
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.607876] evm: security.SMACK64
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.608398] evm: security.SMACK64EXEC
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.609180] evm: security.SMACK64TRANSMUTE
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.609936] evm: security.SMACK64MMAP
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.610503] evm: security.ima
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.610990] evm: security.capability
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.611928] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.613429] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.614739] pinctrl core: initialized pinctrl subsystem
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.615983] RTC time: 12:49:09, date: 08/07/18
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.617510] NET: Registered protocol family 16
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.629083] cpuidle: using governor ladder
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.641094] cpuidle: using governor menu
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.641830] PCCT header not found.
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.642634] ACPI: bus type PCI registered
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.643525] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.644898] PCI: Using configuration type 1 for base access
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.658222] ACPI: Added _OSI(Module Device)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.659029] ACPI: Added _OSI(Processor Device)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.659962] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.660700] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.664158] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.687735] ACPI: Interpreter enabled
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.688388] ACPI: (supports S0 S3 S4 S5)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.689450] ACPI: Using IOAPIC for interrupt routing
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.690180] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.719932] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.720875] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.721855] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.722804] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.725050] PCI host bridge to bus 0000:00
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.725825] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.727421] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.728590] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.730388] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.731693] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.732735] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.733155] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.747743] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.762776] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.764436] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.770007] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.775850] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.790564] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.797171] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.802147] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.817769] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.820584] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.822996] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.826916] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.829886] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.850103] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.851408] vgaarb: loaded
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.852095] SCSI subsystem initialized
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.853040] libata version 3.00 loaded.
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.853068] ACPI: bus type USB registered
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.853988] usbcore: registered new interface driver usbfs
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.855233] usbcore: registered new interface driver hub
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.856094] usbcore: registered new device driver usb
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.857916] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.859226] dmi: Firmware registration failed.
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.860148] PCI: Using ACPI for IRQ routing
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.861157] PCI: pci_cache_line_size set to 64 bytes
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.861253] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.861254] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.861382] NetLabel: Initializing
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.862145] NetLabel:  domain hash size = 128
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.862844] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.863805] NetLabel:  unlabeled traffic allowed by default
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.865268] amd_nb: Cannot enumerate AMD northbridges
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.866213] clocksource: Switched to clocksource kvm-clock
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.873977] pnp: PnP ACPI init
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.874538] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.874618] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.874663] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.874713] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.874754] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.874797] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.874847] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.875005] pnp: PnP ACPI: found 7 devices
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.883436] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.884884] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.884887] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.884889] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.884890] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.884929] NET: Registered protocol family 2
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.885838] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.888167] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.889891] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.891138] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.892469] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.893735] NET: Registered protocol family 1
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.895210] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.896352] PCI: CLS 0 bytes, default 64
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    0.897195] Unpacking initramfs...
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.909817] Freeing initrd memory: 21432K
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.910619] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.911713] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.913340] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.914653] hw unit of domain pp0-core 2^-0 Joules
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.915772] hw unit of domain package 2^-0 Joules
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.916636] hw unit of domain dram 2^-0 Joules
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.917505] Scanning for low memory corruption every 60 seconds
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.918921] audit: initializing netlink subsys (disabled)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.919863] audit: type=2000 audit(1533646151.702:1): initialized
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.921412] Initialise system trusted keyring
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.922634] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.924054] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.926413] zbud: loaded
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.927198] VFS: Disk quotas dquot_6.6.0
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.928106] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.929332] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.930634] fuse init (API version 7.23)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.931462] Key type big_key registered
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.932185] Allocating IMA MOK and blacklist keyrings.
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45dep Button [SLPF]
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.945905] GHES: HEST is not enabled!
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.948515] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.949526] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.954815] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.955885] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.961905] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    2.984666] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.009433] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.033456] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.057550] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.061765] Linux agpgart interface v0.103
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25bat 0x60,0x64 irq 1,12
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.125107] i8042: Warning: Keylock active
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.127191] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.128777] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.130414] mousedev: PS/2 mouse device common for all mice
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.132267] rtc_cmos 00:00: RTC can wake from S4
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.133954] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.136043] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.137905] i2c /dev entries driver
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.138995] device-mapper: uevent: version 1.0.3
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.140340] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.142702] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.145433] NET: Registered protocol family 10
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.146787] NET: Registered protocol family 17
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.147557] Key type dns_resolver registered
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.148810] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.150023] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.151846] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.153338] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.155297] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.157738] registered taskstats version 1
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.158893] Loading compiled-in X.509 certificates
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.160871] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.162615] zswap: loaded using pool lzo/zbud
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.165647] Key type trusted registered
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.169942] Key type encrypted registered
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.170858] ima: No TPM chip found, activating TPM-bypass!
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.171631] evm: HMAC attrs: 0x1
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.172800]   Magic number: 14:596:834
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.174079] rtc_cmos 00:00: setting system clock to 2018-08-07 12:49:12 UTC (1533646152)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.175930] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.177774] EDD information not available.
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.179280] PM: Hibernation image not present or could not be loaded.
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.180741] Freeing unused kernel memory: 1496K
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.181946] Write protecting the kernel read-only data: 14336k
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.183872] Freeing unused kernel memory: 1956K
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.185002] Freeing unused kernel memory: 92K
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.200175] systemd-udevd[118]: starting version 204
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.263586] scsi host0: Virtio SCSI HBA
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.269338] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.277459] AVX version of gcm_enc/dec engaged.
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.278239] AES CTR mode by8 optimization enabled
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.314043] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.314077] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.316652] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.317929] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.318858] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.319064] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.322705]  sda: sda1
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.323978] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.330735] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.914345] tsc: Refined TSC clocksource calibration: 2600.002 MHz
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    3.915380] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3e40e8d, max_idle_ns: 440795298933 ns
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    4.167971] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    6.254413] floppy0: no floppy controllers found
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.410303] raid6: sse2x1   gen()  8890 MB/s
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.478217] raid6: sse2x1   xor()  6760 MB/s
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.546219] raid6: sse2x2   gen() 10958 MB/s
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.614220] raid6: sse2x2   xor()  7343 MB/s
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.682218] raid6: sse2x4   gen() 12834 MB/s
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.750217] raid6: sse2x4   xor()  8974 MB/s
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.750971] raid6: using algorithm sse2x4 gen() 12834 MB/s
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.751724] raid6: .... xor() 8974 MB/s, rmw enabled
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.752492] raid6: using ssse3x2 recovery algorithm
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.754508] xor: automatically using best checksumming function:
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.794215]    avx       : 27816.000 MB/sec
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.807603] Btrfs loaded
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.850161] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.851192] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.914015] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.920244] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.921024] EXT4-fs (sda1): recovery complete
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    7.925688] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    8.122591] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    8.221381] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    8.266572] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    8.455114] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    8.977226] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    9.107254] systemd-udevd[702]: starting version 204
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    9.207436] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    9.277964] intel_rapl: no valid rapl domains found in package 0
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    9.338550] ppdev: user-space parallel port driver
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    9.455069] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    9.498830] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    9.563038] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    9.728394] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [    9.995159] random: mktemp: uninitialized urandom read (12 bytes read, 58 bits of entropy available)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [   10.064464] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [   10.134676] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [   10.175202] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [   10.393444] init: failsafe main process (1093) killed by TERM signal
Aug  7 12:49:19 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO Running set_multiqueue.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO Set channels for eth0 to 4.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [   11.208241] random: nonblocking pool is initialized
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 google-accounts: INFO Starting Google Accounts daemon.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 google-accounts: INFO Creating a new user account for me.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 google-accounts: INFO Created user account me.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 google-accounts: INFO Creating a new user account for aj.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 google-accounts: INFO Created user account aj.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 google-accounts: INFO Creating a new user account for carmen.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 google-accounts: INFO Created user account carmen.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 google-accounts: INFO Removing user packer.
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 cron[1428]: (CRON) INFO (pidfile fd = 3)
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 cron[1469]: (CRON) STARTUP (fork ok)
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 cron[1469]: (CRON) INFO (Running @reboot jobs)
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 acpid: starting up with netlink and the input layer
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 acpid: 1 rule loaded
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 acpid: waiting for events: event logging is off
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 haveged: haveged starting up
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 12:49:20 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 12:49:21 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [   11.693560] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 12:49:21 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [   11.702361] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 12:49:21 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [   11.785483] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 12:49:21 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [   11.789197] Bridge firewalling registered
Aug  7 12:49:21 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [   11.800452] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 12:49:21 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [   11.872062] Initializing XFRM netlink socket
Aug  7 12:49:21 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [   11.879673] Netfilter messages via NETLINK v0.30.
Aug  7 12:49:21 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [   11.882929] ctnetlink v0.93: registering with nfnetlink.
Aug  7 12:49:21 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 12:49:21 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [   12.294386] floppy0: no floppy controllers found
Aug  7 12:49:43 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ntpdate[1763]: adjust time server 169.254.169.254 offset 0.016025 sec
Aug  7 12:49:50 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ntpd[1798]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 12:49:50 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ntpd[1799]: proto: precision = 0.100 usec
Aug  7 12:49:50 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ntpd[1799]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 12:49:50 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ntpd[1799]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 12:49:50 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ntpd[1799]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 12:49:50 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ntpd[1799]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  7 12:49:50 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ntpd[1799]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 12:49:50 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ntpd[1799]: Listen normally on 3 eth0 10.20.0.8 UDP 123
Aug  7 12:49:50 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ntpd[1799]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 12:49:50 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ntpd[1799]: peers refreshed
Aug  7 12:49:50 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ntpd[1799]: Listening on routing socket on fd #21 for interface updates
Aug  7 12:49:50 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [   41.872089] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 12:49:51 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 startup-script: INFO Found startup-script in metadata.
Aug  7 12:49:51 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 12:49:51 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 startup-script: INFO startup-script: job 1 at Tue Aug  7 15:59:00 2018
Aug  7 12:49:51 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 startup-script: INFO startup-script: Return code 0.
Aug  7 12:49:51 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 startup-script: INFO startup-script: Return code 0.
Aug  7 12:49:51 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 startup-script: INFO Finished running startup scripts.
Aug  7 12:49:51 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ec2: 
Aug  7 12:49:51 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ec2: #############################################################
Aug  7 12:49:51 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 12:49:51 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ec2: 1024 77:fe:49:f2:bf:d9:15:1c:2f:30:62:62:93:5e:a1:f0  root@travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 (DSA)
Aug  7 12:49:51 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ec2: 256 22:d8:1c:26:5c:45:6c:8d:f4:03:0f:2f:25:24:d2:d8  root@travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 (ECDSA)
Aug  7 12:49:51 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 ec2: 256 87:c opcode ip:7fdb309bcef1 sp:7ffc14b059b0 error:0 in libstd-e054c7a28f8831a7.so[7fdb30961000+172000]
Aug  7 13:41:36 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [ 3147.519666] traps: a[24289] trap invalid opcode ip:558e610c5e68 sp:7fff9c9d4f60 error:0 in a[558e610c3000+4000]
Aug  7 13:44:27 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [ 3318.396408] a[22136]: segfault at 0 ip 00005619bb252658 sp 00007ffcb24abda0 error 6 in a[5619bb24f000+5000]
Aug  7 13:44:37 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [ 3328.113383] a[22949]: segfault at 1 ip 00005638065f5c6c sp 00007ffd3010cb20 error 6 in a[5638065f3000+4000]
Aug  7 13:44:41 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [ 3332.153534] traps: a[23338] trap invalid opcode ip:55bed66ec5bc sp:7ffc84c80c90 error:0 in a[55bed66e9000+7000]
Aug  7 13:47:30 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [ 3501.713964] veth7b6c304: renamed from eth0
Aug  7 13:47:30 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [ 3501.727691] docker0: port 1(veth080853f) entered disabled state
Aug  7 13:47:30 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [ 3501.773411] docker0: port 1(veth080853f) entered disabled state
Aug  7 13:47:30 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [ 3501.774990] device veth080853f left promiscuous mode
Aug  7 13:47:30 travis-job-b2dab545-6a5d-4ce2-9b7f-25b45d7e29a4 kernel: [ 3501.774992] docker0: port 1(veth080853f) entered disabled state
travis_fold:end:after_failure.1
tra-emscripten/lib
60428 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools
56324 ./src/llvm/test/MC
---
travis_time:end:0783785a:start=1533649652599385900,finish=1533649652606987975,duration=7602075
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05ad970b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0657c938
travis_time:start:0657c938
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0087a814
$ dmesg | grep -i kill
