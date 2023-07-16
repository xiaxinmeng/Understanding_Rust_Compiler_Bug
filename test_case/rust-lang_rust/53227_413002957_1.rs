\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-size_of-cycle.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2017 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/consts/const-size_of-cycle.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2017 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which requires const-evaluating `Foo::bytes::{{constant}}`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/libcore/mem.rs","byte_start":10182,"byte_end":10208,"line_start":321,"line_end":321,"column_start":14,"column_end":40,"is_primary":true,"text":[{"text":"    unsafe { intrinsics::size_of::<T>() }","highlight_start":14,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires computing layout of `Foo`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when const-evaluating `Foo::bytes::{{constant}}`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/libcore/mem.rs","byte_start":10182,"byte_end":10208,"line_start":321,"line_end":321,"column_start":14,"column_end":40,"is_primary":true,"text":[{"text":"    unsafe { intrinsics::size_of::<T>() }","highlight_start":14,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when computing layout of `Foo`\n   |\nnote: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...\nnote: ...which requires const-evaluating `Foo::bytes::{{constant}}`...\n  --> /checkout/src/libcore/mem.rs:321:14\n   |\nLL |     unsafe { intrinsics::size_of::<T>() }\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = note: ...which again requires computing layout of `Foo`, completing the cycle\nnote: cycle used when const-evaluating `Foo::bytes::{{constant}}`\n  --> /checkout/src/libcore/mem.rs:321:14\n   |\nLL |     unsafe { intrinsics::size_of::<T>() }\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:51:16] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:16] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[00:51:16] ------------------------------------------
[00:51:16] 
[00:51:16] thread '[ui] ui/consts/const-size_of-cycle.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:51:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:51:16] 
[00:51:16] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:51:16] 
[00:51:16] 
[00:51:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:16] 
[00:51:16] 
[00:51:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:16] Build completed unsuccessfully in 0:03:14
[00:51:16] Build completed unsuccessfully in 0:03:14
[00:51:16] make: *** [check] Error 1
[00:51:16] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d8ba4fe
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:33c3d59c
$ sudo tail -n 500 /var/log/syslog
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] kvm-clock: using sched offset of 1594696351 cycles
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] Zone ranges:
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000]   Device   empty
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] Movable zone start for each node
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] Early memory node ranges
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] Policy zone: Normal
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] Hierarchical RCU implementation.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] console [ttyS0] enabled
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.000000] tsc: Detected 2299.998 MHz processor
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.307259] Calibrating delay loop (skipped) preset value.. 4599.99 BogoMIPS (lpj=9199992)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.308447] pid_max: default: 32768 minimum: 301
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.309113] ACPI: Core revision 20150930
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.315417] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.316458] Security Framework initialized
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.317023] Yama: becoming mindful.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.317604] AppArmor: AppArmor disabled by boot time parameter
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.320001] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.328873] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.333200] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.334426] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.335803] Initializing cgroup subsys io
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.336406] Initializing cgroup subsys memory
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.337017] Initializing cgroup subsys devices
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.337742] Initializing cgroup subsys freezer
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.338513] Initializing cgroup subsys net_cls
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.339219] Initializing cgroup subsys perf_event
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.340097] Initializing cgroup subsys net_prio
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.340831] Initializing cgroup subsys hugetlb
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.341471] Initializing cgroup subsys pids
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.342220] CPU: Physical Processor ID: 0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.342889] CPU: Processor Core ID: 0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.344248] mce: CPU supports 32 MCE banks
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.344976] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.345816] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.349191] Freeing SMP alternatives memory: 32K
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.359004] ftrace: allocating 32185 entries in 126 pages
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.412077] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.413138] smpboot: Max logical packages: 2
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.414295] x2apic enabled
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.416115] Switched APIC routing to physical x2apic.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.419598] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.526379] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.528085] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.530494] x86: Booting SMP configuration:
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.531155] .... node  #0, CPUs:      #1
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.532012] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.536282]  #2
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.536813] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.541925]  #3
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.542432] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.546497] x86: Booted up 1 node, 4 CPUs
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.547170] smpboot: Total of 4 processors activated (18399.98 BogoMIPS)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.549385] devtmpfs: initialized
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.553601] evm: security.selinux
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.554153] evm: security.SMACK64
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.554694] evm: security.SMACK64EXEC
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.555361] evm: security.SMACK64TRANSMUTE
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.555945] evm: security.SMACK64MMAP
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.556530] evm: security.ima
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.557040] evm: security.capability
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.558067] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.559623] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.560727] pinctrl core: initialized pinctrl subsystem
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.561720] RTC time: 19:27:26, date: 08/14/18
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.563297] NET: Registered protocol family 16
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.574417] cpuidle: using governor ladder
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.586409] cpuidle: using governor menu
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.587279] PCCT header not found.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.587861] ACPI: bus type PCI registered
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.588418] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.589449] PCI: Using configuration type 1 for base access
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.603319] ACPI: Added _OSI(Module Device)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.604018] ACPI: Added _OSI(Processor Device)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.604656] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.605316] ACPI: Added _OSI(Processor Aggregator Device)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.608589] ACPI: Executed 2 blocks of module-level executable AML code
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.631295] ACPI: Interpreter enabled
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.631992] ACPI: (supports S0 S3 S4 S5)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.632644] ACPI: Using IOAPIC for interrupt routing
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.633420] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.662319] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.663357] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.664407] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.665514] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.667784] PCI host bridge to bus 0000:00
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.668483] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.669557] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.670528] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.671855] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.673018] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.673834] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.674251] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.687703] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.702324] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.703853] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.709548] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.713754] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.725269] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.731077] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.735298] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.750437] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.753835] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.756458] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.758946] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.761539] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.781880] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.783236] vgaarb: loaded
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.784327] SCSI subsystem initialized
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.785154] libata version 3.00 loaded.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.785190] ACPI: bus type USB registered
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.786077] usbcore: registered new interface driver usbfs
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.786997] usbcore: registered new interface driver hub
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.788147] usbcore: registered new device driver usb
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.789144] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.790407] dmi: Firmware registration failed.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.791526] PCI: Using ACPI for IRQ routing
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.792282] PCI: pci_cache_line_size set to 64 bytes
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.792385] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.792387] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.792526] NetLabel: Initializing
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.793035] NetLabel:  domain hash size = 128
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.793723] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.794545] NetLabel:  unlabeled traffic allowed by default
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.795603] amd_nb: Cannot enumerate AMD northbridges
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.796670] clocksource: Switched to clocksource kvm-clock
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.804211] pnp: PnP ACPI init
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.804857] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.804925] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.804976] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.805027] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.805070] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.805112] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.805154] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.805314] pnp: PnP ACPI: found 7 devices
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.812800] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.814236] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.814239] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.814240] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.814242] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.814275] NET: Registered protocol family 2
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.815267] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.817263] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.818567] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.819898] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.820843] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.822599] NET: Registered protocol family 1
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.823314] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.824208] PCI: CLS 0 bytes, default 64
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    0.824259] Unpacking initramfs...
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.863933] Freeing initrd memory: 21432K
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.866386] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.868127] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.871203] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.872722] hw unit of domain pp0-core 2^-0 Joules
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.874272] hw unit of domain package 2^-0 Joules
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.875316] hw unit of domain dram 2^-16 Joules
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.876202] Scanning for low memory corruption every 60 seconds
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.877990] audit: initializing netlink subsys (disabled)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.878802] audit: type=2000 audit(1534274848.577:1): initialized
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.880094] Initialise system trusted keyring
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.881452] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.882591] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.885010] zbud: loaded
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.886008] VFS: Disk quotas dquot_6.6.0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.886653] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.888949] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.892740] fuse init (API version 7.23)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.894175] Key type big_key registered
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.895011] Allocating IMA MOK and blacklist keyrings.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.897344] Key type asymmetric registered
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.898244] Asymmetric key parser 'x509' registered
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.899189] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.900554] io scheduler noop registered
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.901300] io scheduler deadline registered (default)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.902245] io scheduler cfq registered
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.903080] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.904154] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.905438] intel_idle: does not run on family 6 model 63
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.905546] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.907226] ACPI: Power Button [PWRF]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.907866] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.911951] ACPI: Sleep Button [SLPF]
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.913454] GHES: HEST is not enabled!
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.916481] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.919216] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.926375] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.927474] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.932851] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.955885] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    2.980721] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.004844] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.029243] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.035576] Linux agpgart interface v0.103
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.040138] loop: module loaded
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.041533] libphy: Fixed MDIO Bus: probed
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.043153] tun: Universal TUN/TAP device driver, 1.6
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.044473] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.080999] PPP generic driver version 2.4.2
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.082466] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.084541] ehci-pci: EHCI PCI platform driver
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.086076] ehci-platform: EHCI generic platform driver
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.087880] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.089778] ohci-pci: OHCI PCI platform driver
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.091081] ohci-platform: OHCI generic platform driver
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.092782] uhci_hcd: USB Universal Host Controller Interface driver
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.094708] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.098190] i8042: Warning: Keylock active
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.100483] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.101820] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.103501] mousedev: PS/2 mouse device common for all mice
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.105731] rtc_cmos 00:00: RTC can wake from S4
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.107994] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.110710] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.112686] i2c /dev entries driver
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.113936] device-mapper: uevent: version 1.0.3
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.116046] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.119365] ledtrig-cpu: registered to indicate activity on CPUs
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.121883] NET: Registered protocol family 10
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.123492] NET: Registered protocol family 17
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.124929] Key type dns_resolver registered
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.126913] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.128289] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.130100] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.131785] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.133641] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.136455] registered taskstats version 1
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.137570] Loading compiled-in X.509 certificates
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.139950] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.143365] zswap: loaded using pool lzo/zbud
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.146763] Key type trusted registered
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.151810] Key type encrypted registered
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.153027] ima: No TPM chip found, activating TPM-bypass!
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.155236] evm: HMAC attrs: 0x1
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.156688]   Magic number: 14:7:496
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.158320] rtc_cmos 00:00: setting system clock to 2018-08-14 19:27:29 UTC (1534274849)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.161520] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.163049] EDD information not available.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.164487] PM: Hibernation image not present or could not be loaded.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.165877] Freeing unused kernel memory: 1496K
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.166735] Write protecting the kernel read-only data: 14336k
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.169042] Freeing unused kernel memory: 1956K
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.170450] Freeing unused kernel memory: 92K
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.187015] systemd-udevd[118]: starting version 204
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.246695] scsi host0: Virtio SCSI HBA
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.250701] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.260364] AVX2 version of gcm_enc/dec engaged.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.262000] AES CTR mode by8 optimization enabled
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.304371] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.304375] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.305050] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.308326] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.309434] sd 0:0:1:0: [sda] Write Protect is off
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.310097] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.310206] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.313207]  sda: sda1
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.314815] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.872839] tsc: Refined TSC clocksource calibration: 2299.999 MHz
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    3.873875] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2127345424d, max_idle_ns: 440795318347 ns
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    4.137852] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    6.240885] floppy0: no floppy controllers found
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    7.400708] raid6: sse2x1   gen()  8649 MB/s
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    7.468712] raid6: sse2x1   xor()  6752 MB/s
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    7.536693] raid6: sse2x2   gen() 11184 MB/s
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    7.604730] raid6: sse2x2   xor()  7486 MB/s
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    7.672701] raid6: sse2x4   gen() 12694 MB/s
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    7.740698] raid6: sse2x4   xor()  8876 MB/s
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    7.808797] raid6: avx2x1   gen() 16795 MB/s
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    7.876713] raid6: avx2x2   gen() 19041 MB/s
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    7.944712] raid6: avx2x4   gen() 23095 MB/s
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    7.945935] raid6: using algorithm avx2x4 gen() 23095 MB/s
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    7.947150] raid6: using avx2x2 recovery algorithm
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    7.949807] xor: automatically using best checksumming function:
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    7.988704]    avx       : 27447.000 MB/sec
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    8.003095] Btrfs loaded
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    8.049962] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    8.051930] EXT4-fs (sda1): write access will be enabled during recovery
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    8.116242] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    8.123241] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    8.124327] EXT4-fs (sda1): recovery complete
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    8.130100] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    8.350510] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    8.473823] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    8.530413] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    8.729497] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    9.314890] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    9.455701] systemd-udevd[703]: starting version 204
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    9.563997] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    9.611006] intel_rapl: no valid rapl domains found in package 0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    9.672304] ppdev: user-space parallel port driver
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    9.757176] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    9.808776] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [    9.872366] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   10.036810] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   10.336161] random: mktemp: uninitialized urandom read (12 bytes read, 58 bits of entropy available)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   10.410491] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   10.483748] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   10.523423] EXT4-fs (sda1): resized filesystem to 7864064
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   10.939348] init: failsafe main process (1095) killed by TERM signal
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO Running set_multiqueue.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO Set channels for eth0 to 4.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 19:27:37 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Starting Google Accounts daemon.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Creating a new user account for me.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   11.751142] random: nonblocking pool is initialized
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Created user account me.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Creating a new user account for henrik.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Created user account henrik.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Creating a new user account for emma.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Created user account emma.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Creating a new user account for igor.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Created user account igor.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-clock-skew: INFO Synced system time with hardware clock.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Created user account konstantinhaase.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf cron[1402]: (CRON) INFO (pidfile fd = 3)
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf cron[1468]: (CRON) STARTUP (fork ok)
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf cron[1468]: (CRON) INFO (Running @reboot jobs)
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Creating a new user account for aj.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf acpid: starting up with netlink and the input layer
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf acpid: 1 rule loaded
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf acpid: waiting for events: event logging is off
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Created user account aj.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Creating a new user account for solarce.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf haveged: haveged starting up
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Created user account solarce.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Creating a new user account for asari.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Created user account asari.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   12.248884] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   12.263506] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Creating a new user account for bogdana.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Created user account bogdana.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Creating a new user account for konstantin.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Created user account konstantin.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   12.367492] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   12.372796] Bridge firewalling registered
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   12.381298] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Creating a new user account for carmen.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Created user account carmen.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Creating a new user account for maria.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   12.468920] Initializing XFRM netlink socket
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   12.476760] Netfilter messages via NETLINK v0.30.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   12.480022] ctnetlink v0.93: registering with nfnetlink.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Created user account maria.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf google-accounts: INFO Removing user packer.
Aug 14 19:27:38 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   12.632802] floppy0: no floppy controllers found
Aug 14 19:28:01 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpdate[1859]: adjust time server 169.254.169.254 offset 0.015963 sec
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpd[1894]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpd[1895]: proto: precision = 0.103 usec
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpd[1895]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpd[1895]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpd[1895]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpd[1895]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpd[1895]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpd[1895]: Listen normally on 3 eth0 10.20.2.104 UDP 123
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpd[1895]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpd[1895]: peers refreshed
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpd[1895]: Listening on routing socket on fd #21 for interface updates
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [   42.450415] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf startup-script: INFO Found startup-script in metadata.
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf startup-script: INFO startup-script: job 1 at Tue Aug 14 22:38:00 2018
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf startup-script: INFO startup-script: Return code 0.
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf startup-script: INFO startup-script: Return code 0.
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf startup-script: INFO Finished running startup scripts.
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ec2: 
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ec2: #############################################################
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ec2: 1024 dc:aa:54:0e:b0:2c:08:27:9f:fc:48:f6:48:a5:84:a3  root@travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf (DSA)
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ec2: 256 b9:41:98:ff:44:7e:5f:08:05:7f:4b:49:f3:e1:c1:cf  root@travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf (ECDSA)
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ec2: 256 e4:29:0f:cd:f1:13:fd:f8:ff:0e:a9:54:2c:71:e8:d4  root@travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf (ED25519)
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ec2: 2048 4b:52:1c:a2:81:11:21:83:d0:02:9d:8b:60:aa:5b:f0  root@travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf (RSA)
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 14 19:28:08 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ec2: #############################################################
Aug 14 19:30:07 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [  161.070335] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 14 19:31:02 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [  216.040064] device veth233fbae entered promiscuous mode
Aug 14 19:31:02 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [  216.147391] cgroup: docker-runc (4887) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 19:31:02 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [  216.147394] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 19:31:02 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [  216.214853] eth0: renamed from veth2664a4b
Aug 14 19:31:02 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [  216.263900] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 19:31:02 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [  216.264834] docker0: port 1(veth233fbae) entered forwarding state
Aug 14 19:31:02 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [  216.264846] docker0: port 1(veth233fbae) entered forwarding state
Aug 14 19:31:02 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [  216.264864] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 14 19:31:05 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpd[1895]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 14 19:31:05 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpd[1895]: Listen normally on 6 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 14 19:31:05 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpd[1895]: Listen normally on 7 docker0 fe80::42:3eff:fe0e:20e2 UDP 123
Aug 14 19:31:05 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpd[1895]: peers refreshed
Aug 14 19:31:05 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf ntpd[1895]: new interface(s) found: waking up resolver
Aug 14 19:31:17 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf kernel: [  231.306207] docker0: port 1(veth233fbae) entered forwarding state
Aug 14 20:17:01 travis-job-c4db35b2-d5b9-4b25-96b3-3b250d69dadf CRON[21440]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
---
travis_time:end:00182a48:start=1534278087536720848,finish=1534278087542636034,duration=5915186
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e70dbd4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02e6b078
travis_time:start:02e6b078
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0b8da45c
$ dmesg | grep -i kill
