plain
[00:51:19] ....................................................................................................
[00:51:21] ....................................................................................................
[00:51:24] ....................................................................................................
[00:51:27] ....................................................................................................
[00:51:30] ........iiiiiiiii...................................................................................
[00:51:36] ....................................................................................................
[00:51:39] ............i.......................................................................................
[00:51:42] .....................i..............................................................................
[00:51:45] ....................................................................................................
---
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:54] 
[00:51:54] running 3057 tests
[00:52:03] ..........FF........................................................................................
[00:52:26] ....................................................................................................
[00:52:36] ....................................................................................................
[00:52:45] ....................................................................................................
[00:53:00] ....................................................................................................
---
[00:57:55] ---- [run-pass] run-pass/allocator/custom.rs stdout ----
[00:57:55] 
[00:57:55] error: compilation failed!
[00:57:55] status: exit code: 1
[00:57:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/allocator/custom.rs" "--target=x86_64-unknown-linux-gnu" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/auxiliary"
[00:57:55] ------------------------------------------
[00:57:55] 
[00:57:55] ------------------------------------------
[00:57:55] stderr:
[00:57:55] stderr:
[00:57:55] ------------------------------------------
[00:57:55] error: linking with `cc` failed: exit code: 1
[00:57:55]   |
[00:57:55]   = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a.custom0-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a.custom1-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a.custom10-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a.custom2-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a.custom3-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a.custom4-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a.custom5-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a.custom6-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a.custom7-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a.custom8-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a.custom9-317d481089b8c8fe83113de504472633.rs.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-g-2866e3dedc0a56e0.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
[00:57:55]   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a.crate.allocator.rcgu.o: In function `__rust_alloc':
[00:57:55]           allocator:(.text.__rust_alloc+0x1): undefined reference to `__rg_alloc'
[00:57:55]           /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a.crate.allocator.rcgu.o: In function `__rust_dealloc':
[00:57:55]           allocator:(.text.__rust_dealloc+0x1): undefined reference to `__rg_dealloc'
[00:57:55]           /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a.crate.allocator.rcgu.o: In function `__rust_realloc':
[00:57:55]           allocator:(.text.__rust_realloc+0x1): undefined reference to `__rg_realloc'
[00:57:55]           /usr/bin/ld: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a: hidden symbol `__rg_alloc' isn't defined
[00:57:55]           /usr/bin/ld: final link failed: Bad value
[00:57:55]           
[00:57:55] 
[00:57:55] error: aborting due to previous error
[00:57:55] 
---
[00:57:55] ---- [run-pass] run-pass/allocator/xcrate-use.rs stdout ----
[00:57:55] 
[00:57:55] error: compilation failed!
[00:57:55] status: exit code: 1
[00:57:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/allocator/xcrate-use.rs" "--target=x86_64-unknown-linux-gnu" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/auxiliary"
[00:57:55] ------------------------------------------
[00:57:55] 
[00:57:55] ------------------------------------------
[00:57:55] stderr:
[00:57:55] stderr:
[00:57:55] ------------------------------------------
[00:57:55] error: linking with `cc` failed: exit code: 1
[00:57:55]   |
[00:57:55]   = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a.xcrate_use0-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a.xcrate_use1-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a.xcrate_use2-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a.xcrate_use3-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a.xcrate_use4-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a.xcrate_use5-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a.xcrate_use6-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a.xcrate_use7-317d481089b8c8fe83113de504472633.rs.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a.crate.allocator.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/auxiliary/libhelper.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/auxiliary/libcustom.rlib" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-e054c7a28f8831a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-17efed325058ddbe.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-a2697584dddf62e0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-ebc4ac8fd3c426d7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-a1822c0b755de650.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-0f56ef064ad577ca.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-aae624166adf9237.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-2866e3dedc0a56e0.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
[00:57:55]   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a.crate.allocator.rcgu.o: In function `__rust_alloc':
[00:57:55]           allocator:(.text.__rust_alloc+0x1): undefined reference to `__rg_alloc'
[00:57:55]           /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a.crate.allocator.rcgu.o: In function `__rust_dealloc':
[00:57:55]           allocator:(.text.__rust_dealloc+0x1): undefined reference to `__rg_dealloc'
[00:57:55]           /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a.crate.allocator.rcgu.o: In function `__rust_realloc':
[00:57:55]           allocator:(.text.__rust_realloc+0x1): undefined reference to `__rg_realloc'
[00:57:55]           /usr/bin/ld: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a: hidden symbol `__rg_alloc' isn't defined
[00:57:55]           /usr/bin/ld: final link failed: Bad value
[00:57:55]           
[00:57:55] 
[00:57:55] error: aborting due to previous error
[00:57:55] 
---
[00:57:55] 
[00:57:55] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:57:55] 
[00:57:55] 
[00:57:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--srcTue, 07 Aug 2018 15:30:24 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_fold:start:after_failure.1
travis_time:start:0085ff48
$ sudo tail -n 500 /var/log/syslog
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] SRAT: Node 0 PXM0-0x0000000000ffffff]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000]   Device   empty
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] Movable zone start for each node
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] Early memory node ranges
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000]m 0xc0000000-0xfffbbfff]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] Policy zone: Normal
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kerne394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.357621] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.358513] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.361733] Freeing SMP alternatives memory: 32K
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.369902] ftrace: allocating 32185 entries in 126 pages
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.417097] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.418330] smpboot: Max logical packages: 2
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.419813] x2apic enabled
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.421724] Switched APIC routing to physical x2apic.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.425280] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.531314] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.533388] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.536803] x86: Booting SMP configuration:
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.537542] .... node  #0, CPUs:      #1
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.538412] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.542456]  #2
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.542996] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.548038]  #3
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.548437] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.552539] x86: Booted up 1 node, 4 CPUs
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.553136] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.555241] devtmpfs: initialized
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.559111] evm: security.selinux
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.559710] evm: security.SMACK64
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.560296] evm: security.SMACK64EXEC
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.560815] evm: security.SMACK64TRANSMUTE
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.561556] evm: security.SMACK64MMAP
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.562065] evm: security.ima
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.562498] evm: security.capability
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.563288] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.564632] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.565773] pinctrl core: initialized pinctrl subsystem
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.566758] RTC time: 14:31:16, date: 08/07/18
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.568208] NET: Registered protocol family 16
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.579349] cpuidle: using governor ladder
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.591368] cpuidle: using governor menu
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.591941] PCCT header not found.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.592523] ACPI: bus type PCI registered
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.593098] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.594162] PCI: Using configuration type 17 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.670530] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.672760] PCI host bridge to bus 0000:00
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.673359] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.674383] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.675327] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.676558] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.677745] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.678819] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.679231] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.692246] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.705422] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.706778] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.711674] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.716452] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.728557] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.733019] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.737185] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.748402] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.750452] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.752634] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.754774] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.756743] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.776724] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 14:31:26:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.784923] NetLabel: Initializing
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.785439] NetLabel:  domain hash size = 128
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.786054] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.786902] NetLabel:  unlabeled traffic allowed by default
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.788078] amd_nb: Cannot enumerate AMD northbridges
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.788878] clocksource: Switched to clocksource kvm-clock
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.796012] pnp: PnP ACPI init
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.796604] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.796672] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.796723] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.796776] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.796823] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.796866] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.796925] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.797099] pnp: PnP ACPI: found 7 devices
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.804413] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.805647] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.805649] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.805651] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.805652] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.805682] NET: Registered protocol family 2
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.806526] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.808427] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.809493] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.810423] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.811339] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.813071] NET: Registered protocol family 1
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.813740] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.814610] PCI: CLS 0 bytes, default 64
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    0.814657] Unpacking initramfs...
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.802755] Freeing initrd memory: 21432K
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.803656] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.804734] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.806273] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.807680] hw unit of domain pp0-core 2^-0 Joules
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.80836r Button [PWRF]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.832156] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.833192] ACPI: Sleep Button [SLPF]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.834028] GHES: HEST is not enabled!
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.836231] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.837151] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.841102] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.842000] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.846291] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.868473] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.891508] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.914652] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.937152] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.939465] Linux agpgart interface v0.103
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.943278] loop: module loaded
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.943986] libphy: Fixed MDIO Bus: probed
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.944834] tun: Universal TUN/TAP device driver, 1.6
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.945819] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.979865] PPP generic driver version 2.4.2
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.980528] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.981348] ehci-pci: EHCI PCI platform driver
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.982392] ehci-platform: EHCI generic platform driver
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.983309] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.984268] ohci-pci: OHCI PCI platform driver
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.985173] ohci-platform: OHCI generic platform driver
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.985912] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.987268] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.989163] i8042: Warning: Keylock active
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.990743] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.991703] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.992844] mousedev: PS/2 mouse device common for all mice
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.994005] rtc_cmos 00:00: RTC can wake from S4
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.995604] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.996929] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.997847] i2c /dev entries driver
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.998466] device-mapper: uevent: version 1.0.3
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    2.999421] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 14:31:26 travis-job-394evis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.015312] zswap: loaded using pool lzo/zbud
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.017738] Key type trusted registered
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.021864] Key type encrypted registered
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.022888] ima: No TPM chip found, activating TPM-bypass!
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.023834] evm: HMAC attrs: 0x1
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.025064]   Magic number: 14:499:535
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.025695] tty ttyS15: hash matches
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.026547] rtc_cmos 00:00: setting system clock to 2018-08-07 14:31:18 UTC (1533652278)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.028052] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.028780] EDD information not available.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.029593] PM: Hibernation image not present or could not be loaded.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.030882] Freeing unused kernel memory: 1496K
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.031666] Write protecting the kernel read-only data: 14336k
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.033329] Freeing unused kernel memory: 1956K
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.034243] Freeing unused kernel memory: 92K
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.046650] systemd-udevd[119]: starting version 204
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.107182] scsi host0: Virtio SCSI HBA
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.112662] AVX2 version of gcm_enc/dec engaged.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.113534] AES CTR mode by8 optimization enabled
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.117201] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.152869] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.153285] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.155075] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.156016] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    3.156722] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [  e-886a-1f01ad138a43 kernel: [    7.532895] raid6: sse2x4   gen() 12758 MB/s
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    7.600895] raid6: sse2x4   xor()  8907 MB/s
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    7.668893] raid6: avx2x1   gen() 16974 MB/s
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    7.736890] raid6: avx2x2   gen() 19680 MB/s
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    7.804893] raid6: avx2x4   gen() 22024 MB/s
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    7.805684] raid6: using algorithm avx2x4 gen() 22024 MB/s
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    7.806512] raid6: using avx2x2 recovery algorithm
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    7.808379] xor: automatically using best checksumming function:
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    7.848899]    avx       : 27283.000 MB/sec
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    7.863028] Btrfs loaded
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    7.908771] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    7.909896] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    7.985366] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    7.990345] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    7.991267] EXT4-fs (sda1): recovery complete
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    7.995810] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    8.199253] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    8.309759] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    8.355510] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    8.536530] random: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    9.092698] random: cloud-init: uninitialized urandom read (32 bytes read, 46 bits of entropy available)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    9.223752] systemd-udevd[702]: starting version 204
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    9.358367] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    9.388545] intel_rapl: no valid rapl domains found in package 0
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    9.447530] ppdev: user-space parallel port driver
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    9.527850] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    9.577019] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    9.641141] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [    9.807207] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [   10.068485] random: mktemp: uninitialized urandom read (12 bytes read, 61 bits of entropy available)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [   10.139534] random: mktemp: uninitialized urandom read (6 bytes read, 62 bits of entropy available)
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [   10.209853] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [   10.247701] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [   10.682416] init: failsafe main process (1093) killed by TERM signal
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO Running set_multiqueue.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO Set channels for eth0 to 4.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 14:31:26 travis-job-394ec26e-0e2b-427ount asari.
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 google-accounts: INFO Removing user packer.
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 cron[1400]: (CRON) INFO (pidfile fd = 3)
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 cron[1468]: (CRON) STARTUP (fork ok)
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 cron[1468]: (CRON) INFO (Running @reboot jobs)
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 acpid: starting up with netlink and the input layer
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 acpid: 1 rule loaded
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 acpid: waiting for events: event logging is off
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 haveged: haveged starting up
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [   11.841103] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [   11.850049] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [   12.050050] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [   12.052466] Bridge firewalling registered
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [   12.062090] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [   12.119689] Initializing XFRM netlink socket
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [   12.126592] Netfilter messages via NETLINK v0.30.
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [   12.129426] ctnetlink v0.93: registering with nfnetlink.
Aug  7 14:31:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [   12.397018] floppy0: no floppy controllers found
Aug  7 14:31:50 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 ntpdate[1769]: adjust time server 169.254.169.254 offset 0.001598 sec
Aug  7 14:31:57 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 ntpd[1804]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 14:31:57 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 ntpd[1805]: proto: precision = 0.124 usec
Aug  7 14:31:57 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 ntpd[1805]: ntp_io: estimated max descriptors: 1024, initial socket boundaavis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 startup-script: INFO Finished running startup scripts.
Aug  7 14:31:57 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 ntpd[1805]: ntp_io: estimated max descriptors: 1024, initial socket boundaavis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 startup-script: INFO Finished running startup scripts.
Aug  7 14:31:57 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 ec2: 
Aug  7 14:31:57 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 ec2: #############################################################
Aug  7 14:31:57 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 14:31:57 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 ec2: 1024 d9:30:1e:e7:81:b1:ce:93:d9:de:df:21:cb:23:ef:72  root@travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 (DSA)
Aug  7 14:31:57 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 ec2: 256 7a:1a:02:fd:96:e1:1a:a0:90:70:9b:56:36:f7:6b:a8  root@travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 (ECDSA)
Aug  7 14:31:57 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 ec2: 256 c6:b3:1c:52:5b:ba:7a:97:c4:b0:4c:ab:14:b6:e6:e9  root@travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 (ED25519)
Aug  7 14:31:57 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 ec2: 2048 0a:c1:17:01:01:05:2b:06:91:e4:c5:ed:53:6b:94:ca  root@travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 (RSA)
Aug  7 14:31:57 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 14:31:57 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 ec2: #############################################################
Aug  7 14:32:27 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [   72.018356] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 14:33:28 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [  132.632298] device veth046394d entered promiscuous mode
Aug  7 14:33:28 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [  132.632365] docker0: port 1(veth046394d) entered forwarding state
Aug  7 14:33:28 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [  132.632373] docker0: port 1(veth046394d) entered forwarding state
Aug  7 14:33:28 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [  132.633017] docker0: port 1(veth046394d) entered disabled state
Aug  7 14:33:28 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [  132.716108] cgroup: docker-runc (4785) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 14:33:28 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [  132.716111] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 14:33:28 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [  132.787751] eth0: renamed from veth12c9f51
Aug  7 14:33:28 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [  132.825989] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 14:33:28 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [  132.826909] docker0: port 1(veth046394d) entered forwarding state
Aug  7 14:33:28 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [  132.826924] docker0: port 1(veth046394d) entered forwarding state
Aug  7 14:33:28 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 kernel: [  132.826944] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 14:33:31 travis-job-394ec26e-0e2b-427e-886a-1f01ad138a43 ntpd[1805]: Listen normally on 5 docker0 fe80::1 UDP 123da1        30G  9.6G   19G  34% /
none            5.0M     0  5.0M   0% /run/lock
none            7.4G     0  7.4G   0% /run/shm
none            100M     0  100M   0% /run/user
none            768M     0  768M   0% /var/ramfs
