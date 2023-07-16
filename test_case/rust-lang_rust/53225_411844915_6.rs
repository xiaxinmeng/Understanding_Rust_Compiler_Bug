\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/adt-nullary-enums.rs","byte_start":1820,"byte_end":1822,"line_start":65,"line_end":65,"column_start":45,"column_end":47,"is_primary":true,"text":[{"text":"            SomeEnum::SomeVariant(Cell::new(&c)),","highlight_start":45,"highlight_end":47}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/adt-nullary-enums.rs","byte_start":1898,"byte_end":1899,"line_start":68,"line_end":68,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    };","highlight_start":5,"highlight_end":6}],"label":"`c` dropped here while still borrowed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the lifetime 'a as defined on the function body at 61:46...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/adt-nullary-enums.rs","byte_start":1687,"byte_end":1689,"line_start":61,"line_end":61,"column_start":46,"column_end":48,"is_primary":true,"text":[{"text":"fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {","highlight_start":46,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0597]: `c` does not live long enough\n  --> /checkout/src/test/ui/nll/user-annotations/adt-nullary-enums.rs:65:45\n   |\nLL |             SomeEnum::SomeVariant(Cell::new(&c)),\n   |                                             ^^ borrowed value does not live long enough\n...\nLL |     };\n   |     - `c` dropped here while still borrowed\n   |\nnote: borrowed value must be valid for the lifetime 'a as defined on the function body at 61:46...\n  --> /checkout/src/test/ui/nll/user-annotations/adt-nullary-enums.rs:61:46\n   |\nLL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {\n   |                                              ^^\n\n"}
[00:47:02] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:47:02] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:47:02] ------------------------------------------
[00:47:02] 
[00:47:02] 
[00:47:02] thread '[ui] ui/nll/user-annotations/adt-nullary-enums.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:47:02] 
[00:47:02] 
[00:47:02] failures:
[00:47:02] failures:
[00:47:02]     [ui] ui/nll/user-annotations/adt-nullary-enums.rs
[00:47:02] test result: FAILED. 2253 passed; 1 failed; 7 ignored; 0 measured; 0 filtered out
[00:47:02] 
[00:47:02] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:47:02] 
[00:47:02] 
[00:47:02] 
[00:47:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:02] 
[00:47:02] 
[00:47:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:02] Build completed unsuccessfully in 0:02:14
[00:47:02] Build completed unsuccessfully in 0:02:14
[00:47:02] Makefile:58: recipe for target 'check' failed
[00:47:02] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05675ec4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:027c96f6
$ sudo tail -n 500 /var/log/syslog
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] kvm-clock: using sched offset of 1513443734 cycles
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] Zone ranges:
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000]   Device   empty
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] Movable zone start for each node
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] Early memory node ranges
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 um: 301
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.332565] ACPI: Core revision 20150930
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.339794] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.341403] Security Framework initialized
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.342031] Yama: becoming mindful.
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.342788] AppArmor: AppArmor disabled by boot time parameter
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.345637] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.355608] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.360862] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.362837] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.364154] Initializing cgroup subsys io
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.364735] Initializing cgroup subsys memory
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.365522] Initializing cgroup subsys devices
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.366249] Initializing cgroup subsys freezer
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.366896] Initializing cgroup subsys net_cls
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.367637] Initializing cgroup subsys perf_event
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.368302] Initializing cgroup subsys net_prio
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.369324] Initializing cgroup subsys hugetlb
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.370065] Initializing cgroup subsys pids
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.370789] CPU: Physical Processor ID: 0
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.371400] CPU: Processor Core ID: 0
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.373302] mce: CPU supports 32 MCE banks
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.374284] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.375638] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.378486] Freeing SMP alternatives memory: 32K
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.387275] ftrace: allocating 32185 entries in 126 pages
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.435982] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.437563] smpboot: Max logical packages: 2
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.438981] x2apic enabled
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.440980] Switched APIC routing to physical x2apic.
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.444855] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.553058] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.556080] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.560039] x86: Booting SMP configuration:
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.561523] .... node  #0, CPUs:      #1
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.562457] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.566856]  #2
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.567354] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.571776]  #3
Aug  9 17:09: resource [io  0x0d00-0xffff window]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.703568] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.705877] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.708799] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.710592] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.711066] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.730355] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.749937] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.751503] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.758321] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.762918] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.778302] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.784965] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.790158] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.806965] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.809690] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.812892] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.815678] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.818237] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.839062] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.841067] vgaarb: loaded
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.841887] SCSI subsystem initialized
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.842708] libata version 3.00 loaded.
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.842731] ACPI: bus type USB registered
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.843564] usbcore: registered new interface driver usbfs
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.844603] usbcore: registered new interface driver hub
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.845607] usbcore: registered new device driver usb
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.846982] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.848158] dmi: Firmware registration failed.
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.849155] PCI: Using ACPI for IRQ routing
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.850207] PCI: pci_cache_line_size set to 64 bytes
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.850315] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.850317] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.850491] NetLabel: Initializing
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.850977] NetLabel:  domain hash size = 128
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.851592] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.852504] NetLabel:  unlabeled traffic allowed by default
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.853787] amd_nb: Cannot enumerate AMD northbridges
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.854728] clocksource: Switched to clocksource kvm-clock
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.863486] pnp: PnP ACPI init
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.864226] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.864299] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.864343] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.864395] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.864436] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.864476] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.864516] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.864684] pnp: PnP ACPI: found 7 devices
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.872760] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.875139] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.875141] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.875143] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.875145] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.875216] NET: Registered protocol family 2
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.876272] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.877910] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.879113] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.880553] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.881977] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.882996] NET: Registered protocol family 1
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.883732] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.884661] PCI: CLS 0 bytes, default 64
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    0.885463] Unpacking initramfs...
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.037282] Freeing initrd memory: 21432K
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.040269] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.044915] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.048193] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.051141] hw unit of domain pp0-core 2^-0 Joules
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.053348] hw unit of domain package 2^-0 Joules
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.055668] hw unit of domain dram 2^-0 Joules
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.057467] Scanning for low memory corruption every 60 seconds
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.060293] audit: initializing netlink subsys (disabled)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.061908] audit: type=2000 audit(1533834544.663:1): initialized
7:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.261272] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.320636] PPP generic driver version 2.4.2
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.322966] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.326457] ehci-pci: EHCI PCI platform driver
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.328827] ehci-platform: EHCI generic platform driver
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.330894] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.334266] ohci-pci: OHCI PCI platform driver
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.336089] ohci-platform: OHCI generic platform driver
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.338290] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.341082] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.344823] i8042: Warning: Keylock active
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.347587] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.349770] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.352390] mousedev: PS/2 mouse device common for all mice
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.355324] rtc_cmos 00:00: RTC can wake from S4
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.357645] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.360224] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.363316] i2c /dev entries driver
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.365752] device-mapper: uevent: version 1.0.3
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.368630] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.372156] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.376502] NET: Registered protocol family 10
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.379561] NET: Registered protocol family 17
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.382409] Key type dns_resolver registered
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.385313] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.388302] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.391621] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.393590] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.395780] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.401072] registered taskstats version 1
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.402689] Loading compiled-in X.509 certificates
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.405923] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.410079] zswap: loaded using pool lzo/zbud
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.417516] Key type trusted registered
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.424302] Key type encrypted registered
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.425985] ima: No TPM chip found, activating TPM-bypass!
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.428459] evm: HMAC attrs: 0x1
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.431595]   Magic number: 14:108:188
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.432968] acpi device:1a: hash matches
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.434627] rtc_cmos 00:00: setting system clock to 2018-08-09 17:09:05 UTC (1533834545)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.438015] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.440514] EDD information not available.
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.442294] PM: Hibernation image not present or could not be loaded.
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.443864] Freeing unused kernel memory: 1496K
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.445253] Write protecting the kernel read-only data: 14336k
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.448543] Freeing unused kernel memory: 1956K
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.450161] Freeing unused kernel memory: 92K
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.470021] systemd-udevd[118]: starting version 204
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.548397] scsi host0: Virtio SCSI HBA
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.555232] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.555925] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.565315] AVX version of gcm_enc/dec engaged.
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.566995] AES CTR mode by8 optimization enabled
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.622962] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.626175] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.628710] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.631118] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.632869] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.633209] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.639642]  sda: sda1
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    3.642810] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    4.054900] tsc: Refined TSC clocksource calibration: 2499.783 MHz
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    4.057342] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24086cd64eb, max_idle_ns: 440795280575 ns
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    4.401583] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    6.518942] floppy0: no floppy controllers found
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    7.706796] raid6: sse2x1   gen()  8471 MB/s
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    7.774912] raid6: sse2x1   xor()  6490 MB/s
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    7.842794] raid6: sse2x2   gen() 10736 MB/s
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    7.910840] raid6: sse2x2   xor()  7276 MB/s
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    7.978807] raid6: sse2x4   gen() 12028 MB/s
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.046800] raid6: sse2x4   xor()  7569 MB/s
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.049652] raid6: using algorithm sse2x4 gen() 12028 MB/s
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.051802] raid6: .... xor() 7569 MB/s, rmw enabled
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.053764] raid6: using ssse3x2 recovery algorithm
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.057008] xor: automatically using best checksumming function:
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.098782]    avx       : 21400.000 MB/sec
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.115632] Btrfs loaded
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.174653] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.178037] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.259102] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.274850] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.278142] EXT4-fs (sda1): recovery complete
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.287157] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.534640] random: init: uninitialized urandom read (12 bytes read, 21 bits of entropy available)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.679873] random: mountall: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.740645] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    8.949172] random: cloud-init: uninitialized urandom read (32 bytes read, 31 bits of entropy available)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    9.604542] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    9.773710] systemd-udevd[703]: starting version 204
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [    9.908330] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   10.020550] ppdev: user-space parallel port driver
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   10.222374] random: mktemp: uninitialized urandom read (6 bytes read, 49 bits of entropy available)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   10.286869] random: mktemp: uninitialized urandom read (6 bytes read, 49 bits of entropy available)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   10.367433] random: cloud-init: uninitialized urandom read (32 bytes read, 50 bits of entropy available)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   10.538734] random: cloud-init: uninitialized urandom read (32 bytes read, 50 bits of entropy available)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   10.808827] random: mktemp: uninitialized urandom read (12 bytes read, 53 bits of entropy available)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   10.889797] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   10.972882] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   11.009146] EXT4-fs (sda1): resized filesystem to 7864064
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   11.370545] init: failsafe main process (1095) killed by TERM signal
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 instance-setup: INFO Running set_multiqueue.
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 instance-setup: INFO Set channels for eth0 to 4.
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  9 17:09:13 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 instance-setup: INFO /proc/irq/26/smp_affiniyou need this.
Aug  9 17:09:14 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   12.878728] Bridge firewalling registered
Aug  9 17:09:14 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   12.891355] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 17:09:14 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 google-accounts: INFO Created user account asari.
Aug  9 17:09:14 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 google-accounts: INFO Creating a new user account for bogdana.
Aug  9 17:09:14 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   12.928233] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 google-accounts: INFO Created user account bogdana.
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   12.990856] floppy0: no floppy controllers found
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 google-accounts: INFO Creating a new user account for konstantin.
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   13.029867] Initializing XFRM netlink socket
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   13.040953] Netfilter messages via NETLINK v0.30.
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   13.046068] ctnetlink v0.93: registering with nfnetlink.
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 google-accounts: INFO Created user account konstantin.
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 google-accounts: INFO Creating a new user account for carmen.
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 google-accounts: INFO Created user account carmen.
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 google-accounts: INFO Creating a new user account for maria.
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 google-accounts: INFO Created user account maria.
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 google-accounts: INFO Removing user packer.
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 cron[1692]: (CRON) INFO (pidfile fd = 3)
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 cron[1740]: (CRON) STARTUP (fork ok)
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 cron[1740]: (CRON) INFO (Running @reboot jobs)
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 acpid: starting up with netlink and the input layer
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 acpid: 1 rule loaded
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 acpid: waiting for events: event logging is off
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 haveged: haveged starting up
Aug  9 17:09:15 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   13.483691] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 17:09:38 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpdate[1852]: adjust time server 169.254.169.254 offset 0.017877 sec
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpd[1889]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpd[1890]: proto: precision = 0.144 usec
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpd[1890]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpd[1890]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpd[1890]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpd[1890]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpd[1890]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpd[1890]: Listen normally on 3 eth0 10.20.0.218 UDP 123
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpd[1890]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpd[1890]: peers refreshed
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpd[1890]: Listening on routing socket on fd #21 for interface updates
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [   43.677941] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 startup-script: INFO Found startup-script in metadata.
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 startup-script: INFO startup-script: job 1 at Thu Aug  9 20:19:00 2018
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 startup-script: INFO startup-script: Return code 0.
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 startup-script: INFO startup-script: Return code 0.
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 startup-script: INFO Finished running startup scripts.
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ec2: 
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ec2: #############################################################
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ec2: 1024 f3:1e:19:66:53:ca:05:9d:08:00:92:7b:33:07:18:15  root@travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 (DSA)
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ec2: 256 79:db:12:66:02:37:71:cd:e1:4d:c2:b2:2f:a6:11:71  root@travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 (ECDSA)
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ec2: 256 93:48:8b:9d:78:00:15:98:99:bc:ab:38:04:1c:f9:9a  root@travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 (ED25519)
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ec2: 2048 2d:d9:27:0a:d0:40:cf:5f:91:e5:28:a8:85:d9:97:3d  root@travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 (RSA)
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 17:09:45 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ec2: #############################################################
Aug  9 17:14:04 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [  302.819069] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 17:15:14 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [  372.841609] device veth3781aa8 entered promiscuous mode
Aug  9 17:15:14 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [  372.841677] docker0: port 1(veth3781aa8) entered forwarding state
Aug  9 17:15:14 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [  372.841685] docker0: port 1(veth3781aa8) entered forwarding state
Aug  9 17:15:14 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [  372.843210] docker0: port 1(veth3781aa8) entered disabled state
Aug  9 17:15:14 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [  372.929924] cgroup: docker-runc (4867) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 17:15:14 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [  372.929927] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  9 17:15:14 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [  373.013485] eth0: renamed from vethbaaff25
Aug  9 17:15:14 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [  373.048760] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  9 17:15:14 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [  373.050229] docker0: port 1(veth3781aa8) entered forwarding state
Aug  9 17:15:14 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [  373.050245] docker0: port 1(veth3781aa8) entered forwarding state
Aug  9 17:15:14 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [  373.050266] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  9 17:15:18 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpd[1890]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  9 17:15:18 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpd[1890]: Listen normally on 6 docker0 fe80::42:8dff:fe27:8960 UDP 123
Aug  9 17:15:18 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpd[1890]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  9 17:15:18 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpd[1890]: peers refreshed
Aug  9 17:15:18 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 ntpd[1890]: new interface(s) found: waking up resolver
Aug  9 17:15:29 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [  388.056814] docker0: port 1(veth3781aa8) entered forwarding state
Aug  9 17:17:01 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 CRON[5861]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  9 18:01:08 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [ 3126.314066] vethbaaff25: renamed from eth0
Aug  9 18:01:08 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [ 3126.324645] docker0: port 1(veth3781aa8) entered disabled state
Aug  9 18:01:08 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [ 3126.379400] docker0: port 1(veth3781aa8) entered disabled state
Aug  9 18:01:08 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [ 3126.381235] device veth3781aa8 left promiscuous mode
Aug  9 18:01:08 travis-job-2c73f196-7445-4062-b4a2-5cec6c3bdb26 kernel: [ 3126.381237] docker0: port 1(veth3781aa8) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:0f342ee8
