plain
[00:49:53] ....................................................................................................
[00:49:56] ................................................................................................i...
[00:49:59] ....................................................................................................
[00:50:02] ....................................................................................................
[00:50:04] .............................................iiiiiiiii..............................................
[00:50:10] ....................................................................................................
[00:50:14] ....................................................................................................
[00:50:17] ........................i...........................................................................
[00:50:20] ...........................i................................................ii..ii..................
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:19] 
[00:57:19] running 46 tests
[00:57:34] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:57:34] ...................................FF.........
[00:57:34] 
[00:57:34] ---- [mir-opt] mir-opt/lower_128bit_debug_test.rs stdout ----
[00:57:34] 
[00:57:34] error: compilation failed!
[00:57:34] error: compilation failed!
[00:57:34] status: exit code: 1
[00:57:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/lower_128bit_debug_test.rs" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "lower_128bit_ops=yes" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test/auxiliary"
[00:57:34] ------------------------------------------
[00:57:34] 
[00:57:34] ------------------------------------------
[00:57:34] stderr:
[00:57:34] stderr:
[00:57:34] ------------------------------------------
[00:57:34] warning: variable does not need to be mutable
[00:57:34]   --> /checkout/src/test/mir-opt/lower_128bit_debug_test.rs:23:23
[00:57:34]    |
[00:57:34] 23 | const fn const_signed(mut x: i128) -> i128 {
[00:57:34]    |                       |
[00:57:34]    |                       help: remove this `mut`
[00:57:34]    |
[00:57:34]    = note: #[warn(unused_mut)] on by default
[00:57:34]    = note: #[warn(unused_mut)] on by default
[00:57:34] 
[00:57:34] warning: variable does not need to be mutable
[00:57:34]   --> /checkout/src/test/mir-opt/lower_128bit_debug_test.rs:27:25
[00:57:34]    |
[00:57:34] 27 | const fn const_unsigned(mut x: u128) -> u128 {
[00:57:34]    |                         |
[00:57:34]    |                         help: remove this `mut`
[00:57:34] 
[00:57:34] error[E0080]: could not evaluate static initializer
[00:57:34] error[E0080]: could not evaluate static initializer
[00:57:34]   --> /checkout/src/test/mir-opt/lower_128bit_debug_test.rs:24:7
[00:57:34]    |
[00:57:34] 20 | static TEST_SIGNED: i128 = const_signed(-222);
[00:57:34]    |                            ------------------ inside call to `const_signed`
[00:57:34] ...
[00:57:34] 24 |     ((((((x + 1) - 2) * 3) / 4) % 5) << 6) >> 7
[00:57:34]    |       ^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to divide by zero
[00:57:34] error[E0080]: could not evaluate static initializer
[00:57:34]   --> /checkout/src/test/mir-opt/lower_128bit_debug_test.rs:28:7
[00:57:34]    |
[00:57:34]    |
[00:57:34] 21 | static TEST_UNSIGNED: u128 = const_unsigned(200);
[00:57:34]    |                              ------------------- inside call to `const_unsigned`
[00:57:34] ...
[00:57:34] 28 |     ((((((x + 1) - 2) * 3) / 4) % 5) << 6) >> 7
[00:57:34]    |       ^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to divide by zero
[00:57:34] error: aborting due to 2 previous errors
[00:57:34] 
[00:57:34] For more information about this error, try `rustc --explain E0080`.
[00:57:34] 
---
[00:57:34] ---- [mir-opt] mir-opt/lower_128bit_test.rs stdout ----
[00:57:34] 
[00:57:34] error: compilation failed!
[00:57:34] status: exit code: 1
[00:57:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/lower_128bit_test.rs" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "lower_128bit_ops=yes" "-C" "debug_assertions=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test/auxiliary"
[00:57:34] ------------------------------------------
[00:57:34] 
[00:57:34] ------------------------------------------
[00:57:34] stderr:
[00:57:34] stderr:
[00:57:34] ------------------------------------------
[00:57:34] warning: variable does not need to be mutable
[00:57:34]   --> /checkout/src/test/mir-opt/lower_128bit_test.rs:20:23
[00:57:34]    |
[00:57:34] 20 | const fn const_signed(mut x: i128) -> i128 {
[00:57:34]    |                       |
[00:57:34]    |                       help: remove this `mut`
[00:57:34]    |
[00:57:34]    = note: #[warn(unused_mut)] on by default
[00:57:34]    = note: #[warn(unused_mut)] on by default
[00:57:34] 
[00:57:34] warning: variable does not need to be mutable
[00:57:34]   --> /checkout/src/test/mir-opt/lower_128bit_test.rs:24:25
[00:57:34]    |
[00:57:34] 24 | const fn const_unsigned(mut x: u128) -> u128 {
[00:57:34]    |                         |
[00:57:34]    |                         help: remove this `mut`
[00:57:34] 
[00:57:34] error[E0080]: could not evaluate static initializer
[00:57:34] error[E0080]: could not evaluate static initializer
[00:57:34]   --> /checkout/src/test/mir-opt/lower_128bit_test.rs:21:7
[00:57:34]    |
[00:57:34] 17 | static TEST_SIGNED: i128 = const_signed(-222);
[00:57:34]    |                            ------------------ inside call to `const_signed`
[00:57:34] ...
[00:57:34] 21 |     ((((((x + 1) - 2) * 3) / 4) % 5) << 6) >> 7
[00:57:34]    |       ^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to divide by zero
[00:57:34] error[E0080]: could not evaluate static initializer
[00:57:34]   --> /checkout/src/test/mir-opt/lower_128bit_test.rs:25:7
[00:57:34]    |
[00:57:34]    |
[00:57:34] 18 | static TEST_UNSIGNED: u128 = const_unsigned(200);
[00:57:34]    |                              ------------------- inside call to `const_unsigned`
[00:57:34] ...
[00:57:34] 25 |     ((((((x + 1) - 2) * 3) / 4) % 5) << 6) >> 7
[00:57:34]    |       ^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to divide by zero
[00:57:34] error: aborting due to 2 previous errors
[00:57:34] 
[00:57:34] For more information about this error, try `rustc --explain E0080`.
[00:57:34] 
---
[00:57:34] test result: FAILED. 44 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[00:57:34] 
[00:57:34] 
[00:57:34] 
[00:57:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:34] 
[00:57:34] 
[00:57:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:34] Build completed unsuccessfully in 0:11:52
[00:57:34] Build completed unsuccessfully in 0:11:52
[00:57:34] Makefile:58: recipe for target 'check' failed
[00:57:34] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:18f70e90
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:05e04cfb
$ sudo tail -n 500 /var/log/syslog
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Using GB pages for direct mapping
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] kvm-clock: using sched offset of 1575391384 cycles
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Zone ranges:
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000]   Device   empty
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Movable zone start for each node
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Early memory node ranges
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Policy zone: Normal
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] console [ttyS0] enabled
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.482376] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.485642] pid_max: default: 32768 minimum: 301
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.487237] ACPI: Core revision 20150930
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.495837] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.498335] Security Framework initialized
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.499897] Yama: becoming mindful.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.501384] AppArmor: AppArmor disabled by boot time parameter
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.505495] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.518041] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.524668] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.527649] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.530983] Initializing cgroup subsys io
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.532817] Initializing cgroup subsys memory
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.534289] Initializing cgroup subsys devices
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.535762] Initializing cgroup subsys freezer
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.537452] Initializing cgroup subsys net_cls
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.538991] Initializing cgroup subsys perf_event
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.540579] Initializing cgroup subsys net_prio
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.542553] Initializing cgroup subsys hugetlb
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.544133] Initializing cgroup subsys pids
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.545954] CPU: Physical Processor ID: 0
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.547257] CPU: Processor Core ID: 0
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.549451] mce: CPU supports 32 MCE banks
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.551243] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.553110] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.558157] Freeing SMP alternatives memory: 32K
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.569922] ftrace: allocating 32185 entries in 126 pages
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.630847] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.633398] smpboot: Max logical packages: 2
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.635534] x2apic enabled
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.639089] Switched APIC routing to physical x2apic.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.644443] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.752562] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.756205] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.761320] x86: Booting SMP configuration:
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.763225] .... node  #0, CPUs:      #1
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.764853] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.770772]  #2
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.771635] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.778659]  #3
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.779628] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.786275] x86: Booted up 1 node, 4 CPUs
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.788016] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.792539] devtmpfs: initialized
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.798420] evm: security.selinux
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.799676] evm: security.SMACK64
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.800893] evm: security.SMACK64EXEC
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.802221] evm: security.SMACK64TRANSMUTE
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.803438] evm: security.SMACK64MMAP
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.805189] evm: security.ima
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.806289] evm: security.capability
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.808170] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.812321] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.815404] pinctrl core: initialized pinctrl subsystem
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.817671] RTC time: 12:05:35, date: 08/16/18
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.820120] NET: Registered protocol family 16
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.832630] cpuidle: using governor ladder
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.844644] cpuidle: using governor menu
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.846140] PCCT header not found.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.847608] ACPI: bus type PCI registered
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.848919] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.851781] PCI: Using configuration type 1 for base access
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.866337] ACPI: Added _OSI(Module Device)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.868404] ACPI: Added _OSI(Processor Device)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.870201] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.872079] ACPI: Added _OSI(Processor Aggregator Device)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.877821] ACPI: Executed 2 blocks of module-level executable AML code
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.905900] ACPI: Interpreter enabled
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.908672] ACPI: (supports S0 S3 S4 S5)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.910747] ACPI: Using IOAPIC for interrupt routing
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.912990] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.947194] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.950687] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.954655] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.957455] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.963080] PCI host bridge to bus 0000:00
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.964599] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.967268] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.970666] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.974048] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.976952] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.979320] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    0.979822] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.002459] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.025468] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.028754] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.038792] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.045959] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.066940] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.076008] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.082345] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.102956] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.107929] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.112124] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.116798] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.121611] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.145091] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.148492] vgaarb: loaded
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.149825] SCSI subsystem initialized
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.151214] libata version 3.00 loaded.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.151244] ACPI: bus type USB registered
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.153168] usbcore: registered new interface driver usbfs
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.156553] usbcore: registered new interface driver hub
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.159622] usbcore: registered new device driver usb
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.162298] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.165402] dmi: Firmware registration failed.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.167566] PCI: Using ACPI for IRQ routing
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.169101] PCI: pci_cache_line_size set to 64 bytes
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.169214] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.169216] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.169354] NetLabel: Initializing
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.170434] NetLabel:  domain hash size = 128
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.171943] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.173641] NetLabel:  unlabeled traffic allowed by default
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.175732] amd_nb: Cannot enumerate AMD northbridges
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.177997] clocksource: Switched to clocksource kvm-clock
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.188210] pnp: PnP ACPI init
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.189458] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.189529] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.189571] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.189622] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.189661] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.189739] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.189782] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.189957] pnp: PnP ACPI: found 7 devices
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.199980] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.203051] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.203054] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.203056] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.203057] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.203093] NET: Registered protocol family 2
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.205745] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.209010] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.212031] TCP: Hash tables configured (established 131072 bind 65536)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.214364] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.217071] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.220433] NET: Registered protocol family 1
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.222573] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.224619] PCI: CLS 0 bytes, default 64
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    1.224723] Unpacking initramfs...
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.490722] Freeing initrd memory: 21432K
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.492515] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.495069] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.498475] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.500573] hw unit of domain pp0-core 2^-0 Joules
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.501762] hw unit of domain package 2^-0 Joules
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.502750] hw unit of domain dram 2^-0 Joules
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.504092] Scanning for low memory corruption every 60 seconds
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.506899] audit: initializing netlink subsys (disabled)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.508238] audit: type=2000 audit(1534421138.002:1): initialized
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.509946] Initialise system trusted keyring
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.511949] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.513557] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.515889] zbud: loaded
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.516954] VFS: Disk quotas dquot_6.6.0
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.517619] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.518952] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.520495] fuse init (API version 7.23)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.521748] Key type big_key registered
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.522539] Allocating IMA MOK and blacklist keyrings.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.525035] Key type asymmetric registered
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.525742] Asymmetric key parser 'x509' registered
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.526674] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.527889] io scheduler noop registered
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.528478] io scheduler deadline registered (default)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.529243] io scheduler cfq registered
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.530076] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.530973] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.532014] intel_idle: does not run on family 6 model 62
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.532124] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.533230] ACPI: Power Button [PWRF]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.533982] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.535233] ACPI: Sleep Button [SLPF]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.536241] GHES: HEST is not enabled!
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.538760] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.539834] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.544668] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.545872] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.550641] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.573294] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.597157] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.620938] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.644656] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.648542] Linux agpgart interface v0.103
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.652218] loop: module loaded
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.653257] libphy: Fixed MDIO Bus: probed
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.654056] tun: Universal TUN/TAP device driver, 1.6
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.655294] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.691476] PPP generic driver version 2.4.2
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.692860] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.694451] ehci-pci: EHCI PCI platform driver
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.695605] ehci-platform: EHCI generic platform driver
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.696993] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.698666] ohci-pci: OHCI PCI platform driver
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.699989] ohci-platform: OHCI generic platform driver
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.701200] uhci_hcd: USB Universal Host Controller Interface driver
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.703511] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.706652] i8042: Warning: Keylock active
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.709027] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.710279] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.711821] mousedev: PS/2 mouse device common for all mice
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.713759] rtc_cmos 00:00: RTC can wake from S4
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.715169] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.716646] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.718286] i2c /dev entries driver
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.719160] device-mapper: uevent: version 1.0.3
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.720679] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.722797] ledtrig-cpu: registered to indicate activity on CPUs
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.725168] NET: Registered protocol family 10
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.727117] NET: Registered protocol family 17
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.728308] Key type dns_resolver registered
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.729858] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.731380] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.732946] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.734682] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.736347] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.739059] registered taskstats version 1
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.740041] Loading compiled-in X.509 certificates
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.741879] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.744435] zswap: loaded using pool lzo/zbud
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.747922] Key type trusted registered
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.753378] Key type encrypted registered
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.754561] ima: No TPM chip found, activating TPM-bypass!
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.756264] evm: HMAC attrs: 0x1
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.757543]   Magic number: 14:0:77
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.759008] rtc_cmos 00:00: setting system clock to 2018-08-16 12:05:38 UTC (1534421138)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.761831] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.763434] EDD information not available.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.764372] PM: Hibernation image not present or could not be loaded.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.766266] Freeing unused kernel memory: 1496K
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.767354] Write protecting the kernel read-only data: 14336k
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.769710] Freeing unused kernel memory: 1956K
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.771116] Freeing unused kernel memory: 92K
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.787716] systemd-udevd[119]: starting version 204
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.837961] scsi host0: Virtio SCSI HBA
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.849959] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.854846] AVX version of gcm_enc/dec engaged.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.855648] AES CTR mode by8 optimization enabled
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.901097] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.901107] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.901108] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.901552] sd 0:0:1:0: [sda] Write Protect is off
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.901554] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.901610] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.903511]  sda: sda1
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.904611] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    3.914648] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    4.502161] tsc: Refined TSC clocksource calibration: 2499.879 MHz
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    4.503262] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2408c77f92e, max_idle_ns: 440795230049 ns
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    4.752198] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    6.866190] floppy0: no floppy controllers found
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.046050] raid6: sse2x1   gen()  8947 MB/s
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.114051] raid6: sse2x1   xor()  6545 MB/s
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.182051] raid6: sse2x2   gen() 10830 MB/s
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.250044] raid6: sse2x2   xor()  7249 MB/s
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.318058] raid6: sse2x4   gen() 12140 MB/s
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.386046] raid6: sse2x4   xor()  8305 MB/s
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.388714] raid6: using algorithm sse2x4 gen() 12140 MB/s
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.390672] raid6: .... xor() 8305 MB/s, rmw enabled
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.392403] raid6: using ssse3x2 recovery algorithm
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.396919] xor: automatically using best checksumming function:
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.438054]    avx       : 21349.000 MB/sec
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.455910] Btrfs loaded
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.509694] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.512891] EXT4-fs (sda1): write access will be enabled during recovery
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.597200] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.613667] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.615551] EXT4-fs (sda1): recovery complete
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.622522] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    8.861103] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    9.001741] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    9.056059] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    9.275599] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [    9.902327] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   10.063244] systemd-udevd[701]: starting version 204
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   10.185169] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   10.292778] ppdev: user-space parallel port driver
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   10.406106] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   10.467028] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   10.539333] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   10.711952] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   10.979780] random: mktemp: uninitialized urandom read (12 bytes read, 57 bits of entropy available)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   11.062766] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   11.153946] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   11.195189] EXT4-fs (sda1): resized filesystem to 7864064
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   11.644231] init: failsafe main process (1092) killed by TERM signal
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO Running set_multiqueue.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO Set channels for eth0 to 4.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 16 12:05:46 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 google-accounts: INFO Starting Google Accounts daemon.
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 google-accounts: INFO Creating a new user account for me.
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 google-accounts: INFO Created user account me.
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 google-accounts: INFO Creating a new user account for bogdana.
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 google-accounts: INFO Created user account bogdana.
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 google-accounts: INFO Creating a new user account for aj.
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 google-accounts: INFO Created user account aj.
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 google-accounts: INFO Creating a new user account for asari.
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 google-accounts: INFO Created user account asari.
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   12.854222] random: nonblocking pool is initialized
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 google-accounts: INFO Removing user packer.
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   13.080668] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   13.085635] Bridge firewalling registered
Aug 16 12:05:47 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   13.099686] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 google-clock-skew: INFO Synced system time with hardware clock.
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   13.134415] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   13.202698] Initializing XFRM netlink socket
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   13.210049] Netfilter messages via NETLINK v0.30.
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   13.212571] ctnetlink v0.93: registering with nfnetlink.
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   13.258206] floppy0: no floppy controllers found
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 cron[1624]: (CRON) INFO (pidfile fd = 3)
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 cron[1666]: (CRON) STARTUP (fork ok)
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 cron[1666]: (CRON) INFO (Running @reboot jobs)
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 acpid: starting up with netlink and the input layer
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 acpid: 1 rule loaded
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 acpid: waiting for events: event logging is off
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 haveged: haveged starting up
Aug 16 12:05:48 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   13.873070] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 16 12:05:53 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpd[1770]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 16 12:05:53 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpd[1771]: proto: precision = 0.108 usec
Aug 16 12:05:53 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpd[1771]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 16 12:05:53 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpd[1771]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 16 12:05:53 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpd[1771]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 16 12:05:53 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpd[1771]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 16 12:05:53 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpd[1771]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 16 12:05:53 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpd[1771]: Listen normally on 3 eth0 10.20.0.170 UDP 123
Aug 16 12:05:53 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpd[1771]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 16 12:05:53 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpd[1771]: peers refreshed
Aug 16 12:05:53 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpd[1771]: Listening on routing socket on fd #21 for interface updates
Aug 16 12:05:53 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   19.067050] init: plymouth-upstart-bridge main process ended, respawning
Aug 16 12:05:54 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 startup-script: INFO Found startup-script in metadata.
Aug 16 12:05:54 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 16 12:05:54 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 startup-script: INFO startup-script: job 1 at Thu Aug 16 15:15:00 2018
Aug 16 12:05:54 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 startup-script: INFO startup-script: Return code 0.
Aug 16 12:05:54 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 startup-script: INFO startup-script: Return code 0.
Aug 16 12:05:54 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 startup-script: INFO Finished running startup scripts.
Aug 16 12:05:54 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ec2: 
Aug 16 12:05:54 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ec2: #############################################################
Aug 16 12:05:54 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 16 12:05:54 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ec2: 1024 ac:48:4c:6c:78:f1:ba:d2:9b:14:52:a0:dc:18:6e:17  root@travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 (DSA)
Aug 16 12:05:54 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ec2: 256 89:ce:79:7b:ea:e0:63:71:37:26:f0:f2:bf:8c:44:e6  root@travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 (ECDSA)
Aug 16 12:05:54 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ec2: 256 fe:a0:76:e7:77:db:97:ef:c9:a5:11:0c:da:97:c1:98  root@travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 (ED25519)
Aug 16 12:05:54 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ec2: 2048 63:85:cb:5f:a2:6e:70:61:b9:c6:b7:ea:d1:b3:fc:f0  root@travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 (RSA)
Aug 16 12:05:54 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 16 12:05:54 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ec2: #############################################################
Aug 16 12:06:03 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpdate[2221]: the NTP socket is in use, exiting
Aug 16 12:06:41 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [   66.879757] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 16 12:08:23 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [  168.098344] device vethb164b20 entered promiscuous mode
Aug 16 12:08:23 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [  168.098431] docker0: port 1(vethb164b20) entered forwarding state
Aug 16 12:08:23 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [  168.098439] docker0: port 1(vethb164b20) entered forwarding state
Aug 16 12:08:23 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [  168.099048] docker0: port 1(vethb164b20) entered disabled state
Aug 16 12:08:23 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [  168.230675] cgroup: docker-runc (4872) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 16 12:08:23 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [  168.230678] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 16 12:08:23 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [  168.314324] eth0: renamed from veth3357bab
Aug 16 12:08:23 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [  168.360791] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 16 12:08:23 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [  168.362483] docker0: port 1(vethb164b20) entered forwarding state
Aug 16 12:08:23 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [  168.362510] docker0: port 1(vethb164b20) entered forwarding state
Aug 16 12:08:23 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [  168.362530] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 16 12:08:26 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpd[1771]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 16 12:08:26 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpd[1771]: Listen normally on 6 docker0 fe80::42:5eff:fe35:ea3a UDP 123
Aug 16 12:08:26 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpd[1771]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 16 12:08:26 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpd[1771]: peers refreshed
Aug 16 12:08:26 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 ntpd[1771]: new interface(s) found: waking up resolver
Aug 16 12:08:38 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [  183.369475] docker0: port 1(vethb164b20) entered forwarding state
Aug 16 12:17:01 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 CRON[12432]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 16 12:57:45 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [ 3130.827058] traps: a[15511] trap invalid opcode ip:55db2f26fa9b sp:7fff904b0b80 error:0 in a[55db2f26c000+6000]
Aug 16 12:58:00 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [ 3145.829203] traps: a[18350] trap invalid opcode ip:7fa54b8e5381 sp:7ffebd797e50 error:0 in libstd-2339b911e3c09de8.so[7fa54b885000+16f000]
Aug 16 12:58:00 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [ 3145.859470] traps: a[18369] trap invalid opcode ip:7f413838d381 sp:7fffc55885f0 error:0 in libstd-2339b911e3c09de8.so[7f413832d000+16f000]
Aug 16 12:59:25 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [ 3230.223310] traps: a[780] trap invalid opcode ip:55897beebd98 sp:7ffc518acc20 error:0 in a[55897bee9000+4000]
Aug 16 13:02:16 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [ 3401.034470] a[29284]: segfault at 0 ip 00005632841a2463 sp 00007ffc9f5ccce0 error 6 in a[56328419f000+5000]
Aug 16 13:02:26 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [ 3411.595480] a[30115]: segfault at 1 ip 000055cb56fc9b8c sp 00007ffee4c7a460 error 6 in a[55cb56fc7000+4000]
Aug 16 13:02:30 travis-job-d8ca11d9-035e-4dde-9a79-e401cb0ba5e7 kernel: [ 3415.409913] traps: a[30474] trap invalid opcode ip:56417f12942c sp:7ffed2c11d90 error:0 in a[56417f126000+7000]
---
travis_time:end:1e8123a0:start=1534424659256770107,finish=1534424659265523359,duration=8753252
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12809621
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1cadfc48
travis_time:start:1cadfc48
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:029fa270
$ dmesg | grep -i kill
