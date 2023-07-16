\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs","byte_start":611,"byte_end":620,"line_start":17,"line_end":17,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    x.push(y); //~ ERROR explicit lifetime required","highlight_start":5,"highlight_end":14}],"label":"lifetime `'a` required","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add explicit lifetime `'a` to the type of `y`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs","byte_start":554,"byte_end":556,"line_start":13,"line_end":13,"column_start":42,"column_end":44,"is_primary":true,"text":[{"text":"fn baz<'a, 'b, T>(x: &mut Vec<&'a T>, y: &T)","highlight_start":42,"highlight_end":44}],"label":null,"suggested_replacement":"&'a T","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0621]: explicit lifetime required in the type of `y`\n  --> /checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs:17:5\n   |\nLL | fn baz<'a, 'b, T>(x: &mut Vec<&'a T>, y: &T)\n   |                                          -- help: add explicit lifetime `'a` to the type of `y`: `&'a T`\n...\nLL |     x.push(y); //~ ERROR explicit lifetime required\n   |     ^^^^^^^^^ lifetime `'a` required\n\n"}
[00:44:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:44:30] {"message":"For more information about this error, try `rustc --explain E0621`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0621`.\n"}
[00:44:30] ------------------------------------------
[00:44:30] 
[00:44:30] thread '[ui (nll)] ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:44:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:44:30] 
[00:44:30] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:44:30] 
[00:44:30] 
[00:44:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:44:30] 
[00:44:30] 
[00:44:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:30] Build completed unsuccessfully in 0:03:04
[00:44:30] Build completed unsuccessfully in 0:03:04
[00:44:30] make: *** [check] Error 1
[00:44:30] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f90be20
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:03349eb1
$ sudo tail -n 500 /var/log/syslog
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   00000-9FFFF write-back
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   A0000-BFFFF uncachable
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   C0000-FFFFF write-protect
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] MTRR variable ranges enabled:
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   0 base 0000C0000000 mask 3FFFC0000000 uncachable
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   1 disabled
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   2 disabled
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   3 disabled
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   4 disabled
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   5 disabled
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   6 disabled
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   oogle)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] kvm-clock: using sched offset of 1517351032 cycles
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] Zone ranges:
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   Device   empty
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] Movable zone start for each node
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] Early memory node ranges
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] Policy zone: Normal
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] console [ttyS0] enabled
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.505258] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.507975] pid_max: default: 32768 minimum: 301
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.509738] ACPI: Core revision 20150930
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.517210] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.519962] Security Framework initialized
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.521761] Yama: becoming mindful.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.523277] AppArmor: AppArmor disabled by boot time parameter
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.527201] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.538705] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.545174] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.547212] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.549959] Initializing cgroup subsys io
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.551719] Initializing cgroup subsys memory
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.553193] Initializing cgroup subsys devices
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.554792] Initializing cgroup subsys freezer
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.556106] Initializing cgroup subsys net_cls
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.557717] Initializing cgroup subsys perf_event
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.559448] Initializing cgroup subsys net_prio
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.560790] Initializing cgroup 0 pin1=0 apic2=-1 pin2=-1
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.755437] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.758679] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.763414] x86: Booting SMP configuration:
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.764799] .... node  #0, CPUs:      #1
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.766484] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.772644]  #2
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.773697] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.779140]  #3
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.780247] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.785735] x86: Booted up 1 node, 4 CPUs
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.787556] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.790992] devtmpfs: initialized
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.796382] evm: security.selinux
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.797559] evm: security.SMACK64
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.798808] evm: security.SMACK64EXEC
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.800279] evm: security.SMACK64TRANSMUTE
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.801666] evm: security.SMACK64MMAP
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.803321] evm: security.ima
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.804234] evm: security.capability
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.805871] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.808824] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.810977] pinctrl core: initialized pinctrl subsystem
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.812724] RTC time: 14:02:22, date: 08/08/18
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.815118] NET: Registered protocol family 16
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.827495] cpuidle: using governor ladder
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.839492] cpuidle: using governor menu
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.840993] PCCT header not found.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.842249] ACPI: bus type PCI registered
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.843417] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.845793] PCI: Using configuration type 1 for base access
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.861020] ACPI: Added _OSI(Module Device)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.862456] ACPI: Added _OSI(Processor Device)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.863712] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.865428] ACPI: Added _OSI(Processor Aggregator Device)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.869639] ACPI: Executed 2 blocks of module-level executable AML code
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.894298] ACPI: Interpreter enabled
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.895538] ACPI: (supports S0 S3 S4 S5)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.897237] ACPI: Using IOAPIC for interrupt routing
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.898703] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.930510] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.932483] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.934624] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.936718] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.941060] PCI host bridge to bus 0000:00
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.942380] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.944295] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.946692] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.949077] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    0.951254] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.078166] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.081742] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.085495] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.107683] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.109827] vgaarb: loaded
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.111119] SCSI subsystem initialized
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.112600] libata version 3.00 loaded.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.112624] ACPI: bus type USB registered
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.113754] usbcore: registered new interface driver usbfs
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.115525] usbcore: registered new interface driver hub
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.117394] usbcore: registered new device driver usb
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.119546] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.121892] dmi: Firmware registration failed.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.123635] PCI: Using ACPI for IRQ routing
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.125297] PCI: pci_cache_line_size set to 64 bytes
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.125410] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.125412] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.125565] NetLabel: Initializing
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.126605] NetLabel:  domain hash size = 128
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.127836] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.129235] NetLabel:  unlabeled traffic allowed by default
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.131017] amd_nb: Cannot enumerate AMD northbridges
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.133186] clocksource: Switched to clocksource kvm-clock
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.144224] pnp: PnP ACPI init
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.145235] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.145307] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.145350] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.145399] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.145439] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.145477] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.145515] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.145682] pnp: PnP ACPI: found 7 devices
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.153937] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.156650] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.156652] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.156653] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.156654] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.156696] NET: Registered protocol family 2
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.158125] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.160405] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.162838] TCP: Hash tables configured (established 131072 bind 65536)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.164839] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.166667] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.170214] NET: Registered protocol family 1
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.171506] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.173210] PCI: CLS 0 bytes, default 64
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    1.173277] Unpacking initramfs...
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.318266] Freeing initrd memory: 21432K
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.319898] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.321788] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.326246] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.329479] hw unit of domain pp0-core 2^-0 Joules
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.331047] hw unit of domain package 2^-0 Joules
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.332419] hw unit of domain dram 2^-16 Joules
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.333957] Scanning for low memory corruption every 60 seconds
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.336848] audit: initializing netlink subsys (disabled)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.338650] audit: type=2000 audit(1533736945.138:1): initialized
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.340992] Initialise system trusted keyring
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.342872] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.344840] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.348025] zbud: loaded
Aug  8 14:02:33 travis-job-b066228c-cf51-47f-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.374566] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.377011] intel_idle: does not run on family 6 model 63
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.377119] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.379736] ACPI: Power Button [PWRF]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.381105] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.383878] ACPI: Sleep Button [SLPF]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.385540] GHES: HEST is not enabled!
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.389231] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.391438] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.399259] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.401290] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.408818] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.433012] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.458643] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.485324] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.510488] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.516005] Linux agpgart interface v0.103
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.521318] loop: module loaded
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.522825] libphy: Fixed MDIO Bus: probed
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.524885] tun: Universal TUN/TAP device driver, 1.6
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.527011] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.567166] PPP generic driver version 2.4.2
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.569307] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.572119] ehci-pci: EHCI PCI platform driver
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.573455] ehci-platform: EHCI generic platform driver
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.575143] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.577120] ohci-pci: OHCI PCI platform driver
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.578556] ohci-platform: OHCI generic platform driver
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.580594] uhci_hcd: USB Universal Host Controller Interface driver
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.583069] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.585889] i8042: Warning: Keylock active
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.588638] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.590456] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.592282] mousedev: PS/2 mouse device common for all mice
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.594701] rtc_cmos 00:00: RTC can wake from S4
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.596399] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.599157] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.601480] i2c /dev entries driver
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.602606] device-mapper: uevent: version 1.0.3
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.604154] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.606702] ledtrig-cpu: registered to indicate activity on CPUs
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.609942] NET: Registered protocol family 10
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.611873] NET: Registered protocol family 17
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.613669] Key type dns_resolver registered
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.616040] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.618480] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.620467] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.622594] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.624530] microcode: Microcode Upis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.665180] PM: Hibernation image not present or could not be loaded.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.667063] Freeing unused kernel memory: 1496K
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.668663] Write protecting the kernel read-only data: 14336k
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.671718] Freeing unused kernel memory: 1956K
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.673420] Freeing unused kernel memory: 92K
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.691179] systemd-udevd[119]: starting version 204
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.749324] scsi host0: Virtio SCSI HBA
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.756453] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.762981] AVX2 version of gcm_enc/dec engaged.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.764377] AES CTR mode by8 optimization enabled
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.793638] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.803003] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.803176] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.803178] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.803920] sd 0:0:1:0: [sda] Write Protect is off
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.803921] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.804000] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.806931]  sda: sda1
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    3.808392] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    4.333333] tsc: Refined TSC clocksource calibration: 2299.796 MHz
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    4.336098] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2126747b124, max_idle_ns: 440795298764 ns
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    4.631650] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    6.737465] floppy0: no floppy controllers found
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    7.901225] raid6: sse2x1   gen()  8904 MB/s
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    7.969223] raid6: sse2x1   xor()  6542 MB/s
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.037227] raid6: sse2x2   gen() 11003 MB/s
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.105224] raid6: sse2x2   xor()  7535 MB/s
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.173225] raid6: sse2x4   gen() 12191 MB/s
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.241226] raid6: sse2x4   xor()  8020 MB/s
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.309225] raid6: avx2x1   gen() 16885 MB/s
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.377223] raid6: avx2x2   gen() 19615 MB/s
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.445224] raid6: avx2x4   gen() 20528 MB/s
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.446669] raid6: using algorithm avx2x4 gen() 20528 MB/s
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.448669] raid6: using avx2x2 recovery algorithm
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.451876] xor: automatically using best checksumming function:
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.493210]    avx       : 21586.000 MB/sec
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.508955] Btrfs loaded
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.561457] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.563782] EXT4-fs (sda1): write access will be enabled during recovery
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.633987] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.642684] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.644523] EXT4-fs (sda1): recovery complete
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.650752] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    8.880635] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    9.012911] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    9.067845] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    9.283816] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [    9.882712] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   10.007454] systemd-udevd[702]: starting version 204
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   10.114656] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   10.188701] intel_rapl: no valid rapl domains found in package 0
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   10.242500] ppdev: user-space parallel port driver
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   10.316098] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   10.367886] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   10.430834] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   10.589994] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   10.852925] random: mktemp: uninitialized urandom read (12 bytes read, 59 bits of entropy available)
Aug  8 14:02:33 travis-jonity 1
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 google-accounts: INFO Starting Google Accounts daemon.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 google-accounts: INFO Creating a new user account for me.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 google-accounts: INFO Created user account me.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 google-accounts: INFO Removing user packer.
Aug  8 14:02:33 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   12.152400] random: nonblocking pool is initialized
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 cron[1404]: (CRON) INFO (pidfile fd = 3)
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 cron[1443]: (CRON) STARTUP (fork ok)
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 cron[1443]: (CRON) INFO (Running @reboot jobs)
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 acpid: starting up with netlink and the input layer
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 acpid: 1 rule loaded
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 acpid: waiting for events: event logging is off
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 haveged: haveged starting up
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   12.646408] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   12.655948] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   12.753031] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   12.757463] Bridge firewalling registered
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   12.768817] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   12.837360] Initializing XFRM netlink socket
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   12.845136] Netfilter messages via NETLINK v0.30.
Aug  8 14:02:34 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   12.848450] ctnetlink v0.93: registering with nfnetlink.
Aug  8 14:02:35 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 google-clock-skew: INFO Synced system time with hardware clock.
Aug  8 14:02:35 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [   13.185359] floppy0: no floppy controllers found
Aug  8 14:02:58 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ntpdate[1757]: adjust time server 169.254.169.254 offset 0.007135 sec
Aug  8 14:03:04 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ntpd[1793]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  8 14:03:04 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ntpd[1794]: proto: precision = 0.105 usec
Aug  8 14:03:04 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ntpd[1794]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 14:03:04 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ntpd[1794]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 14:03:04 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ntpd[1794]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  8 14:03:04 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ntpd[1794]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  8 14:03:04 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ntpd[1c0f8075 ec2: 1024 cf:51:04:74:66:01:2d:07:1f:03:33:2c:60:a3:d7:ca  root@travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 (DSA)
Aug  8 14:03:05 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ec2: 256 8c:e2:e0:31:0d:fb:a1:d4:25:c3:e4:c2:06:bd:ac:5b  root@travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 (ECDSA)
Aug  8 14:03:05 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ec2: 256 b4:ae:27:bf:23:de:d6:ff:7f:53:d0:a0:0a:bf:9e:6d  root@travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 (ED25519)
Aug  8 14:03:05 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ec2: 2048 ee:6d:26:cb:af:f0:6f:fa:dc:1b:d3:0a:7d:20:87:4c  root@travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 (RSA)
Aug  8 14:03:05 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  8 14:03:05 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ec2: #############################################################
Aug  8 14:04:26 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [  124.584382] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  8 14:05:28 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [  186.396857] device vethe9a9595 entered promiscuous mode
Aug  8 14:05:28 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [  186.495042] cgroup: docker-runc (4786) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  8 14:05:28 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [  186.495044] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  8 14:05:28 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [  186.570694] eth0: renamed from veth57bae50
Aug  8 14:05:28 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [  186.606814] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  8 14:05:28 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [  186.608099] docker0: port 1(vethe9a9595) entered forwarding state
Aug  8 14:05:28 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [  186.608142] docker0: port 1(vethe9a9595) entered forwarding state
Aug  8 14:05:28 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [  186.608162] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  8 14:05:31 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ntpd[1794]: Listen normally on 5 docker0 fe80::42:81ff:fe06:4aa3 UDP 123
Aug  8 14:05:31 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ntpd[1794]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  8 14:05:31 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ntpd[1794]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  8 14:05:31 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ntpd[1794]: peers refreshed
Aug  8 14:05:31 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 ntpd[1794]: new interface(s) found: waking up resolver
Aug  8 14:05:43 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [  201.622359] docker0: port 1(vethe9a9595) entered forwarding state
Aug  8 14:17:01 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 CRON[12523]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  8 14:48:58 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [ 2795.790214] veth57bae50: renamed from eth0
Aug  8 14:48:58 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [ 2795.825059] docker0: port 1(vethe9a9595) entered disabled state
Aug  8 14:48:58 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [ 2795.846562] docker0: port 1(vethe9a9595) entered disabled state
Aug  8 14:48:58 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [ 2795.848475] device vethe9a9595 left promiscuous mode
Aug  8 14:48:58 travis-job-b066228c-cf51-47f6-b209-3f341c0f8075 kernel: [ 2795.848479] docker0: port 1(vethe9a9595) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:13f034c4
---
travis_time:end:23cffd36:start=1533739739518269498,finish=1533739739524891825,duration=6622327
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15e200c7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13db6236
travis_time:start:13db6236
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0853ec7a
$ dmesg | grep -i kill
