\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs","byte_start":979,"byte_end":997,"line_start":30,"line_end":30,"column_start":9,"column_end":27,"is_primary":true,"text":[{"text":"    use edition_lint_paths::foo;","highlight_start":9,"highlight_end":27}],"label":"Did you mean `self::edition_lint_paths`?","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0432]: unresolved import `edition_lint_paths`\n  --> /checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs:30:9\n   |\nLL |     use edition_lint_paths::foo;\n   |         ^^^^^^^^^^^^^^^^^^ Did you mean `self::edition_lint_paths`?\n\n"}
[00:43:49] {"message":"unused extern crate","code":{"code":"unused_extern_crates","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs","byte_start":719,"byte_end":751,"line_start":22,"line_end":22,"column_start":1,"column_end":33,"is_primary":true,"text":[{"text":"extern crate edition_lint_paths;","highlight_start":1,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs","byte_start":678,"byte_end":694,"line_start":19,"line_end":19,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"#![deny(rust_2018_idioms)]","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[deny(unused_extern_crates)] implied by #[deny(rust_2018_idioms)]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove it","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs","byte_start":719,"byte_end":751,"line_start":22,"line_end":22,"column_start":1,"column_end":33,"is_primary":true,"text":[{"text":"extern crate edition_lint_paths;","highlight_start":1,"highlight_end":33}],"label":null,"suggested_replacement":"","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused extern crate\n  --> /checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs:22:1\n   |\nLL | extern crate edition_lint_paths;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove it\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs:19:9\n   |\nLL | #![deny(rust_2018_idioms)]\n   |         ^^^^^^^^^^^^^^^^\n   = note: #[deny(unused_extern_crates)] implied by #[deny(rust_2018_idioms)]\n\n"}
[00:43:49] {"message":"`extern crate` is not idiomatic in the new edition","code":{"code":"unused_extern_crates","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs","byte_start":784,"byte_end":823,"line_start":25,"line_end":25,"column_start":1,"column_end":40,"is_primary":true,"text":[{"text":"extern crate edition_lint_paths as bar;","highlight_start":1,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"convert it to a `use`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs","byte_start":784,"byte_end":823,"line_start":25,"line_end":25,"column_start":1,"column_end":40,"is_primary":true,"text":[{"text":"extern crate edition_lint_paths as bar;","highlight_start":1,"highlight_end":40}],"label":null,"suggested_replacement":"use edition_lint_paths as bar;","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `extern crate` is not idiomatic in the new edition\n  --> /checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs:25:1\n   |\nLL | extern crate edition_lint_paths as bar;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert it to a `use`\n\n"}
[00:43:49] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:43:49] {"message":"For more information about this error, try `rustc --explain E0432`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0432`.\n"}
[00:43:49] ------------------------------------------
[00:43:49] 
[00:43:49] thread '[ui] ui/rust-2018/extern-crate-idiomatic-in-2018.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:43:49] 
---
[00:43:49] 
[00:43:49] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:43:49] 
[00:43:49] 
[00:43:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:43:49] 
[00:43:49] 
[00:43:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:43:49] Build completed unsuccessfully in 0:02:10
[00:43:49] Build completed unsuccessfully in 0:02:10
[00:43:49] Makefile:58: recipe for target 'check' failed
[00:43:49] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0563802c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:1f146d50
$ sudo tail -n 500 /var/log/syslog
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   3 disabled
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   4 disabled
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   5 disabled
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   6 disabled
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   7 disabled
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Using GB pages for direct mapping
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] kvm-clock: using sched offset of 1633601233 cycles
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Zone ranges:
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   Device   empty
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Movable zone start for each node
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Early memory node ranges
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Policy zone: Normal
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Hierarchical RCU implementation.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] console [ttyS0] enabled
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.319764] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.320940] pid_max: default: 32768 minimum: 301
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.321601] ACPI: Core revision 20150930
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.328131] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.329604] Security Framework initialized
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.330343] Yama: becoming mindful.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.331360] AppArmor: AppArmor disabled by boot time parameter
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.334067] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.343906] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.348211] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.349272] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.350519] Initializing cgroup subsys io
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.351452] Initializing cgroup subsys memory
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.352243] Initializing cgroup subsys devices
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.353141] Initializing cgroup subsys freezer
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.354103] Initializing cgroup subsys net_cls
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.354880] Initializing cgroup subsys perf_event
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.355626] Initializing cgroup subsys net_prio
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.356373] Initializing cgroup subsys hugetlb
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.357114] Initializing cgroup subsys pids
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.357883] CPU: Physical Processor ID: 0
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.358464] CPU: Processor Core ID: 0
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.360186] mce: CPU supports 32 MCE banks
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.360999] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.361806] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.364597] Freeing SMP alternatives memory: 32K
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.373009] ftrace: allocating 32185 entries in 126 pages
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.420787] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.421817] smpboot: Max logical packages: 2
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.423132] x2apic enabled
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.424815] Switched APIC routing to physical x2apic.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.428821] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.534947] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.536591] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.539970] x86: Booting SMP configuration:
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.540738] .... node  #0, CPUs:      #1
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.541528] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.546238]  #2
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.546684] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.551942]  #3
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.552522] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.556952] x86: Booted up 1 node, 4 CPUs
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.557630] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.560182] devtmpfs: initialized
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.564596] evm: security.selinux
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.565176] evm: security.SMACK64
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.565697] evm: security.SMACK64EXEC
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.566323] evm: security.SMACK64TRANSMUTE
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.567157] evm: security.SMACK64MMAP
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.567823] evm: security.ima
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.568264] evm: security.capability
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.569173] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.570682] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.571816] pinctrl core: initialized pinctrl subsystem
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.572849] RTC time: 14:03:03, date: 08/09/18
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.574376] NET: Registered protocol family 16
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.586988] cpuidle: using governor ladder
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.599018] cpuidle: using governor menu
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.599660] PCCT header not found.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.600501] ACPI: bus type PCI registered
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.601204] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.602300] PCI: Using configuration type 1 for base access
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.616004] ACPI: Added _OSI(Module Device)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.616664] ACPI: Added _OSI(Processor Device)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.617276] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.617975] ACPI: Added _OSI(Processor Aggregator Device)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.621398] ACPI: Executed 2 blocks of module-level executable AML code
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.645113] ACPI: Interpreter enabled
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.645955] ACPI: (supports S0 S3 S4 S5)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.646711] ACPI: Using IOAPIC for interrupt routing
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.647500] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.677300] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.678469] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.679686] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.685095] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.687754] PCI host bridge to bus 0000:00
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.688356] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.689498] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.690603] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.691941] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.693330] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.694308] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.694754] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.709554] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.723680] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.725346] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.731590] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.737637] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.750975] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.756833] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.761206] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.776953] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.779084] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.781604] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.783866] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.785933] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.806495] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.807668] vgaarb: loaded
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.808278] SCSI subsystem initialized
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.808904] libata version 3.00 loaded.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.808920] ACPI: bus type USB registered
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.809490] usbcore: registered new interface driver usbfs
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.810257] usbcore: registered new interface driver hub
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.811038] usbcore: registered new device driver usb
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.811875] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.813131] dmi: Firmware registration failed.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.814057] PCI: Using ACPI for IRQ routing
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.814738] PCI: pci_cache_line_size set to 64 bytes
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.814841] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.814842] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.814966] NetLabel: Initializing
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.815576] NetLabel:  domain hash size = 128
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.816204] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.817085] NetLabel:  unlabeled traffic allowed by default
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.818218] amd_nb: Cannot enumerate AMD northbridges
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.819018] clocksource: Switched to clocksource kvm-clock
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.826783] pnp: PnP ACPI init
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.827357] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.827423] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.827463] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.827511] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.827549] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.827587] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.827624] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.827782] pnp: PnP ACPI: found 7 devices
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.835143] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.836644] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.836646] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.836648] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.836649] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.836681] NET: Registered protocol family 2
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.837536] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.839675] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.841112] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.842199] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.843129] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.844859] NET: Registered protocol family 1
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.845500] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.846748] PCI: CLS 0 bytes, default 64
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    0.846828] Unpacking initramfs...
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.933085] Freeing initrd memory: 21432K
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.934127] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.935149] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.936906] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.938316] hw unit of domain pp0-core 2^-0 Joules
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.939198] hw unit of domain package 2^-0 Joules
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.940002] hw unit of domain dram 2^-0 Joules
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.940795] Scanning for low memory corruption every 60 seconds
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.942375] audit: initializing netlink subsys (disabled)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.943402] audit: type=2000 audit(1533823385.940:1): initialized
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.944653] Initialise system trusted keyring
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.945590] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.946806] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.948949] zbud: loaded
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.949648] VFS: Disk quotas dquot_6.6.0
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.950246] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.951929] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.953137] fuse init (API version 7.23)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.953841] Key type big_key registered
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.954413] Allocating IMA MOK and blacklist keyrings.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.956457] Key type asymmetric registered
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.957066] Asymmetric key parser 'x509' registered
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.958056] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.959180] io scheduler noop registered
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.959875] io scheduler deadline registered (default)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.960747] io scheduler cfq registered
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.961380] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.962177] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.963342] intel_idle: does not run on family 6 model 62
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.963443] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.964506] ACPI: Power Button [PWRF]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.965188] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.966330] ACPI: Sleep Button [SLPF]
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.967359] GHES: HEST is not enabled!
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.969841] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.970864] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.975619] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.976562] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    2.981066] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.003890] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.027345] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.050180] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.073444] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.077184] Linux agpgart interface v0.103
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.080003] loop: module loaded
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.080904] libphy: Fixed MDIO Bus: probed
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.081780] tun: Universal TUN/TAP device driver, 1.6
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.082902] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.121842] PPP generic driver version 2.4.2
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.122961] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.124117] ehci-pci: EHCI PCI platform driver
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.124972] ehci-platform: EHCI generic platform driver
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.126176] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.127425] ohci-pci: OHCI PCI platform driver
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.128527] ohci-platform: OHCI generic platform driver
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.129511] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.130946] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.133304] i8042: Warning: Keylock active
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.135354] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.136478] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.137719] mousedev: PS/2 mouse device common for all mice
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.139079] rtc_cmos 00:00: RTC can wake from S4
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.140495] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.142057] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.143554] i2c /dev entries driver
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.144231] device-mapper: uevent: version 1.0.3
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.145261] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.147057] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.148868] NET: Registered protocol family 10
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.149928] NET: Registered protocol family 17
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.150765] Key type dns_resolver registered
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.151857] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.152909] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.153914] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.155244] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.156548] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.158347] registered taskstats version 1
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.159264] Loading compiled-in X.509 certificates
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.160691] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.162815] zswap: loaded using pool lzo/zbud
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.165488] Key type trusted registered
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.170001] Key type encrypted registered
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.170933] ima: No TPM chip found, activating TPM-bypass!
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.172218] evm: HMAC attrs: 0x1
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.173283]   Magic number: 14:684:80
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.174433] rtc_cmos 00:00: setting system clock to 2018-08-09 14:03:06 UTC (1533823386)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.176253] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.177609] EDD information not available.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.178620] PM: Hibernation image not present or could not be loaded.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.180001] Freeing unused kernel memory: 1496K
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.180752] Write protecting the kernel read-only data: 14336k
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.182336] Freeing unused kernel memory: 1956K
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.183222] Freeing unused kernel memory: 92K
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.196478] systemd-udevd[118]: starting version 204
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.249934] scsi host0: Virtio SCSI HBA
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.253428] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.261430] AVX version of gcm_enc/dec engaged.
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.262677] AES CTR mode by8 optimization enabled
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.290079] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.290089] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.292360] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.293257] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.294101] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.294211] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.296775]  sda: sda1
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.297748] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.335661] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.939206] tsc: Refined TSC clocksource calibration: 2499.784 MHz
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    3.940498] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24086dc9c08, max_idle_ns: 440795236697 ns
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    4.172940] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    6.247172] floppy0: no floppy controllers found
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.419070] raid6: sse2x1   gen()  8779 MB/s
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.487092] raid6: sse2x1   xor()  6875 MB/s
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.555070] raid6: sse2x2   gen() 11233 MB/s
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.623062] raid6: sse2x2   xor()  7999 MB/s
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.691045] raid6: sse2x4   gen() 12603 MB/s
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.759088] raid6: sse2x4   xor()  8909 MB/s
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.760000] raid6: using algorithm sse2x4 gen() 12603 MB/s
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.761035] raid6: .... xor() 8909 MB/s, rmw enabled
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.761971] raid6: using ssse3x2 recovery algorithm
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.764100] xor: automatically using best checksumming function:
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.803063]    avx       : 21998.000 MB/sec
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.817482] Btrfs loaded
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.854378] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.855523] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.920863] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.933260] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.934069] EXT4-fs (sda1): recovery complete
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    7.938495] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    8.134078] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    8.245654] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    8.289164] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    8.475610] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    9.018791] random: cloud-init: uninitialized urandom read (32 bytes read, 46 bits of entropy available)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    9.146519] systemd-udevd[701]: starting version 204
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    9.245655] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    9.340719] ppdev: user-space parallel port driver
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    9.429343] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    9.485952] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    9.547652] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    9.706223] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    9.925480] random: mktemp: uninitialized urandom read (12 bytes read, 61 bits of entropy available)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [    9.993407] random: mktemp: uninitialized urandom read (6 bytes read, 62 bits of entropy available)
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [   10.058817] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [   10.099275] EXT4-fs (sda1): resized filesystem to 7864064
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [   10.499245] init: failsafe main process (1093) killed by TERM signal
Aug  9 14:03:13 travis-job-468cd807-8025-40ce-a4fa-1733b782383f rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO Running set_multiqueue.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO Set channels for eth0 to 4.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f google-accounts: INFO Starting Google Accounts daemon.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f google-accounts: INFO Creating a new user account for me.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [   11.218024] random: nonblocking pool is initialized
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f google-accounts: INFO Created user account me.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f google-accounts: INFO Creating a new user account for bogdana.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f google-accounts: INFO Created user account bogdana.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f google-accounts: INFO Creating a new user account for aj.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f google-accounts: INFO Created user account aj.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f google-accounts: INFO Creating a new user account for asari.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f google-accounts: INFO Created user account asari.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f google-accounts: INFO Removing user packer.
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f cron[1434]: (CRON) INFO (pidfile fd = 3)
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f cron[1469]: (CRON) STARTUP (fork ok)
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f cron[1469]: (CRON) INFO (Running @reboot jobs)
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f acpid: starting up with netlink and the input layer
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f acpid: 1 rule loaded
Aug  9 14:03:14 travis-job-468cd807-8025-40ce-a4fa-1733b782383f acpid: waiting for events: event logging is off
Aug  9 14:03:15 travis-job-468cd807-8025-40ce-a4fa-1733b782383f pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 14:03:15 travis-job-468cd807-8025-40ce-a4fa-1733b782383f pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 14:03:15 travis-job-468cd807-8025-40ce-a4fa-1733b782383f haveged: haveged starting up
Aug  9 14:03:15 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [   11.751971] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 14:03:15 travis-job-468cd807-8025-40ce-a4fa-1733b782383f google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 14:03:15 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [   11.763010] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 14:03:15 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [   11.882755] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  9 14:03:15 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [   11.889426] Bridge firewalling registered
Aug  9 14:03:15 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [   11.901057] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 14:03:15 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [   11.967332] Initializing XFRM netlink socket
Aug  9 14:03:15 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [   11.974323] Netfilter messages via NETLINK v0.30.
Aug  9 14:03:15 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [   11.977084] ctnetlink v0.93: registering with nfnetlink.
Aug  9 14:03:15 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [   12.311168] floppy0: no floppy controllers found
Aug  9 14:03:38 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpdate[1773]: adjust time server 169.254.169.254 offset 0.012157 sec
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpd[1808]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpd[1809]: proto: precision = 0.110 usec
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpd[1809]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpd[1809]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpd[1809]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpd[1809]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpd[1809]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpd[1809]: Listen normally on 3 eth0 10.20.0.13 UDP 123
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpd[1809]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpd[1809]: peers refreshed
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpd[1809]: Listening on routing socket on fd #21 for interface updates
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [   41.937401] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f startup-script: INFO Found startup-script in metadata.
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f startup-script: INFO startup-script: job 1 at Thu Aug  9 17:13:00 2018
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f startup-script: INFO startup-script: Return code 0.
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f startup-script: INFO startup-script: Return code 0.
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f startup-script: INFO Finished running startup scripts.
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ec2: 
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ec2: #############################################################
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ec2: 1024 8c:62:b1:4b:9a:72:a9:87:ea:c9:f8:bc:8f:5f:27:f9  root@travis-job-468cd807-8025-40ce-a4fa-1733b782383f (DSA)
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ec2: 256 93:79:96:f1:ff:87:35:bf:1d:e0:1f:ef:37:7c:9c:2f  root@travis-job-468cd807-8025-40ce-a4fa-1733b782383f (ECDSA)
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ec2: 256 5c:26:34:99:7b:2f:01:f5:80:17:99:b7:ad:97:1c:62  root@travis-job-468cd807-8025-40ce-a4fa-1733b782383f (ED25519)
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ec2: 2048 48:d5:c6:d6:8e:c7:a6:2a:05:02:fa:01:31:4c:ee:e1  root@travis-job-468cd807-8025-40ce-a4fa-1733b782383f (RSA)
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 14:03:45 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ec2: #############################################################
Aug  9 14:05:04 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [  121.464734] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 14:06:06 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [  182.914015] device vethe0babdf entered promiscuous mode
Aug  9 14:06:06 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [  182.914080] docker0: port 1(vethe0babdf) entered forwarding state
Aug  9 14:06:06 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [  182.914088] docker0: port 1(vethe0babdf) entered forwarding state
Aug  9 14:06:06 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [  182.914517] docker0: port 1(vethe0babdf) entered disabled state
Aug  9 14:06:06 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [  182.997084] cgroup: docker-runc (4798) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 14:06:06 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [  182.997087] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  9 14:06:06 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [  183.060671] eth0: renamed from veth1ab4ef1
Aug  9 14:06:06 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [  183.095251] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  9 14:06:06 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [  183.096321] docker0: port 1(vethe0babdf) entered forwarding state
Aug  9 14:06:06 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [  183.096334] docker0: port 1(vethe0babdf) entered forwarding state
Aug  9 14:06:06 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [  183.096352] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  9 14:06:10 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpd[1809]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  9 14:06:10 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpd[1809]: Listen normally on 6 docker0 fe80::42:6dff:fe23:cc3 UDP 123
Aug  9 14:06:10 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpd[1809]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  9 14:06:10 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpd[1809]: peers refreshed
Aug  9 14:06:10 travis-job-468cd807-8025-40ce-a4fa-1733b782383f ntpd[1809]: new interface(s) found: waking up resolver
Aug  9 14:06:21 travis-job-468cd807-8025-40ce-a4fa-1733b782383f kernel: [  198.104002] docker0: port 1(vethe0babdf) entered forwarding state
Aug  9 14:17:01 travis-job-468cd807-8025-40ce-a4fa-1733b782383f CRON[12457]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
---
travis_time:end:0f60ef94:start=1533826136020623269,finish=1533826136028750712,duration=8127443
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1943ad2d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:183bc37a
travis_time:start:183bc37a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:07e4358b
$ dmesg | grep -i kill
