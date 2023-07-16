\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs","byte_start":611,"byte_end":620,"line_start":17,"line_end":17,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    x.push(y); //~ ERROR explicit lifetime required","highlight_start":5,"highlight_end":14}],"label":"lifetime `'a` required","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add explicit lifetime `'a` to the type of `y`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs","byte_start":554,"byte_end":556,"line_start":13,"line_end":13,"column_start":42,"column_end":44,"is_primary":true,"text":[{"text":"fn baz<'a, 'b, T>(x: &mut Vec<&'a T>, y: &T)","highlight_start":42,"highlight_end":44}],"label":null,"suggested_replacement":"&'a T","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0621]: explicit lifetime required in the type of `y`\n  --> /checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs:17:5\n   |\nLL | fn baz<'a, 'b, T>(x: &mut Vec<&'a T>, y: &T)\n   |                                          -- help: add explicit lifetime `'a` to the type of `y`: `&'a T`\n...\nLL |     x.push(y); //~ ERROR explicit lifetime required\n   |     ^^^^^^^^^ lifetime `'a` required\n\n"}
[00:50:21] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:21] {"message":"For more information about this error, try `rustc --explain E0621`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0621`.\n"}
[00:50:21] ------------------------------------------
[00:50:21] 
[00:50:21] thread '[ui (nll)] ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:50:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:50:21] 
[00:50:21] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:50:21] 
[00:50:21] 
[00:50:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:50:21] 
[00:50:21] 
[00:50:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:21] Build completed unsuccessfully in 0:03:21
[00:50:21] Build completed unsuccessfully in 0:03:21
[00:50:21] make: *** [check] Error 1
[00:50:21] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1c2a3b6f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:03f62563
$ sudo tail -n 500 /var/log/syslog
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] kvm-clock: using sched offset of 1559116688 cycles
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] Zone ranges:
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000]   Device   empty
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] Movable zone start for each node
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] Early memory node ranges
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] Policy zone: Normal
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] console [ttyS0] enabled
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.551122] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.555150] pid_max: default: 32768 minimum: 301
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.557611] ACPI: Core revision 20150930
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.565058] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.568155] Security Framework initialized
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.570662] Yama: becoming mindful.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.572526] AppArmor: AppArmor disabled by boot time parameter
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.577370] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.589334] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.596276] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.600668] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.604081] Initializing cgroup subsys io
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.606261] Initializing cgroup subsys memory
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.608241] Initializing cgroup subsys devices
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.610254] Initializing cgroup subsys freezer
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.612920] Initializing cgroup subsys net_cls
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.615111] Initializing cgroup subsys perf_event
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.618198] Initializing cgroup subsys net_prio
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.620979] Initializing cgroup subsys hugetlb
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.623092] Initializing cgroup subsys pids
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.625385] CPU: Physical Processor ID: 0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.627584] CPU: Processor Core ID: 0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.630139] mce: CPU supports 32 MCE banks
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.632211] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.635194] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.640181] Freeing SMP alternatives memory: 32K
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.650359] ftrace: allocating 32185 entries in 126 pages
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.700087] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.703583] smpboot: Max logical packages: 2
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.706518] x2apic enabled
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.709598] Switched APIC routing to physical x2apic.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.715049] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.824659] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.829082] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.834708] x86: Booting SMP configuration:
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.836544] .... node  #0, CPUs:      #1
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.838462] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.844563]  #2
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.845655] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.851546]  #3
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.852726] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.858933] x86: Booted up 1 node, 4 CPUs
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.861063] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.865995] devtmpfs: initialized
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.871259] evm: security.selinux
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.873043] evm: security.SMACK64
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.874432] evm: security.SMACK64EXEC
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.876190] evm: security.SMACK64TRANSMUTE
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.877918] evm: security.SMACK64MMAP
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.879869] evm: security.ima
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.881479] evm: security.capability
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.883743] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.888238] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.891318] pinctrl core: initialized pinctrl subsystem
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.894053] RTC time: 14:02:05, date: 08/08/18
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.897363] NET: Registered protocol family 16
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.908757] cpuidle: using governor ladder
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.920757] cpuidle: using governor menu
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.923167] PCCT header not found.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.924992] ACPI: bus type PCI registered
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.927146] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.930170] PCI: Using configuration type 1 for base access
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.946301] ACPI: Added _OSI(Module Device)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.949050] ACPI: Added _OSI(Processor Device)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.951496] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.954102] ACPI: Added _OSI(Processor Aggregator Device)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.959582] ACPI: Executed 2 blocks of module-level executable AML code
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.985358] ACPI: Interpreter enabled
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.987057] ACPI: (supports S0 S3 S4 S5)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.988776] ACPI: Using IOAPIC for interrupt routing
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    0.991016] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.023256] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.026224] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.029647] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.032767] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.039743] PCI host bridge to bus 0000:00
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.042009] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.045229] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.048288] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.051307] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.054733] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.057778] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.058244] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.080349] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.102087] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.106278] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.114478] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.121097] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.140363] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.148497] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.155269] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.175331] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.180304] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.185072] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.189490] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.194251] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.215813] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.218985] vgaarb: loaded
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.220506] SCSI subsystem initialized
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.222127] libata version 3.00 loaded.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.222148] ACPI: bus type USB registered
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.223855] usbcore: registered new interface driver usbfs
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.226444] usbcore: registered new interface driver hub
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.228813] usbcore: registered new device driver usb
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.231643] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.234551] dmi: Firmware registration failed.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.236707] PCI: Using ACPI for IRQ routing
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.238940] PCI: pci_cache_line_size set to 64 bytes
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.239039] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.239041] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.239186] NetLabel: Initializing
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.240865] NetLabel:  domain hash size = 128
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.242988] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.245176] NetLabel:  unlabeled traffic allowed by default
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.248445] amd_nb: Cannot enumerate AMD northbridges
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.251011] clocksource: Switched to clocksource kvm-clock
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.263402] pnp: PnP ACPI init
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.265233] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.265309] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.265352] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.265400] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.265441] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.265482] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.265521] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.265688] pnp: PnP ACPI: found 7 devices
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.274826] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.279322] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.279324] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.279326] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.279327] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.279373] NET: Registered protocol family 2
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.281529] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.285289] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.288664] TCP: Hash tables configured (established 131072 bind 65536)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.291689] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.294692] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.298560] NET: Registered protocol family 1
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.300418] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.303187] PCI: CLS 0 bytes, default 64
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    1.303253] Unpacking initramfs...
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.338985] Freeing initrd memory: 21432K
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.340813] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.343571] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.348070] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.352121] hw unit of domain pp0-core 2^-0 Joules
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.354553] hw unit of domain package 2^-0 Joules
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.356698] hw unit of domain dram 2^-16 Joules
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.359149] Scanning for low memory corruption every 60 seconds
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.362971] audit: initializing netlink subsys (disabled)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.365573] audit: type=2000 audit(1533736927.093:1): initialized
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.368885] Initialise system trusted keyring
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.371367] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.374198] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.378484] zbud: loaded
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.379977] VFS: Disk quotas dquot_6.6.0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.381777] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.384732] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.387700] fuse init (API version 7.23)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.389697] Key type big_key registered
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.391764] Allocating IMA MOK and blacklist keyrings.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.398263] Key type asymmetric registered
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.400337] Asymmetric key parser 'x509' registered
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.402745] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.406535] io scheduler noop registered
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.408660] io scheduler deadline registered (default)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.411091] io scheduler cfq registered
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.412721] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.415145] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.418344] intel_idle: does not run on family 6 model 63
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.418447] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.421587] ACPI: Power Button [PWRF]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.423174] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.426428] ACPI: Sleep Button [SLPF]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.428504] GHES: HEST is not enabled!
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.432418] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.435384] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.444141] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.446809] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.456630] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.480949] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.505775] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.530566] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.555273] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.560880] Linux agpgart interface v0.103
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.567633] loop: module loaded
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.569392] libphy: Fixed MDIO Bus: probed
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.571124] tun: Universal TUN/TAP device driver, 1.6
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.573259] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.616988] PPP generic driver version 2.4.2
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.620153] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.622944] ehci-pci: EHCI PCI platform driver
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.625147] ehci-platform: EHCI generic platform driver
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.627751] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.630618] ohci-pci: OHCI PCI platform driver
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.632764] ohci-platform: OHCI generic platform driver
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.635045] uhci_hcd: USB Universal Host Controller Interface driver
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.638128] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.642139] i8042: Warning: Keylock active
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.644806] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.646961] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.649449] mousedev: PS/2 mouse device common for all mice
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.652576] rtc_cmos 00:00: RTC can wake from S4
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.654900] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.657673] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.660367] i2c /dev entries driver
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.662071] device-mapper: uevent: version 1.0.3
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.664228] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.667721] ledtrig-cpu: registered to indicate activity on CPUs
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.671438] NET: Registered protocol family 10
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.673976] NET: Registered protocol family 17
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.676110] Key type dns_resolver registered
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.678463] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.680972] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.683999] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.686530] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.689548] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.694106] registered taskstats version 1
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.696084] Loading compiled-in X.509 certificates
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.698802] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.703603] zswap: loaded using pool lzo/zbud
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.708422] Key type trusted registered
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.715474] Key type encrypted registered
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.717496] ima: No TPM chip found, activating TPM-bypass!
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.719889] evm: HMAC attrs: 0x1
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.721734]   Magic number: 14:59:30
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.723604] rtc_cmos 00:00: setting system clock to 2018-08-08 14:02:07 UTC (1533736927)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.728479] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.731158] EDD information not available.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.732965] PM: Hibernation image not present or could not be loaded.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.734461] Freeing unused kernel memory: 1496K
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.736543] Write protecting the kernel read-only data: 14336k
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.740128] Freeing unused kernel memory: 1956K
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.742411] Freeing unused kernel memory: 92K
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.759146] systemd-udevd[118]: starting version 204
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.819320] scsi host0: Virtio SCSI HBA
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.825357] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.833706] AVX2 version of gcm_enc/dec engaged.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.836184] AES CTR mode by8 optimization enabled
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.851361] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.880982] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.881025] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.881027] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.881420] sd 0:0:1:0: [sda] Write Protect is off
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.881422] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.881489] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.883002]  sda: sda1
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    3.883971] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    4.359187] tsc: Refined TSC clocksource calibration: 2300.001 MHz
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    4.360128] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735f0517, max_idle_ns: 440795237604 ns
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    4.688194] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    6.843219] floppy0: no floppy controllers found
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    7.995041] raid6: sse2x1   gen()  9005 MB/s
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.063050] raid6: sse2x1   xor()  6718 MB/s
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.131038] raid6: sse2x2   gen() 11130 MB/s
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.199055] raid6: sse2x2   xor()  7399 MB/s
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.267064] raid6: sse2x4   gen() 12995 MB/s
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.335073] raid6: sse2x4   xor()  9116 MB/s
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.403062] raid6: avx2x1   gen() 17299 MB/s
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.471069] raid6: avx2x2   gen() 20003 MB/s
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.539079] raid6: avx2x4   gen() 22374 MB/s
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.541533] raid6: using algorithm avx2x4 gen() 22374 MB/s
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.544371] raid6: using avx2x2 recovery algorithm
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.548044] xor: automatically using best checksumming function:
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.591127]    avx       : 26889.000 MB/sec
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.607450] Btrfs loaded
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.667007] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.670815] EXT4-fs (sda1): write access will be enabled during recovery
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.749439] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.759068] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.761266] EXT4-fs (sda1): recovery complete
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    8.768859] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    9.022898] random: init: uninitialized urandom read (12 bytes read, 23 bits of entropy available)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    9.163269] random: mountall: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    9.214685] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [    9.422052] random: cloud-init: uninitialized urandom read (32 bytes read, 33 bits of entropy available)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   10.049321] random: cloud-init: uninitialized urandom read (32 bytes read, 41 bits of entropy available)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   10.196072] systemd-udevd[701]: starting version 204
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   10.319286] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   10.382118] intel_rapl: no valid rapl domains found in package 0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   10.424681] intel_rapl: no valid rapl domains found in package 0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   10.468955] ppdev: user-space parallel port driver
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   10.545588] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   10.599626] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   10.669879] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   10.836357] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   11.105306] random: mktemp: uninitialized urandom read (12 bytes read, 56 bits of entropy available)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   11.184591] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   11.263933] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   11.309706] EXT4-fs (sda1): resized filesystem to 7864064
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   11.552733] init: failsafe main process (1093) killed by TERM signal
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO Running set_multiqueue.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO Set channels for eth0 to 4.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  8 14:02:15 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Starting Google Accounts daemon.
Aug  8 14:02:16 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Creating a new user account for me.
Aug  8 14:02:16 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Created user account me.
Aug  8 14:02:16 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Creating a new user account for henrik.
Aug  8 14:02:16 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Created user account henrik.
Aug  8 14:02:16 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Creating a new user account for emma.
Aug  8 14:02:16 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Created user account emma.
Aug  8 14:02:16 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Creating a new user account for igor.
Aug  8 14:02:16 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Created user account igor.
Aug  8 14:02:16 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  8 14:02:16 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Created user account konstantinhaase.
Aug  8 14:02:16 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Creating a new user account for aj.
Aug  8 14:02:16 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Created user account aj.
Aug  8 14:02:16 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Creating a new user account for solarce.
Aug  8 14:02:16 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Created user account solarce.
Aug  8 14:02:16 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Creating a new user account for asari.
Aug  8 14:02:16 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Created user account asari.
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-clock-skew: INFO Synced system time with hardware clock.
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Creating a new user account for bogdana.
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Created user account bogdana.
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Creating a new user account for konstantin.
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   12.883081] random: nonblocking pool is initialized
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Created user account konstantin.
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Creating a new user account for carmen.
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Created user account carmen.
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Creating a new user account for maria.
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   13.011181] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   13.014507] Bridge firewalling registered
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   13.025645] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Created user account maria.
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   13.059475] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 google-accounts: INFO Removing user packer.
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   13.130979] Initializing XFRM netlink socket
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   13.139808] Netfilter messages via NETLINK v0.30.
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   13.143384] ctnetlink v0.93: registering with nfnetlink.
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   13.403298] floppy0: no floppy controllers found
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 14:02:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 14:02:18 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 cron[1708]: (CRON) INFO (pidfile fd = 3)
Aug  8 14:02:18 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 cron[1740]: (CRON) STARTUP (fork ok)
Aug  8 14:02:18 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 cron[1740]: (CRON) INFO (Running @reboot jobs)
Aug  8 14:02:18 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 14:02:18 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 14:02:18 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 acpid: starting up with netlink and the input layer
Aug  8 14:02:18 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 acpid: 1 rule loaded
Aug  8 14:02:18 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 acpid: waiting for events: event logging is off
Aug  8 14:02:19 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 haveged: haveged starting up
Aug  8 14:02:19 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   14.917375] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpd[1842]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpd[1843]: proto: precision = 0.100 usec
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpd[1843]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpd[1843]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpd[1843]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpd[1843]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpd[1843]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpd[1843]: Listen normally on 3 eth0 10.20.2.154 UDP 123
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpd[1843]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpd[1843]: peers refreshed
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpd[1843]: Listening on routing socket on fd #21 for interface updates
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [   20.126298] init: plymouth-upstart-bridge main process ended, respawning
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 startup-script: INFO Found startup-script in metadata.
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 startup-script: INFO startup-script: job 1 at Wed Aug  8 17:12:00 2018
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 startup-script: INFO startup-script: Return code 0.
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 startup-script: INFO startup-script: Return code 0.
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 startup-script: INFO Finished running startup scripts.
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ec2: 
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ec2: #############################################################
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ec2: 1024 f4:3c:0d:1f:13:04:75:37:84:8f:69:59:c5:11:8a:44  root@travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 (DSA)
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ec2: 256 e6:cb:ab:3e:e9:fd:c7:58:ac:23:ef:2f:6e:b3:91:24  root@travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 (ECDSA)
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ec2: 256 7e:42:55:c4:68:81:64:60:7c:9c:a5:6b:40:96:80:7a  root@travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 (ED25519)
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ec2: 2048 a4:b7:0b:15:6f:e6:d9:df:7b:bf:de:e7:2d:9c:22:fc  root@travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 (RSA)
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  8 14:02:24 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ec2: #############################################################
Aug  8 14:02:32 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpdate[2246]: the NTP socket is in use, exiting
Aug  8 14:04:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [  133.630707] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  8 14:05:13 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [  189.678658] device vethfdd1586 entered promiscuous mode
Aug  8 14:05:14 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [  189.768312] cgroup: docker-runc (4852) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  8 14:05:14 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [  189.768315] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  8 14:05:14 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [  189.836009] eth0: renamed from veth11cf005
Aug  8 14:05:14 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [  189.874099] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  8 14:05:14 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [  189.875181] docker0: port 1(vethfdd1586) entered forwarding state
Aug  8 14:05:14 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [  189.875198] docker0: port 1(vethfdd1586) entered forwarding state
Aug  8 14:05:14 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [  189.875219] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  8 14:05:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpd[1843]: Listen normally on 5 docker0 fe80::42:dfff:feb1:ef6f UDP 123
Aug  8 14:05:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpd[1843]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  8 14:05:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpd[1843]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  8 14:05:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpd[1843]: peers refreshed
Aug  8 14:05:17 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 ntpd[1843]: new interface(s) found: waking up resolver
Aug  8 14:05:29 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 kernel: [  204.915973] docker0: port 1(vethfdd1586) entered forwarding state
Aug  8 14:17:01 travis-job-9edcc70b-40d5-45aa-965e-07bcacb4ef18 CRON[12369]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
---
travis_time:end:2ded52d0:start=1533740082006840371,finish=1533740082013727645,duration=6887274
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03acd730
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:086637db
travis_time:start:086637db
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0a83a060
$ dmesg | grep -i kill
