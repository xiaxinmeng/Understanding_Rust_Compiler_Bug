\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/empty/empty-extern-arg.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2014 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the `i586-unknown-linux-gnu` target may not be installed","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0463]: can't find crate for `std`\n   |\n   = note: the `i586-unknown-linux-gnu` target may not be installed\n\n"}
[00:46:10] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:46:10] {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
[00:46:10] ------------------------------------------
[00:46:10] 
[00:46:10] thread '[ui] ui/empty/empty-extern-arg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:46:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:46:10] 
[00:46:10] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:46:10] 
[00:46:10] 
[00:46:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "7.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:10] 
[00:46:10] 
[00:46:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:46:10] Build completed unsuccessfully in 0:43:04
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:1098e0cb
$ sudo tail -n 500 /var/log/syslog
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] kvm-clock: using sched offset of 2450823842 cycles
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] Zone ranges:
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000]   Device   empty
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] Movable zone start for each node
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] Early memory node ranges
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] Policy zone: Normal
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] Hierarchical RCU implementation.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] console [ttyS0] enabled
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.471599] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.475368] pid_max: default: 32768 minimum: 301
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.477056] ACPI: Core revision 20150930
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.484813] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.487201] Security Framework initialized
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.488607] Yama: becoming mindful.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.489948] AppArmor: AppArmor disabled by boot time parameter
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.493991] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.505225] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.512047] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.515032] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.518189] Initializing cgroup subsys io
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.519899] Initializing cgroup subsys memory
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.521302] Initializing cgroup subsys devices
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.522887] Initializing cgroup subsys freezer
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.524717] Initializing cgroup subsys net_cls
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.526699] Initializing cgroup subsys perf_event
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.528330] Initializing cgroup subsys net_prio
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.529651] Initializing cgroup subsys hugetlb
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.531105] Initializing cgroup subsys pids
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.532639] CPU: Physical Processor ID: 0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.533903] CPU: Processor Core ID: 0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.535221] mce: CPU supports 32 MCE banks
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.536777] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.538945] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.543261] Freeing SMP alternatives memory: 32K
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.555096] ftrace: allocating 32185 entries in 126 pages
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.614795] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.617016] smpboot: Max logical packages: 2
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.618933] x2apic enabled
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.621475] Switched APIC routing to physical x2apic.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.627006] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.737799] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.740869] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.746022] x86: Booting SMP configuration:
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.747557] .... node  #0, CPUs:      #1
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.749205] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.753736]  #2
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.754826] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.759826]  #3
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.760574] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.765324] x86: Booted up 1 node, 4 CPUs
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.767059] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.770747] devtmpfs: initialized
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.776659] evm: security.selinux
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.778085] evm: security.SMACK64
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.779328] evm: security.SMACK64EXEC
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.780422] evm: security.SMACK64TRANSMUTE
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.782358] evm: security.SMACK64MMAP
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.783464] evm: security.ima
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.784733] evm: security.capability
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.787788] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.791129] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.793941] pinctrl core: initialized pinctrl subsystem
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.796374] RTC time: 22:52:15, date: 08/10/18
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.798893] NET: Registered protocol family 16
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.809866] cpuidle: using governor ladder
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.821856] cpuidle: using governor menu
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.823330] PCCT header not found.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.824710] ACPI: bus type PCI registered
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.826179] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.828576] PCI: Using configuration type 1 for base access
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.843483] ACPI: Added _OSI(Module Device)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.845352] ACPI: Added _OSI(Processor Device)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.847862] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.849971] ACPI: Added _OSI(Processor Aggregator Device)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.854993] ACPI: Executed 2 blocks of module-level executable AML code
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.883046] ACPI: Interpreter enabled
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.884232] ACPI: (supports S0 S3 S4 S5)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.885606] ACPI: Using IOAPIC for interrupt routing
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.887549] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.920514] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.923328] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.926261] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.928867] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.933267] PCI host bridge to bus 0000:00
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.935352] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.937666] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.939945] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.942763] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.945441] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.947557] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.948081] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    0.973348] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.002148] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.005699] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.018386] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.028441] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.053164] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.071118] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.082612] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.107535] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.112428] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.119996] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.128738] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.136050] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.160130] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.162923] vgaarb: loaded
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.164529] SCSI subsystem initialized
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.166985] libata version 3.00 loaded.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.167015] ACPI: bus type USB registered
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.168658] usbcore: registered new interface driver usbfs
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.170958] usbcore: registered new interface driver hub
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.173624] usbcore: registered new device driver usb
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.175771] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.179284] dmi: Firmware registration failed.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.181815] PCI: Using ACPI for IRQ routing
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.184575] PCI: pci_cache_line_size set to 64 bytes
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.184686] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.184689] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.184823] NetLabel: Initializing
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.186584] NetLabel:  domain hash size = 128
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.189215] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.191272] NetLabel:  unlabeled traffic allowed by default
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.195020] amd_nb: Cannot enumerate AMD northbridges
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.198184] clocksource: Switched to clocksource kvm-clock
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.207874] pnp: PnP ACPI init
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.210304] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.210373] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.210418] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.210470] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.210517] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.210588] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.210632] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.210807] pnp: PnP ACPI: found 7 devices
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.221130] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.225466] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.225469] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.225471] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.225473] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.225509] NET: Registered protocol family 2
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.227777] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.231793] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.235200] TCP: Hash tables configured (established 131072 bind 65536)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.240611] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.244742] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.249497] NET: Registered protocol family 1
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.251575] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.254614] PCI: CLS 0 bytes, default 64
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    1.254678] Unpacking initramfs...
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.426719] Freeing initrd memory: 21432K
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.428343] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.430734] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.434838] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.438784] hw unit of domain pp0-core 2^-0 Joules
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.440421] hw unit of domain package 2^-0 Joules
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.442366] hw unit of domain dram 2^-0 Joules
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.444049] Scanning for low memory corruption every 60 seconds
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.446605] audit: initializing netlink subsys (disabled)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.448860] audit: type=2000 audit(1533941537.763:1): initialized
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.451131] Initialise system trusted keyring
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.453197] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.455326] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.460296] zbud: loaded
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.461788] VFS: Disk quotas dquot_6.6.0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.463493] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.466045] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.468728] fuse init (API version 7.23)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.470272] Key type big_key registered
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.471505] Allocating IMA MOK and blacklist keyrings.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.481615] Key type asymmetric registered
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.483476] Asymmetric key parser 'x509' registered
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.485708] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.489281] io scheduler noop registered
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.491614] io scheduler deadline registered (default)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.494232] io scheduler cfq registered
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.496180] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.498076] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.501288] intel_idle: does not run on family 6 model 45
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.501418] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.504156] ACPI: Power Button [PWRF]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.505327] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.507953] ACPI: Sleep Button [SLPF]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.510327] GHES: HEST is not enabled!
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.514483] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.516695] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.527124] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.529643] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.539622] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.563771] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.587969] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.613384] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.637835] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.644050] Linux agpgart interface v0.103
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.650153] loop: module loaded
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.651947] libphy: Fixed MDIO Bus: probed
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.653371] tun: Universal TUN/TAP device driver, 1.6
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.655007] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.712778] PPP generic driver version 2.4.2
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.715123] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.717555] ehci-pci: EHCI PCI platform driver
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.720117] ehci-platform: EHCI generic platform driver
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.721861] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.724788] ohci-pci: OHCI PCI platform driver
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.726483] ohci-platform: OHCI generic platform driver
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.728488] uhci_hcd: USB Universal Host Controller Interface driver
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.731408] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.734910] i8042: Warning: Keylock active
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.737709] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.740763] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.743706] mousedev: PS/2 mouse device common for all mice
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.746791] rtc_cmos 00:00: RTC can wake from S4
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.748645] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.751386] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.753304] i2c /dev entries driver
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.754821] device-mapper: uevent: version 1.0.3
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.756856] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.760621] ledtrig-cpu: registered to indicate activity on CPUs
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.764471] NET: Registered protocol family 10
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.766731] NET: Registered protocol family 17
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.768407] Key type dns_resolver registered
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.770996] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.774869] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.778237] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.781276] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.783536] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.791623] registered taskstats version 1
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.793430] Loading compiled-in X.509 certificates
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.796302] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.799704] zswap: loaded using pool lzo/zbud
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.803968] Key type trusted registered
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.809799] Key type encrypted registered
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.812174] ima: No TPM chip found, activating TPM-bypass!
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.814340] evm: HMAC attrs: 0x1
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.816163]   Magic number: 14:437:906
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.817975] rtc_cmos 00:00: setting system clock to 2018-08-10 22:52:18 UTC (1533941538)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.821370] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.823571] EDD information not available.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.825076] PM: Hibernation image not present or could not be loaded.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.826688] Freeing unused kernel memory: 1496K
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.828659] Write protecting the kernel read-only data: 14336k
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.831435] Freeing unused kernel memory: 1956K
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.833682] Freeing unused kernel memory: 92K
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.852214] systemd-udevd[119]: starting version 204
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.931179] scsi host0: Virtio SCSI HBA
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.940040] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.944469] AVX version of gcm_enc/dec engaged.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.946238] AES CTR mode by8 optimization enabled
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.949102] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.990286] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.992410] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.995450] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.997817] sd 0:0:1:0: [sda] Write Protect is off
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.999495] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    3.999758] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    4.005491]  sda: sda1
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    4.007576] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    4.442346] tsc: Refined TSC clocksource calibration: 2599.769 MHz
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    4.444679] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257961c8102, max_idle_ns: 440795256056 ns
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    4.792647] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    6.902413] floppy0: no floppy controllers found
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.058206] raid6: sse2x1   gen()  9151 MB/s
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.126197] raid6: sse2x1   xor()  6885 MB/s
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.194201] raid6: sse2x2   gen() 11413 MB/s
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.262200] raid6: sse2x2   xor()  7729 MB/s
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.330205] raid6: sse2x4   gen() 12541 MB/s
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.398200] raid6: sse2x4   xor()  8536 MB/s
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.399039] raid6: using algorithm sse2x4 gen() 12541 MB/s
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.399969] raid6: .... xor() 8536 MB/s, rmw enabled
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.400927] raid6: using ssse3x2 recovery algorithm
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.403304] xor: automatically using best checksumming function:
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.442198]    avx       : 22136.000 MB/sec
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.457177] Btrfs loaded
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.501609] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.502776] EXT4-fs (sda1): write access will be enabled during recovery
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.572026] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.577621] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.578502] EXT4-fs (sda1): recovery complete
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.583348] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.776245] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.876528] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    8.921328] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    9.100964] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    9.621052] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    9.743091] systemd-udevd[701]: starting version 204
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    9.857608] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    9.922730] intel_rapl: no valid rapl domains found in package 0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [    9.968965] ppdev: user-space parallel port driver
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   10.070359] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   10.115076] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   10.175533] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   10.332500] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   10.564555] random: mktemp: uninitialized urandom read (12 bytes read, 59 bits of entropy available)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   10.634621] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   10.703334] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   10.735732] EXT4-fs (sda1): resized filesystem to 7864064
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   11.018253] init: failsafe main process (1093) killed by TERM signal
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO Running set_multiqueue.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO Set channels for eth0 to 4.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 10 22:52:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d cron[1347]: (CRON) INFO (pidfile fd = 3)
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d cron[1386]: (CRON) STARTUP (fork ok)
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d cron[1386]: (CRON) INFO (Running @reboot jobs)
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d acpid: starting up with netlink and the input layer
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d acpid: 1 rule loaded
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d acpid: waiting for events: event logging is off
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   11.824511] random: nonblocking pool is initialized
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Starting Google Accounts daemon.
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Creating a new user account for me.
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Created user account me.
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Creating a new user account for henrik.
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Created user account henrik.
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Creating a new user account for emma.
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Created user account emma.
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Creating a new user account for igor.
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Created user account igor.
Aug 10 22:52:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-clock-skew: INFO Synced system time with hardware clock.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Created user account konstantinhaase.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Creating a new user account for aj.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d haveged: haveged starting up
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Created user account aj.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   12.410385] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   12.413657] Bridge firewalling registered
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Creating a new user account for solarce.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   12.428820] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   12.443273] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   12.459899] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Created user account solarce.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Creating a new user account for asari.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Created user account asari.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Creating a new user account for bogdana.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Created user account bogdana.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Creating a new user account for konstantin.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Created user account konstantin.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Creating a new user account for carmen.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Created user account carmen.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Creating a new user account for maria.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Created user account maria.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d google-accounts: INFO Removing user packer.
Aug 10 22:52:27 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   12.942420] floppy0: no floppy controllers found
Aug 10 22:52:28 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   13.508829] Initializing XFRM netlink socket
Aug 10 22:52:28 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   13.516379] Netfilter messages via NETLINK v0.30.
Aug 10 22:52:28 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   13.518906] ctnetlink v0.93: registering with nfnetlink.
Aug 10 22:52:49 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpdate[1858]: adjust time server 169.254.169.254 offset 0.007891 sec
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpd[1895]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpd[1896]: proto: precision = 0.104 usec
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpd[1896]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpd[1896]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpd[1896]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpd[1896]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpd[1896]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpd[1896]: Listen normally on 3 eth0 10.20.2.61 UDP 123
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpd[1896]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpd[1896]: peers refreshed
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpd[1896]: Listening on routing socket on fd #21 for interface updates
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   42.649613] init: plymouth-upstart-bridge main process ended, respawning
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d startup-script: INFO Found startup-script in metadata.
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d startup-script: INFO startup-script: job 1 at Sat Aug 11 02:02:00 2018
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d startup-script: INFO startup-script: Return code 0.
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d startup-script: INFO startup-script: Return code 0.
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d startup-script: INFO Finished running startup scripts.
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ec2: 
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ec2: #############################################################
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ec2: 1024 13:07:ec:b5:53:48:af:de:2b:28:d0:5a:a5:42:14:72  root@travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d (DSA)
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ec2: 256 ba:ca:22:d4:02:3e:80:fc:20:18:e5:fd:6d:e2:4b:96  root@travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d (ECDSA)
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ec2: 256 c0:28:db:ae:7f:1c:ba:f8:4e:84:3f:51:2f:4b:e6:90  root@travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d (ED25519)
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ec2: 2048 3a:4f:13:e7:cb:87:c0:ff:e8:f8:cb:5a:3c:a1:4f:83  root@travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d (RSA)
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 10 22:52:57 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ec2: #############################################################
Aug 10 22:53:26 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [   72.153310] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 10 22:54:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [  130.553982] device veth11ad513 entered promiscuous mode
Aug 10 22:54:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [  130.554099] docker0: port 1(veth11ad513) entered forwarding state
Aug 10 22:54:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [  130.554110] docker0: port 1(veth11ad513) entered forwarding state
Aug 10 22:54:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [  130.554551] docker0: port 1(veth11ad513) entered disabled state
Aug 10 22:54:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [  130.643221] cgroup: docker-runc (4889) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 10 22:54:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [  130.643224] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 10 22:54:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [  130.718798] eth0: renamed from vetha2db9f3
Aug 10 22:54:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [  130.755644] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 10 22:54:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [  130.756821] docker0: port 1(veth11ad513) entered forwarding state
Aug 10 22:54:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [  130.756840] docker0: port 1(veth11ad513) entered forwarding state
Aug 10 22:54:25 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [  130.756868] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 10 22:54:29 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpd[1896]: Listen normally on 5 docker0 fe80::42:ffff:feb5:5945 UDP 123
Aug 10 22:54:29 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpd[1896]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 10 22:54:29 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpd[1896]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 10 22:54:29 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpd[1896]: peers refreshed
Aug 10 22:54:29 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d ntpd[1896]: new interface(s) found: waking up resolver
Aug 10 22:54:40 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d kernel: [  145.786315] docker0: port 1(veth11ad513) entered forwarding state
Aug 10 23:17:01 travis-job-3ac2236a-ef8c-430a-ba47-2aabcd7bfa0d CRON[400]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
---
travis_time:end:376abdd6:start=1533944379516280814,finish=1533944379524341338,duration=8060524
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0042a4e1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:030783c9
travis_time:start:030783c9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:026eccdb
$ dmesg | grep -i kill
