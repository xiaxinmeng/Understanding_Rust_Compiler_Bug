plain
[00:49:46] ....................................................................................................
[00:49:50] ...............................................................................................i....
[00:49:52] ....................................................................................................
[00:49:56] ....................................................................................................
[00:49:58] ............................................iiiiiiiii...............................................
[00:50:04] ....................................................................................................
[00:50:08] ....................................................................................................
[00:50:10] .......................i............................................................................
[00:50:13] ..........................i...............................................i.i..ii...................
---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:35] 
[01:02:35] running 255 tests
[01:03:46] ......................i.................................................................F...........
[01:04:49] ......................i.F...................F.......................................................
[01:05:25] failures:
[01:05:25] 
[01:05:25] ---- [rustdoc] rustdoc/issue-13698.rs stdout ----
[01:05:25] 
[01:05:25] 
[01:05:25] error: htmldocck failed!
[01:05:25] status: exit code: 1
[01:05:25] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-13698" "/checkout/src/test/rustdoc/issue-13698.rs"
[01:05:25] ------------------------------------------
[01:05:25] 
[01:05:25] ------------------------------------------
[01:05:25] stderr:
[01:05:25] stderr:
[01:05:25] ------------------------------------------
[01:05:25] 17: @!has check failed
[01:05:25]  `XPATH PATTERN` did not match
[01:05:25]  // @!has issue_13698/struct.Foo.html '//*[@id="method.foo"]' 'fn foo'
[01:05:25] Encountered 1 errors
[01:05:25] 
[01:05:25] ------------------------------------------
[01:05:25] 
[01:05:25] 
[01:05:25] thread '[rustdoc] rustdoc/issue-13698.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[01:05:25] 
[01:05:25] ---- [rustdoc] rustdoc/issue-25001.rs stdout ----
[01:05:25] 
[01:05:25] error: htmldocck failed!
[01:05:25] status: exit code: 1
[01:05:25] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25001" "/checkout/src/test/rustdoc/issue-25001.rs"
[01:05:25] ------------------------------------------
[01:05:25] 
[01:05:25] ------------------------------------------
[01:05:25] stderr:
[01:05:25] stderr:
[01:05:25] ------------------------------------------
[01:05:25] 40: @has check failed
[01:05:25]  `XPATH PATTERN` did not match
[01:05:25]      // @has - '//*[@id="method.quux"]//code' 'fn quux(self)'
[01:05:25] 47: @has check failed
[01:05:25]  `XPATH PATTERN` did not match
[01:05:25]      // @has - '//*[@id="method.quux-1"]//code' 'fn quux(self)'
[01:05:25] 54: @has check failed
[01:05:25]  `XPATH PATTERN` did not match
[01:05:25]      // @has - '//*[@id="method.quux-2"]//code' 'fn quux(self)'
[01:05:25] Encountered 3 errors
[01:05:25] 
[01:05:25] ------------------------------------------
[01:05:25] 
[01:05:25] 
[01:05:25] thread '[rustdoc] rustdoc/issue-25001.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[01:05:25] 
[01:05:25] ---- [rustdoc] rustdoc/issue-33302.rs stdout ----
[01:05:25] 
[01:05:25] error: htmldocck failed!
[01:05:25] status: exit code: 1
[01:05:25] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33302" "/checkout/src/test/rustdoc/issue-33302.rs"
[01:05:25] ------------------------------------------
[01:05:25] 
[01:05:25] ------------------------------------------
[01:05:25] stderr:
[01:05:25] stderr:
[01:05:25] ------------------------------------------
[01:05:25] 40: @has check failed
[01:05:25]  `XPATH PATTERN` did not match
[01:05:25]          // @has - '//*[@class="docblock"]' 'C: [i32; 16] = [0; 4 * 4]'
[01:05:25] Encountered 1 errors
[01:05:25] 
[01:05:25] ------------------------------------------
[01:05:25] 
---
[01:05:25] 
[01:05:25] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:05:25] 
[01:05:25] 
[01:05:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:25] 
[01:05:25] 
[01:05:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:25] Build completed unsuccessfully in 0:19:43
[01:05:25] Build completed unsuccessfully in 0:19:43
[01:05:25] Makefile:58: recipe for target 'check' failed
[01:05:25] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0862c35e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:046f2c08
$ sudo tail -n 500 /var/log/syslog
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   2 disabled
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   3 disabled
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   4 disabled
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   5 disabled
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   6 disabled
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   7 disabled
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Using GB pages for direct mapping
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] kvm-clock: using sched offset of 1655514141 cycles
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Zone ranges:
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   Device   empty
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Movable zone start for each node
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Early memory node ranges
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Policy zone: Normal
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] console [ttyS0] enabled
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.474807] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.477596] pid_max: default: 32768 minimum: 301
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.479028] ACPI: Core revision 20150930
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.486937] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.489194] Security Framework initialized
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.490900] Yama: becoming mindful.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.492079] AppArmor: AppArmor disabled by boot time parameter
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.495933] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.507917] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.514270] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.516590] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.519610] Initializing cgroup subsys io
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.520997] Initializing cgroup subsys memory
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.523111] Initializing cgroup subsys devices
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.524565] Initializing cgroup subsys freezer
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.526095] Initializing cgroup subsys net_cls
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.527763] Initializing cgroup subsys perf_event
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.529070] Initializing cgroup subsys net_prio
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.530531] Initializing cgroup subsys hugetlb
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.531993] Initializing cgroup subsys pids
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.533919] CPU: Physical Processor ID: 0
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.535608] CPU: Processor Core ID: 0
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.538156] mce: CPU supports 32 MCE banks
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.539716] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.541793] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.546594] Freeing SMP alternatives memory: 32K
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.558690] ftrace: allocating 32185 entries in 126 pages
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.620776] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.623422] smpboot: Max logical packages: 2
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.625762] x2apic enabled
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.627968] Switched APIC routing to physical x2apic.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.632849] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.741341] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.746215] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.751508] x86: Booting SMP configuration:
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.753487] .... node  #0, CPUs:      #1
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.754870] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.761241]  #2
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.762195] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.768142]  #3
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.769045] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.775747] x86: Booted up 1 node, 4 CPUs
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.777225] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.781878] devtmpfs: initialized
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.787444] evm: security.selinux
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.788734] evm: security.SMACK64
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.789660] evm: security.SMACK64EXEC
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.791219] evm: security.SMACK64TRANSMUTE
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.792699] evm: security.SMACK64MMAP
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.794000] evm: security.ima
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.795735] evm: security.capability
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.797430] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.801576] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.804185] pinctrl core: initialized pinctrl subsystem
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.806283] RTC time: 21:10:41, date: 08/14/18
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.809001] NET: Registered protocol family 16
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.821458] cpuidle: using governor ladder
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.833445] cpuidle: using governor menu
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.834904] PCCT header not found.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.836119] ACPI: bus type PCI registered
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.837836] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.841051] PCI: Using configuration type 1 for base access
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.855069] ACPI: Added _OSI(Module Device)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.856884] ACPI: Added _OSI(Processor Device)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.858192] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.859479] ACPI: Added _OSI(Processor Aggregator Device)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.863809] ACPI: Executed 2 blocks of module-level executable AML code
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.889810] ACPI: Interpreter enabled
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.891346] ACPI: (supports S0 S3 S4 S5)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.893229] ACPI: Using IOAPIC for interrupt routing
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.894782] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.927458] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.929979] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.932084] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.934838] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.940565] PCI host bridge to bus 0000:00
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.941874] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.944396] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.946526] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.949378] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.951918] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.953527] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.953992] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    0.989154] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.021280] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.023988] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.036580] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.048039] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.077451] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.107034] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.116489] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.144847] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.150563] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.155874] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.161881] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.167494] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.190572] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.193431] vgaarb: loaded
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.194709] SCSI subsystem initialized
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.196574] libata version 3.00 loaded.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.196612] ACPI: bus type USB registered
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.198298] usbcore: registered new interface driver usbfs
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.200371] usbcore: registered new interface driver hub
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.202497] usbcore: registered new device driver usb
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.204759] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.207277] dmi: Firmware registration failed.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.209467] PCI: Using ACPI for IRQ routing
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.211144] PCI: pci_cache_line_size set to 64 bytes
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.211256] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.211259] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.211403] NetLabel: Initializing
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.212806] NetLabel:  domain hash size = 128
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.214379] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.216217] NetLabel:  unlabeled traffic allowed by default
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.218951] amd_nb: Cannot enumerate AMD northbridges
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.220700] clocksource: Switched to clocksource kvm-clock
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.230323] pnp: PnP ACPI init
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.231902] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.231973] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.232049] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.232097] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.232138] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.232177] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.232215] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.232409] pnp: PnP ACPI: found 7 devices
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.242343] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.246502] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.246505] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.246506] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.246508] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.246552] NET: Registered protocol family 2
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.249098] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.253225] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.256360] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.259188] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.261665] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.266668] NET: Registered protocol family 1
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.268811] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.270784] PCI: CLS 0 bytes, default 64
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    1.270846] Unpacking initramfs...
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.558132] Freeing initrd memory: 21432K
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.559837] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.562451] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.565593] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.569686] hw unit of domain pp0-core 2^-0 Joules
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.571788] hw unit of domain package 2^-0 Joules
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.573097] hw unit of domain dram 2^-0 Joules
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.574389] Scanning for low memory corruption every 60 seconds
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.577126] audit: initializing netlink subsys (disabled)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.579174] audit: type=2000 audit(1534281043.781:1): initialized
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.581349] Initialise system trusted keyring
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.583259] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.585425] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.589685] zbud: loaded
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.591326] VFS: Disk quotas dquot_6.6.0
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.592682] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.594765] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.596588] fuse init (API version 7.23)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.597765] Key type big_key registered
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.598844] Allocating IMA MOK and blacklist keyrings.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.601258] Key type asymmetric registered
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.602026] Asymmetric key parser 'x509' registered
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.602952] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.604183] io scheduler noop registered
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.604938] io scheduler deadline registered (default)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.605776] io scheduler cfq registered
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.606428] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.607193] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.608186] intel_idle: does not run on family 6 model 62
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.608284] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.609699] ACPI: Power Button [PWRF]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.610438] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.611798] ACPI: Sleep Button [SLPF]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.612819] GHES: HEST is not enabled!
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.615753] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.616814] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.622312] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.623287] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.628895] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.651569] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.675468] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.699262] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.723461] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.727136] Linux agpgart interface v0.103
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.731153] loop: module loaded
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.732395] libphy: Fixed MDIO Bus: probed
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.733884] tun: Universal TUN/TAP device driver, 1.6
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.735103] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.779970] PPP generic driver version 2.4.2
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.781461] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.783657] ehci-pci: EHCI PCI platform driver
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.785120] ehci-platform: EHCI generic platform driver
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.786494] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.788391] ohci-pci: OHCI PCI platform driver
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.789696] ohci-platform: OHCI generic platform driver
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.791237] uhci_hcd: USB Universal Host Controller Interface driver
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.793192] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.797022] i8042: Warning: Keylock active
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.799428] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.801098] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.802679] mousedev: PS/2 mouse device common for all mice
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.805190] rtc_cmos 00:00: RTC can wake from S4
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.806965] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.808910] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.810477] i2c /dev entries driver
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.811522] device-mapper: uevent: version 1.0.3
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kerne   3.835741] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.838868] zswap: loaded using pool lzo/zbud
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.842402] Key type trusted registered
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.847758] Key type encrypted registered
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.849671] ima: No TPM chip found, activating TPM-bypass!
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.851642] evm: HMAC attrs: 0x1
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.853144]   Magic number: 14:907:196
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.854718] rtc_cmos 00:00: setting system clock to 2018-08-14 21:10:44 UTC (1534281044)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.858425] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.860657] EDD information not available.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.861999] PM: Hibernation image not present or could not be loaded.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.863620] Freeing unused kernel memory: 1496K
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.865460] Write protecting the kernel read-only data: 14336k
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.868037] Freeing unused kernel memory: 1956K
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.869941] Freeing unused kernel memory: 92K
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.886500] systemd-udevd[119]: starting version 204
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.959299] scsi host0: Virtio SCSI HBA
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.968963] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.972108] AVX version of gcm_enc/dec engaged.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    3.973815] AES CTR mode by8 optimization enabled
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    4.005370] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    4.026522] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    4.026628] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    4.026630] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    4.027330] sd 0:0:1:0: [sda] Write Protect is off
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    4.027332] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    4.027422] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    4.029760]  sda: sda1
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    4.031471] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    4.572911] tsc: Refined TSC clocksource calibration: 2499.783 MHz
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    4.575849] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24086cd64eb, max_idle_ns: 440795280575 ns
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    4.842015] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    6.933050] floppy0: no floppy controllers found
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.116714] raid6: sse2x1   gen()  9035 MB/s
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.184715] raid6: sse2x1   xor()  6865 MB/s
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.252712] raid6: sse2x2   gen() 11188 MB/s
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.320711] raid6: sse2x2   xor()  7824 MB/s
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.388716] raid6: sse2x4   gen() 12135 MB/s
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.456716] raid6: sse2x4   xor()  8263 MB/s
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.458756] raid6: using algorithm sse2x4 gen() 12135 MB/s
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.460847] raid6: .... xor() 8263 MB/s, rmw enabled
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.462845] raid6: using ssse3x2 recovery algorithm
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.467086] xor: automatically using best checksumming function:
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.508758]    avx       : 21539.000 MB/sec
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.525492] Btrfs loaded
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.583763] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.586217] EXT4-fs (sda1): write access will be enabled during recovery
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.682644] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.698661] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.700376] EXT4-fs (sda1): recovery complete
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.709383] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    8.965380] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    9.122784] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    9.181478] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [    9.425537] random: cloud-init: uninitialized urandom read (32 bytes read, 34 bits of entropy available)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   10.025818] random: cloud-init: uninitialized urandom read (32 bytes read, 42 bits of entropy available)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   10.172124] systemd-udevd[701]: starting version 204
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   10.288994] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   10.385065] ppdev: user-space parallel port driver
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   10.491491] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   10.547410] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   10.615193] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   10.778480] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   11.035967] random: mktemp: uninitialized urandom read (12 bytes read, 57 bits of entropy available)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   11.117394] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   11.203786] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   11.243815] EXT4-fs (sda1): resized filesystem to 7864064
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   11.639254] init: failsafe main process (1092) killed by TERM signal
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO Running set_multiqueue.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO Set channels for eth0 to 4.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 14 21:10:52 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   12.433039] random: nonblocking pool is initialized
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 google-accounts: INFO Starting Google Accounts daemon.
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 google-accounts: INFO Creating a new user account for me.
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 google-accounts: INFO Created user account me.
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 google-accounts: INFO Removing user packer.
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 cron[1402]: (CRON) INFO (pidfile fd = 3)
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 cron[1446]: (CRON) STARTUP (fork ok)
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 cron[1446]: (CRON) INFO (Running @reboot jobs)
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 acpid: starting up with netlink and the input layer
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 acpid: 1 rule loaded
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 acpid: waiting for events: event logging is off
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 haveged: haveged starting up
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   12.906681] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   12.917943] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   13.046330] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   13.053713] Bridge firewalling registered
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   13.064485] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   13.134646] Initializing XFRM netlink socket
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   13.143144] Netfilter messages via NETLINK v0.30.
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   13.146709] ctnetlink v0.93: registering with nfnetlink.
Aug 14 21:10:53 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   13.336842] floppy0: no floppy controllers found
Aug 14 21:10:59 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ntpdate[974]: adjust time server 169.254.169.254 offset -0.035453 sec
Aug 14 21:11:16 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ntpdate[1730]: adjust time server 169.254.169.254 offset 0.013912 sec
Aug 14 21:11:23 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ntpd[1763]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 14 21:11:23 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ntpd[1764]: proto: precision = 0.100 usec
---
Aug 14 21:11:23 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ntpd[1764]: Listen normally on 3 eth0 10.20.0.19 UDP 123
Aug 14 21:11:23 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ntpd[1764]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 14 21:11:23 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ntpd[1764]: peers refreshed
Aug 14 21:11:23 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ntpd[1764]: Listening on routing socket on fd #21 for interface updates
Aug 14 21:11:23 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [   43.134675] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 21:11:23 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 startup-script: INFO Found startup-script in metadata.
Aug 14 21:11:23 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 14 21:11:23 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 startup-script: INFO startup-script: job 1 at Wed Aug 15 00:21:00 2018
Aug 14 21:11:23 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 startup-script: INFO startup-script: Return code 0.
Aug 14 21:11:23 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 startup-script: INFO startup-script: Return code 0.
Aug 14 21:11:23 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 startup-script: INFO Finished running startup scripts.
Aug 14 21:11:24 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ec2: 
Aug 14 21:11:24 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ec2: #############################################################
Aug 14 21:11:24 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 14 21:11:24 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ec2: 1024 dc:b4:48:bf:64:70:27:6d:ff:9f:ae:9b:ca:0c:d7:2b  root@travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 (DSA)
Aug 14 21:11:24 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ec2: 256 b8:a3:66:37:5f:44:b5:e6:f1:b8:15:0d:c7:18:d6:56  root@travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 (ECDSA)
Aug 14 21:11:24 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ec2: 256 bb:08:c8:a7:5b:78:46:d2:42:e6:a6:c8:1c:e2:91:83  root@travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 (ED25519)
Aug 14 21:11:24 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ec2: 2048 17:e9:43:0e:95:55:f0:1b:31:35:53:21:04:76:78:26  root@travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 (RSA)
Aug 14 21:11:24 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 14 21:11:24 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ec2: #############################################################
Aug 14 21:13:02 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [  141.806767] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 14 21:14:07 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [  206.552074] device vethfa822a5 entered promiscuous mode
Aug 14 21:14:07 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [  206.684127] cgroup: docker-runc (4755) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 21:14:07 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [  206.684130] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 21:14:07 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [  206.911777] eth0: renamed from veth882b485
Aug 14 21:14:07 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [  206.957955] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 21:14:07 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [  206.959367] docker0: port 1(vethfa822a5) entered forwarding state
Aug 14 21:14:07 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [  206.959383] docker0: port 1(vethfa822a5) entered forwarding state
Aug 14 21:14:07 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [  206.959401] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 14 21:14:10 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ntpd[1764]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 14 21:14:10 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ntpd[1764]: Listen normally on 6 docker0 fe80::42:65ff:fe91:1843 UDP 123
Aug 14 21:14:10 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ntpd[1764]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 14 21:14:10 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ntpd[1764]: peers refreshed
Aug 14 21:14:10 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 ntpd[1764]: new interface(s) found: waking up resolver
Aug 14 21:14:22 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [  221.971018] docker0: port 1(vethfa822a5) entered forwarding state
Aug 14 21:17:01 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 CRON[6113]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 14 22:03:59 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [ 3198.592002] traps: a[15665] trap invalid opcode ip:55d437611a9b sp:7ffef971d2e0 error:0 in a[55d43760e000+6000]
Aug 14 22:04:14 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [ 3213.393458] traps: a[18475] trap invalid opcode ip:7f21f3491381 sp:7ffc0173a030 error:0 in libstd-2339b911e3c09de8.so[7f21f3431000+16f000]
Aug 14 22:04:14 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [ 3213.451225] traps: a[18480] trap invalid opcode ip:7f1bb5055381 sp:7ffeaea4a6e0 error:0 in libstd-2339b911e3c09de8.so[7f1bb4ff5000+16f000]
Aug 14 22:05:37 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [ 3296.724210] traps: a[894] trap invalid opcode ip:563551551d98 sp:7ffc887b6540 error:0 in a[56355154f000+4000]
Aug 14 22:08:23 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [ 3463.058298] a[29464]: segfault at 0 ip 000055e5df729463 sp 00007fff8f9a83b0 error 6 in a[55e5df726000+5000]
Aug 14 22:08:33 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [ 3472.196803] a[30226]: segfault at 1 ip 000055e36a556b8c sp 00007fff53efc450 error 6 in a[55e36a554000+4000]
Aug 14 22:08:37 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [ 3476.089594] traps: a[30595] trap invalid opcode ip:559c931b842c sp:7ffd7b9c1b10 error:0 in a[559c931b5000+7000]
Aug 14 22:17:01 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 CRON[26632]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 14 22:18:29 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [ 4068.054351] docker0: port 1(vethfa822a5) entered disabled state
Aug 14 22:18:29 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [ 4068.054430] veth882b485: renamed from eth0
Aug 14 22:18:29 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [ 4068.120826] docker0: port 1(vethfa822a5) entered disabled state
Aug 14 22:18:29 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [ 4068.122890] device vethfa822a5 left promiscuous mode
Aug 14 22:18:29 travis-job-c75c875f-04a8-444d-b3ce-0410efe25b19 kernel: [ 4068.122895] docker0: port 1(vethfa822a5) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:11487c81
---
travis_time:end:03fc8900:start=1534285111504772041,finish=1534285111513905405,duration=9133364
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2916df77
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1aba9e38
travis_time:start:1aba9e38
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/a
